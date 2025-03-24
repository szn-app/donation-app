#!/bin/bash
set -e

create_oauth2_client_for_trusted_app@hydra() {
    environment=${1:-development}

    set -a 
        if [ -f ./config/.env.$environment ]; then
            source ./config/.env.$environment
        elif [ -f ./config/.env.$environment.local ]; then
            source ./config/.env.$environment.local
        else
            echo "Error: .env.$environment file not found."
            exit 1
        fi
    set +a

    echo "Creating oauth2 clients for trusted apps in Ory Hydra" 
    if [ "$environment" = "development" ]; then
        echo "APP_URL: $APP_URL" # debug print
    fi

    example_hydra_admin() {
        kubectl run -it --rm --image=debian:latest debug-pod --namespace auth -- /bin/bash
        {
            # install hydra
            apt update >/dev/null 2>&1 && apt install curl jq -y >/dev/null 2>&1
            bash <(curl https://raw.githubusercontent.com/ory/meta/master/install.sh) -d -b . hydra v2.2.0 >/dev/null 2>&1 && mv hydra /usr/bin/

            curl http://hydra-admin/admin/clients | jq

            delete_all_clients() { 
                client_list=$(curl -X GET 'http://hydra-admin/admin/clients' | jq -r '.[].client_id')
                for client in $client_list
                do
                    echo "Deleting client: $client"
                    curl -X DELETE "http://hydra-admin/admin/clients/$client"
                done
            }
        }

        hydra list oauth2-clients --endpoint "http://hydra-admin"
    }

    # port-forward hydra-admin 
    # kpf -n auth services/hydra-admin 4445:4445

    kubectl run --image=debian:latest setup-pod --namespace auth -- /bin/sh -c "while true; do sleep 60; done"
    kubectl wait --for=condition=ready pod/setup-pod --namespace=auth --timeout=300s

    {
                    t="$(mktemp).sh" && cat << 'EOF' > $t
#!/bin/bash
apt update >/dev/null 2>&1 && apt install curl jq -y >/dev/null 2>&1
bash <(curl https://raw.githubusercontent.com/ory/meta/master/install.sh) -d -b . hydra v2.2.0 >/dev/null 2>&1 && mv hydra /usr/bin/
curl -s http://hydra-admin/admin/clients | jq
EOF
        kubectl cp $t setup-pod:$t --namespace auth
        kubectl exec setup-pod --namespace auth -- /bin/sh -c "chmod +x $t && $t" >/dev/null 2>&1
    }
    
    example_using_hydra() {
        t="$(mktemp).sh" && cat << 'EOF' > $t
#!/bin/bash
hydra create oauth2-client --name frontend-client-2 --audience backend-service --endpoint http://hydra-admin --grant-type authorization_code,refresh_token --response-type code --redirect-uri ${APP_URL} --scope offline_access,openid --skip-consent --skip-logout-consent --token-endpoint-auth-method client_secret_post
EOF
        kubectl cp $t setup-pod:$t --namespace auth
        kubectl exec setup-pod --namespace auth -- /bin/bash -c "chmod +x $t && $t"
    }

    example_alternative_option() {
        # TODO: incomplete example
        # https://www.ory.sh/docs/hydra/self-hosted/quickstart
        hydra create client \
            --endpoint http://hydra-admin \
            --grant-type authorization_code,refresh_token \
            --response-type code,id_token \
            --format json \
            --scope openid --scope offline \
            --redirect-uri ${APP_URL} --token-endpoint-auth-method none

        code_client_id=$(echo $code_client | jq -r '.client_id')
        code_client_secret=$(echo $code_client | jq -r '.client_secret')
    }

    {
        # app client users for trusted app

        # redirect uri is where the resource owner (user) will be redirected to once the authorization server grants permission to the client
        # NOTE: using the `authorization code` the client gets both `accesst token` and `id token` when `scope` includes `openid`.
        client_name="frontend-client"
        client_secret="$(LC_ALL=C tr -dc 'A-Za-z0-9' < /dev/urandom | head -c 32 | base64 -w 0)"
        client_exist=$(kubectl exec setup-pod --namespace auth -- curl -s 'http://hydra-admin/admin/clients' | jq -r ".[] | select(.client_id==\"$client_name\") | .client_id")

        if [[ -z "$client_exist" ]]; then
            echo 'Adding oauth2 client'

            t="$(mktemp).sh" && cat << EOF > $t
#!/bin/bash

# Hybrid Flow involves code+id_toekn
curl -X POST 'http://hydra-admin/admin/clients' -H 'Content-Type: application/json' \
--data '{
    "client_id": "frontend-client",
    "client_name": "frontend-client",
    "client_secret": "${client_secret}",
    "grant_types": ["authorization_code", "refresh_token"],
    "response_types": ["code", "code id_token"],
    "redirect_uris": ["${APP_URL}", "${APP_URL}/callback"], 
    "audience": ["${APP_URL}"],    
    "scope": "offline_access openid email profile",
    "token_endpoint_auth_method": "client_secret_post",
    "skip_consent": true,
    "skip_logout_consent": true,
    "post_logout_redirect_uris": ["${APP_URL}", "${APP_URL}/callback"]
}'

EOF
            kubectl cp $t setup-pod:$t --namespace auth
            kubectl exec setup-pod --namespace auth -- /bin/sh -c "chmod +x $t && $t"
            # create/update secret 
            kubectl create secret generic ory-hydra-client--frontend-client -n auth --from-literal=client_secret="$client_secret" --dry-run=client -o yaml | kubectl apply -f - >/dev/null 2>&1
        else
            echo "Oauth2 client 'frontend-client' already exist"
        fi

    }
    {

        client_name="frontend-client-oauth"
        client_secret="$(LC_ALL=C tr -dc 'A-Za-z0-9' < /dev/urandom | head -c 32 | base64 -w 0)"
        client_exist=$(kubectl exec setup-pod --namespace auth -- curl -s 'http://hydra-admin/admin/clients' | jq -r ".[] | select(.client_id==\"$client_name\") | .client_id")

        if [[ -z "$client_exist" ]]; then
            echo 'Adding oauth2 client'
        
            t="$(mktemp).sh" && cat << EOF > $t
#!/bin/bash

curl -X POST 'http://hydra-admin/admin/clients' -H 'Content-Type: application/json' \
--data '{
    "client_id": "frontend-client-oauth",
    "client_name": "frontend-client-oauth",
    "client_secret": "${client_secret}",
    "grant_types": ["authorization_code", "refresh_token"],
    "response_types": ["code"],
    "redirect_uris": ["${APP_URL}", "${APP_URL}/callback"], 
    "audience": ["${APP_URL}"],    
    "scope": "offline_access openid",
    "token_endpoint_auth_method": "client_secret_post",
    "skip_consent": true,
    "skip_logout_consent": true,
    "post_logout_redirect_uris": ["${APP_URL}", "${APP_URL}/callback"]
}'
EOF
            kubectl cp $t setup-pod:$t --namespace auth
            kubectl exec setup-pod --namespace auth -- /bin/sh -c "chmod +x $t && $t"
            # create/update secret 
            kubectl create secret generic ory-hydra-client--frontend-client-oauth -n auth --from-literal=client_secret="$client_secret" --dry-run=client -o yaml | kubectl apply -f - >/dev/null 2>&1
        else
            echo "Oauth2 client 'frontend-client-oauth' already exist"
        fi
    }

    {
        client_name="frontend-client-oauth-consent"
        client_secret="$(LC_ALL=C tr -dc 'A-Za-z0-9' < /dev/urandom | head -c 32 | base64 -w 0)"
        client_exist=$(kubectl exec setup-pod --namespace auth -- curl -s 'http://hydra-admin/admin/clients' | jq -r ".[] | select(.client_id==\"$client_name\") | .client_id")
        
        if [[ -z "$client_exist" ]]; then
            echo 'Adding oauth2 client'
            
            t="$(mktemp).sh" && cat << EOF > $t
#!/bin/bash

curl -X POST 'http://hydra-admin/admin/clients' -H 'Content-Type: application/json' \
--data '{
    "client_id": "frontend-client-oauth-consent",
    "client_name": "frontend-client-oauth-consent",
    "client_secret": "${client_secret}",
    "grant_types": ["authorization_code", "refresh_token"],
    "response_types": ["code id_token"],
    "redirect_uris": ["${APP_URL}", "${APP_URL}/callback"], 
    "audience": ["${APP_URL}"],    
    "scope": "offline_access openid",
    "token_endpoint_auth_method": "client_secret_post",
    "skip_consent": false,
    "skip_logout_consent": true,
    "post_logout_redirect_uris": ["${APP_URL}", "${APP_URL}/callback"]
}'
EOF
            kubectl cp $t setup-pod:$t --namespace auth
            kubectl exec setup-pod --namespace auth -- /bin/sh -c "chmod +x $t && $t"
            # create/update secret 
            kubectl create secret generic ory-hydra-client--frontend-client-oauth-consent -n auth --from-literal=client_secret="$client_secret" --dry-run=client -o yaml | kubectl apply -f - >/dev/null 2>&1
        else
            echo "Oauth2 client 'frontend-client-oauth-consent' already exist"
        fi
    }

    {
        # internal service communication
        client_name="internal-communication"
        client_secret="$(LC_ALL=C tr -dc 'A-Za-z0-9' < /dev/urandom | head -c 32 | base64 -w 0)"
        client_exist=$(kubectl exec setup-pod --namespace auth -- curl -s 'http://hydra-admin/admin/clients' | jq -r ".[] | select(.client_id==\"$client_name\") | .client_id")

        if [[ -z "$client_exist" ]]; then
            echo 'Adding oauth2 client'
    
            t="$(mktemp).sh" && cat << EOF > $t
#!/bin/bash

curl -X POST 'http://hydra-admin/admin/clients' -H 'Content-Type: application/json' \
--data '{
    "client_id": "internal-communication",
    "client_name": "internal-communication",
    "client_secret": "${client_secret}",
    "grant_types": ["client_credentials"],
    "response_types": [],
    "audience": ["internal-service", "external-service"],
    "scope": "offline_access openid email profile",
    "token_endpoint_auth_method": "client_secret_basic"
}'
EOF
            kubectl cp $t setup-pod:$t --namespace auth
            kubectl exec setup-pod --namespace auth -- /bin/sh -c "chmod +x $t && $t"
            # create/update secret 
            kubectl create secret generic ory-hydra-client--internal-communication -n auth --from-literal=client_secret="$client_secret" --dry-run=client -o yaml | kubectl apply -f - >/dev/null 2>&1
        else
            echo "Oauth2 client 'internal-communication' already exist"
        fi

    }



    {
        # Oathkeeper introspection
        client_name="oathkeeper-introspection"
        client_secret="$(LC_ALL=C tr -dc 'A-Za-z0-9' < /dev/urandom | head -c 32 | base64 -w 0)"
        client_exist=$(kubectl exec setup-pod --namespace auth -- curl -s 'http://hydra-admin/admin/clients' | jq -r ".[] | select(.client_id==\"$client_name\") | .client_id")
        
        if [[ -z "$client_exist" ]]; then
            echo 'Adding oauth2 client'

            t="$(mktemp).sh" && cat << EOF > $t
#!/bin/bash

curl -X POST 'http://hydra-admin/admin/clients' -H 'Content-Type: application/json' \
--data '{
    "client_id": "oathkeeper-introspection",
    "client_name": "oathkeeper-introspection",
    "client_secret": "${client_secret}",
    "grant_types": ["client_credentials"],
    "response_types": ["token"],
    "audience": ["internal-service"],
    "scope": "introspect",
    "token_endpoint_auth_method": "client_secret_basic"
}'                     
EOF
            kubectl cp $t setup-pod:$t --namespace auth
            kubectl exec setup-pod --namespace auth -- /bin/sh -c "chmod +x $t && $t"
            # create/update secret 
            kubectl create secret generic ory-hydra-client--oathkeeper-introspection -n auth --from-literal=client_secret="$client_secret" --dry-run=client -o yaml | kubectl apply -f - >/dev/null 2>&1
        else
            echo "Oauth2 client 'oathkeeper-introspection' already exist"
        fi

    }
    
    kubectl delete --force pod setup-pod -n auth > /dev/null 2>&1


    # NOTE: this is not a proper OIDC exposure to other services (only an example) 
    # for third party apps to access data 
    # curl -X POST 'http://hydra-admin/admin/clients' \
    # -H 'Content-Type: application/json' \
    # --data-raw '{
    #     "client_id": "third-party",
    #     "client_name": "third-party",
    #     "grant_types": ["authorization_code", "refresh_token"],
    #     "response_types": ["code"],
    #     "redirect_uris": ["http://localhost:8000/oauth-redirect"],
    #     "audience": ["external-service"],
    #     "scope": "offline_access openid custom_scope:read",
    #     "token_endpoint_auth_method": "client_secret_post",
    #     "skip_consent": false,
    #     "post_logout_redirect_uris": [],
    #     "skip_logout_consent": false
    # }'

    manual_verify() { 
        kubectl run -it --rm --image=nicolaka/netshoot debug-pod --namespace auth -- curl http://hydra-admin/admin/clients | jq
        kubectl run -it --rm --image=nicolaka/netshoot debug-pod --namespace auth -- curl http://hydra-admin/admin/clients/frontend-client | jq
        
    }

}

