create_identities@kratos() {
    local environment=$1

    printf "Kratos: create default users \n"
    
    if [ "$environment" = "production" ]; then
        local sleep_time=60
    else 
        local sleep_time=600000
    fi
    kubectl run --image=debian:latest setup-pod-kratos --namespace auth -- /bin/sh -c "while true; do sleep $sleep_time; done"
    kubectl wait --for=condition=ready pod/setup-pod-kratos --namespace=auth --timeout=300s

    {
        t="$(mktemp).sh" && cat << 'EOF' > $t
#!/bin/bash

# install kratos cli
apt update >/dev/null 2>&1 && apt install curl jq -y >/dev/null 2>&1
bash <(curl https://raw.githubusercontent.com/ory/meta/master/install.sh) -d -b . kratos v1.3.1 >/dev/null 2>&1 && chmod +x ./kratos && mv ./kratos /usr/bin/

verify() {
kratos version
}
EOF
        kubectl cp $t setup-pod-kratos:$t --namespace auth
        kubectl exec setup-pod-kratos --namespace auth -- /bin/bash -c "chmod +x $t && $t" >/dev/null 2>&1
    }
    {
        
        if [ "$environment" = "production" ]; then
            set -a
                source config/admin_user_secret.env
            set +a

            username=$ADMIN_USERNAME
            password=$ADMIN_PASSWORD
        else
            username=admin@app.com
            password=admin123456
        fi

        q="$(mktemp).json" && cat << EOF > $q
{
"schema_id": "default",
"traits": {
    "email": "$username"
},
"credentials": {
    "password": {
        "config": {
            "password": "$password"
        }
    }
}
}
EOF

        kubectl cp $q setup-pod-kratos:default_user_kratos.json --namespace auth
        t="$(mktemp).sh" && cat << 'EOF' > $t
#!/bin/bash
# https://www.ory.sh/docs/kratos/cli/kratos

# create default users

kratos import identities ./default_user_kratos.json \
--endpoint http://kratos-admin:80

EOF
        kubectl cp $t setup-pod-kratos:$t --namespace auth
        kubectl exec setup-pod-kratos --namespace auth -- /bin/bash -c "chmod +x $t && $t"

        # NOTE: this is error prone hackish way to get the uuid of the user from debug output of kratos cli
        uuid=$(kubectl exec setup-pod-kratos --namespace auth -- /bin/bash -c "kratos list identities --format json --endpoint http://kratos-admin:80 2> /dev/null | jq -r --arg email \"$username\" '.identities[] | select(.traits.email == \$email) | .id'")

        # create a secret to store the values
        kubectl create secret generic default-admin-user-credentials -n auth --from-literal=username="$username" --from-literal=uuid="$uuid" --from-literal=password="$password" --dry-run=client -o yaml | kubectl apply -f - >/dev/null 2>&1

    }

    # if [ "$environment" = "production" ]; then
    kubectl delete --force pod setup-pod-kratos -n auth > /dev/null 2>&1 || true
    # fi
}

generate_default_username@kratos() {( # use subshell to avoid change variables    
    pushd "$(realpath "$(dirname "$(dirname "${BASH_SOURCE[0]}")")")"

    local environment=production
    if [ -f ./config/.env.$environment ]; then
        source ./config/.env.$environment
    elif [ -f ./config/.env.$environment.local ]; then
        source ./config/.env.$environment.local
    else
        echo "Error: .env.$environment file not found."
        exit 1
    fi

    if [ -z "$DOMAIN_NAME" ]; then
        echo "Error: DOMAIN_NAME environment variable is not set"
        popd
        return 
    fi

    local username="admin-$(openssl rand -hex 8)@$DOMAIN_NAME"
    local password="$(openssl rand -base64 64 | tr -dc 'A-Za-z0-9!@#$%^&*()_+-=[]{}|;:,.<>?~`' | head -c 32)"

    file="./config/admin_user_secret.env"
    if [ ! -f "$file" ]; then
        t=$(mktemp) && cat <<EOF > "$t"
ADMIN_USERNAME="$username"
ADMIN_PASSWORD="$password"
EOF

        mv $t $file
        echo "generated secrets file: file://$(readlink -f $file)"
    else
        echo "Secrets file file://$(readlink -f $file) already exists."
    fi

    popd
)}

generate_env_file@kratos() {(
    pushd "$(realpath "$(dirname "$(dirname "${BASH_SOURCE[0]}")")")"

    env_file_name="./config/jsonnet.env"
    google_jsonnet_file="./config/google-oidc-mapper.jsonnet"

    # Check if the JSONNET file exists
    if [[ ! -f "$google_jsonnet_file" ]]; then
        echo "Error: File '$google_jsonnet_file' not found!"
        popd
        return 1
    fi

    # Read the JSONNET file and encode it as base64
    google_jsonnet_base64=$(base64 -w 0 < "$google_jsonnet_file")

    t=$(mktemp) && cat <<EOF > "$t"
GOOGLE_JSONNET_MAPPER_BASE64="$google_jsonnet_base64"
EOF
    mv $t $env_file_name
    echo "generated env file: file://$(readlink -f $env_file_name)"

    popd
)}

# create .env files from default template if doesn't exist
create_env_files@kratos() {(
    pushd "$(dirname "$(dirname "${BASH_SOURCE[0]}")")"

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

    popd
)}

delete@kratos() {
    helm delete kratos -n auth
}

install@kratos() {
    set -e
    local environment=$1

    # ory stack charts
    helm repo add ory https://k8s.ory.sh/helm/charts > /dev/null 2>&1
    helm repo update > /dev/null 2>&1 

    generate_default_username@kratos
    create_env_files@kratos
    generate_env_file@kratos
    
    printf "install Ory Kratos \n"
    {
        
        {
            # Read the secret
            secret_data=$(kubectl get secret kratos--postgresql-credentials-user -n auth -o json)

            # Extract the username and password from the secret data
            username=$(echo "$secret_data" | jq -r '.data.username' | base64 --decode)
            password=$(echo "$secret_data" | jq -r '.data.password' | base64 --decode)

            # Set the environment variables
            export DB_USER=$username
            export DB_PASSWORD=$password
            export DB_NAME='kratos-db'
            export DB_HOST='kratos--cluster-rw'
        }

        # preprocess file through substituting env values
        t="$(mktemp).yaml" && ./script/render-template-config.script.rs --environment $environment < kratos-config.yaml.tera > $t && printf "generated manifest with replaced env variables: file://$t\n" 
        q="$(mktemp).yaml" && ./script/render-template-helm.script.rs --environment $environment < helm-values.yaml.tera > $q && printf "generated manifest with replaced env variables: file://$q\n" 
        
        default_secret="$(openssl rand -hex 16)"
        cookie_secret="$(LC_ALL=C tr -dc 'A-Za-z0-9' < /dev/urandom | head -c 32)" 
        cipher_secret="$(openssl rand -hex 16)"
        l="$(mktemp).log" && helm upgrade --debug --install --atomic kratos ory/kratos -n auth --create-namespace -f $q -f $t \
            --set-file kratos.identitySchemas.identity-schema\\.json=./config/identity-schema.json \
            --set kratos.config.secrets.default[0]="$default_secret" \
            --set kratos.config.secrets.cookie[0]="$cookie_secret" \
            --set kratos.config.secrets.cipher[0]="$cipher_secret" \
            --set env[0].name=DB_USER --set env[0].value=${DB_USER} \
            --set env[1].name=DB_PASSWORD --set env[1].value=${DB_PASSWORD} > $l 2>&1 && printf "kratos logs: file://$l\n"
    }

    # Wait for Kratos deployment to be ready
    printf "Waiting for Kratos deployment to be ready...\n"
    kubectl wait --for=condition=available deployment/kratos --namespace=auth --timeout=300s

    create_identities@kratos $environment
    
    set +e
}

verify#example@kratos()  {
    verify_jsonnet() {
        kratos help jsonnet lint
        kratos jsonnet lint ./google-oidc-mapper.template.json
    }

    {
        # manually validate rendered templates and deployment manifest files
        y="$(mktemp).yaml" && helm upgrade --dry-run --install kratos ory/kratos -n auth --create-namespace -f $q -f $t \
            --set-file kratos.identitySchemas.identity-schema\\.json=./config/identity-schema.json \
            --set kratos.config.secrets.default[0]="$default_secret" \
            --set kratos.config.secrets.cookie[0]="$cookie_secret" \
            --set kratos.config.secrets.cipher[0]="$cipher_secret" \
            --set env[0].name=DB_USER --set env[0].value=${DB_USER} \
            --set env[1].name=DB_PASSWORD --set env[1].value=${DB_PASSWORD} > $y && printf "generated manifest with replaced env variables: file://$y\n"
    }
    
    # https://www.ory.sh/docs/kratos/self-service
    check_authentication_flow() {
        {
            # https://www.ory.sh/docs/kratos/quickstart#perform-registration-login-and-logout
            # return a new login flow and csrf_token 
            flow=$(curl -k -s -X GET -H "Accept: application/json" "https://auth.donation-app.test/authenticate/self-service/login/api")
            flowId=$(echo $flow | jq -r '.id')
            actionUrl=$(echo $flow | jq -r '.ui.action')
            echo $actionUrl
            # display info about the new login flow and required parameters
            curl -k -s -X GET -H "Accept: application/json" "https://auth.donation-app.test/authenticate/self-service/login/flows?id=$flowId" | jq
            curl -k -s -X POST -H  "Accept: application/json" -H "Content-Type: application/json" -d '{"identifier": "i-do-not-exist@user.org", "password": "the-wrong-password", "method": "password"}' "$actionUrl" | jq
        }
        {
            # makes internal call to https://auth.donation-app.test/authenticate/self-service/login/api to retrieve csrf_token and redirect user
            curl -k -s -i -X GET -H "Accept: text/html" https://auth.donation-app.test/authenticate/self-service/login/browser 
            # login will make POST request with required parameters to /self-service/login/flows?id=$flowId 
            printf "visit https://auth.donation-app.test/login?flow=$flowId\n"   
        }

        # send cookies in curl
        {
            # A cookie jar for storing the CSRF tokens
            cookieJar=$(mktemp) && flowId=$(curl -k -s -X GET --cookie-jar $cookieJar --cookie $cookieJar -H "Accept: application/json" https://auth.donation-app.test/authenticate/self-service/login/browser | jq -r '.id')
            # The endpoint uses Ory Identities' REST API to fetch information about the request (requires the CSRF cookie created for the login flow)
            curl -k -s -X GET --cookie-jar $cookieJar --cookie $cookieJar -H "Accept: application/json" "https://auth.donation-app.test/authenticate/self-service/login/flows?id=$flowId" | jq

            # TODO: check session kratos info
            # otherwise can check https://auth.donation-app.test/sessions
            {
                curl -k -i http://kratos-read:80/sessions/whoami
            }
        }
    }

    # registration flow 
    registration_flow() {
        flowId=$(curl -k -s -X GET -H "Accept: application/json" https://auth.donation-app.test/authenticate/self-service/registration/api | jq -r '.id')
        curl -k -s -X GET -H "Accept: application/json" "https://auth.donation-app.test/authenticate/self-service/registration/flows?id=$flowId" | jq
    }

}