# create .env files from default template if doesn't exist
create_env_files@hydra() {(
    # Find all *.env.template files
    find ./config -name "*.env.template" | while IFS= read -r template_file; do
            # Extract filename without extension
            filename=$(basename "$template_file" | cut -d '.' -f 1)
            env_file="$(dirname "$template_file")/$filename.env"

            # Check if .env file already exists
            if [ ! -f "$env_file" ]; then
                # Create a new .env file from the template in the same directory
                cp "$template_file" "$env_file" 
                echo "created env file file://$(readlink -f $env_file) from $template_file"
            else
                echo "env file already exists: file://$(readlink -f $env_file)"
            fi
    done
)}

delete@hydra() {
    helm uninstall hydra -n auth
}

install@hydra() {
    set -e
    local environment=$1

    # ory stack charts
    helm repo add ory https://k8s.ory.sh/helm/charts > /dev/null 2>&1
    helm repo update > /dev/null 2>&1 

    create_env_files@hydra

    printf "install Ory Hydra \n"
    {
        {
            # Read the secret
            secret_data=$(kubectl get secret hydra--postgresql-credentials-user -n auth -o json)

            # Extract the username and password from the secret data
            username=$(echo "$secret_data" | jq -r '.data.username' | base64 --decode)
            password=$(echo "$secret_data" | jq -r '.data.password' | base64 --decode)

            # Set the environment variables
            export DB_USER=$username
            export DB_PASSWORD=$password
            export DB_NAME='hydra-db'
            export DB_HOST='hydra--cluster-rw'
        }

        # preprocess file through substituting env values
        t="$(mktemp).yaml" && ./script/render-template-config.script.rs --environment $environment < ./hydra-config.yaml.tera > $t && printf "generated manifest with replaced env variables: file://$t\n" 
        q="$(mktemp).yaml" && ./script/render-template-helm.script.rs --environment $environment < ./helm-values.yaml.tera > $q && printf "generated manifest with replaced env variables: file://$q\n" 
        
        system_secret="$(LC_ALL=C tr -dc 'A-Za-z0-9' < /dev/urandom | head -c 32 | base64 -w 0)" 
        cookie_secret="$(LC_ALL=C tr -dc 'A-Za-z0-9' < /dev/urandom | head -c 32)" 
        
        l="$(mktemp).log" && helm upgrade --debug --install hydra ory/hydra -n auth --create-namespace -f $q -f $t \
            --set kratos.config.secrets.system[0]="$system_secret" \
            --set kratos.config.secrets.cookie[0]="$cookie_secret" \
            --set env[0].name=DB_USER --set env[0].value=${DB_USER} \
            --set env[1].name=DB_PASSWORD --set env[1].value=${DB_PASSWORD} > $l 2>&1 && printf "Hydra logs: file://$l\n"
    }

    create_oauth2_client_for_trusted_app@hydra $environment

    # Wait for Hydra deployment to be ready
    printf "Waiting for Hydra deployment to be ready...\n"
    kubectl wait --for=condition=available deployment/hydra --namespace=auth --timeout=300s

    set +e
}

verify#example@hydra() { 
    print_info() {
        curl -k -s https://auth.donation-app.local/authorize/.well-known/openid-configuration | jq
        curl -k -s https://auth.donation-app.local/authorize/.well-known/jwks.json | jq
    }

    # /.well-known/jwks.json
    # /.well-known/openid-configuration
    # /oauth2/auth
    # /oauth2/token
    # /oauth2/revoke
    # /oauth2/fallbacks/consent
    # /oauth2/fallbacks/error
    # /oauth2/sessions/logout
    # /userinfo

    kubectl run -it --rm --image=debian:latest debug-pod-client --namespace auth -- /bin/bash
    {
        # install dependencies including Hydra
        {
            apt update >/dev/null 2>&1 && apt install curl jq -y >/dev/null 2>&1
            bash <(curl https://raw.githubusercontent.com/ory/meta/master/install.sh) -d -b . hydra v2.2.0 >/dev/null 2>&1 && mv hydra /usr/bin/
        }


        curl -s http://hydra-admin/admin/clients | jq

        # https://www.ory.sh/docs/hydra/self-hosted/quickstart
        # [OAuth 2.0] create client and perform "clients credentials" grant
        {
            client=$(hydra create client --endpoint http://hydra-admin--format json --grant-type client_credentials)
            # parse the JSON response using jq to get the client ID and client secret:
            client_id=$(echo $client | jq -r '.client_id')
            client_secret=$(echo $client | jq -r '.client_secret')

            # perform client credentials grant
            CREDENTIALS_GRANT=$(hydra perform client-credentials --endpoint http://hydra-public/ --client-id "$client_id" --client-secret "$client_secret")
            printf "%s\n" "$CREDENTIALS_GRANT"
            TOKEN=$(printf "%s\n" "$CREDENTIALS_GRANT" | grep "ACCESS TOKEN" | awk '{if($1 == "ACCESS" && $2 == "TOKEN") {print $3}}')

            # token introspection 
            hydra introspect token --format json-pretty --endpoint http://hydra-admin$TOKEN
            { # test envoy gateway + oathkeeper as external authorization  
                # introspects http://oathkeeper-api:80/decisions
                curl -i -k -H "Authorization: Bearer $TOKEN" ${TEST_SUBDOMAIN_URL}/oauth-header 
            }
        }

        # [OAuth 2.0] user "Code" grant 
        {
            # example of public client which cannot provide client secrets (authentication flow only using client id )
            code_client=$(hydra create client --endpoint http://hydra-admin --grant-type authorization_code,refresh_token --response-type code,id_token --format json --scope openid --scope offline --redirect-uri http://hydra-public/callback --token-endpoint-auth-method none)
            code_client=$(hydra create client --endpoint http://hydra-admin --grant-type authorization_code,refresh_token --response-type code,id_token --format json --scope openid --scope offline --redirect-uri http://hydra-public/callback)
            code_client_id=$(echo $code_client | jq -r '.client_id')
            code_client_secret=$(echo $code_client | jq -r '.client_secret')
            # perform Authorization Code flow to grant Code 
            hydra perform authorization-code --port 5555 --client-id $code_client_id --endpoint http://hydra-admin--scope openid --scope offline
            # [execute on local mahcine] access hydra's Authorization Code flow endpoint
            # NOTE: requires exposing all relied on services because the examplery authorization page on 5555 redirects to the endpoint hydra-admin which is not exposed in localhost 
            {
                # TODO: APPROACH NOT WORKING - browser doesn't resolve kubernetes services
                kubectl run --image=overclockedllama/docker-chromium debug-auth-browser --namespace auth
                kubectl port-forward pod/debug-auth-browser 5800:5800 --namespace auth
                # access browser at localhost:5800 (on local machine) and navigate to localhost:5555 (which is inside kubernetes)
                kubectl delete pod debug-auth-browser --grace-period=0 --force -n auth
            }
            # [another approach]  requires a reverse proxy or solution to map /etc/hosts domains to specific localhost port, in order to fix redirections
            {
                # TODO: APPROACH NOT WOKRING INCOMPLETE
                kubectl port-forward pod/debug-pod-client 5555:5555 --namespace auth
                kubectl port-forward service/hydra-admin 5556:80 --namespace auth
                kubectl port-forward service/hydra-public 5557:80 --namespace auth
                # echo "127.0.0.1 localhost:5556" | tee -a /etc/hosts
                # sed -i '/127.0.0.1 example1.com/d' /etc/hosts
            }
        }
    }
}

