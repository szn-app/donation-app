create_kratos_identities() {
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
        kubectl exec -it setup-pod-kratos --namespace auth -- /bin/bash -c "chmod +x $t && $t" >/dev/null 2>&1
    }
    {
        
        if [ "$environment" = "production" ]; then
            set -a
                source admin_user_secret.env
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
        kubectl exec -it setup-pod-kratos --namespace auth -- /bin/bash -c "chmod +x $t && $t"

        # NOTE: this is error prone hackish way to get the uuid of the user from debug output of kratos cli
        uuid=$(kubectl exec -it setup-pod-kratos --namespace auth -- /bin/bash -c "kratos list identities --format json --endpoint http://kratos-admin:80 2> /dev/null | jq -r --arg email \"$username\" '.identities[] | select(.traits.email == \$email) | .id'")

        # create a secret to store the values
        kubectl create secret generic default-admin-user-credentials -n auth --from-literal=username="$username" --from-literal=uuid="$uuid" --from-literal=password="$password" --dry-run=client -o yaml | kubectl apply -f - >/dev/null 2>&1

    }

    # if [ "$environment" = "production" ]; then
    kubectl delete --force pod setup-pod-kratos -n auth > /dev/null 2>&1 || true
    # fi
}

install_kratos() {
    set -e
    local environment=$1

    generate_default_username_kratos() {( # use subshell to avoid change variables    
        local environment=production
        if [ -f ./.env.$environment ]; then
            source ./.env.$environment
        elif [ -f ./.env.$environment.local ]; then
            source ./.env.$environment.local
        else
            echo "Error: .env.$environment file not found."
            exit 1
        fi

        if [ -z "$DOMAIN_NAME" ]; then
            echo "Error: DOMAIN_NAME environment variable is not set"
            return 
        fi

        local username="admin-$(openssl rand -hex 8)@$DOMAIN_NAME"
        local password="$(openssl rand -base64 64 | tr -dc 'A-Za-z0-9!@#$%^&*()_+-=[]{}|;:,.<>?~`' | head -c 32)"

        file="./admin_user_secret.env"
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
    )}

    generate_database_kratos_credentials() {( # use subshell to avoid change variables
        db_secret_file="./db_kratos_secret.env"
        if [ ! -f "$db_secret_file" ]; then
            t=$(mktemp) && cat <<EOF > "$t"
DB_USER="$(shuf -n 1 /usr/share/dict/words | tr -d '\n')"
DB_PASSWORD="$(openssl rand -base64 32 | tr -dc 'A-Za-z0-9')"
EOF

            mv $t $db_secret_file
            echo "generated secrets file: file://$(readlink -f $db_secret_file)"
        else
            echo "Secrets file file://$(readlink -f $db_secret_file) already exists."
        fi

    )}

    # create .env files from default template if doesn't exist
    create_env_files() {
        # Find all *.env.template files
        find . -name "*.env.template" | while IFS= read -r template_file; do
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
    }

    # ory stack charts
    helm repo add ory https://k8s.ory.sh/helm/charts > /dev/null 2>&1
    # postgreSQL
    helm repo add bitnami https://charts.bitnami.com/bitnami > /dev/null 2>&1 
    helm repo update > /dev/null 2>&1 

    generate_kratos_env_file() {
        env_file_name="jsonnet.env"
        google_jsonnet_file="./google-oidc-mapper.jsonnet"

        # Check if the JSONNET file exists
        if [[ ! -f "$google_jsonnet_file" ]]; then
            echo "Error: File '$google_jsonnet_file' not found!"
            return 1
        fi

        # Read the JSONNET file and encode it as base64
        google_jsonnet_base64=$(base64 -w 0 < "$google_jsonnet_file")

        t=$(mktemp) && cat <<EOF > "$t"
GOOGLE_JSONNET_MAPPER_BASE64="$google_jsonnet_base64"
EOF
        mv $t $env_file_name
        echo "generated env file: file://$(readlink -f $env_file_name)"
    }

    generate_database_kratos_credentials
    generate_default_username_kratos
    create_env_files
    generate_kratos_env_file
    
    if helm list -n auth | grep -q 'postgres-kratos' && [ "$environment" = "development" ]; then
        upgrade_db=false
    else
        upgrade_db=true
    fi

    if [ "$upgrade_db" = true ]; then
        printf "install Postgresql for Ory Kratos \n"
        set -a
            source db_kratos_secret.env
        set +a
        
        l="$(mktemp).log" && helm upgrade --debug --reuse-values --install postgres-kratos bitnami/postgresql -n auth --create-namespace -f ./postgresql-values.yml \
            --set auth.username=${DB_USER} \
            --set auth.password=${DB_PASSWORD} \
            --set auth.database=kratos_db > $l 2>&1 && printf "Kratos database logs: file://$l\n"
        # this will generate 'postgres-kratos-postgresql' service
    fi

    printf "install Ory Kratos \n"
    {
        # preprocess file through substituting env values
        t="$(mktemp).yml" && cargo run --release --bin render-template-config -- --environment $environment < kratos-config.yaml.tera > $t && printf "generated manifest with replaced env variables: file://$t\n" 
        q="$(mktemp).yml" && cargo run --release --bin render-template-helm -- --environment $environment < helm-values.yaml.tera > $q && printf "generated manifest with replaced env variables: file://$q\n" 
        set -a
            source db_kratos_secret.env
        set +a
        default_secret="$(openssl rand -hex 16)"
        cookie_secret="$(LC_ALL=C tr -dc 'A-Za-z0-9' < /dev/urandom | head -c 32)" 
        cipher_secret="$(openssl rand -hex 16)"
        l="$(mktemp).log" && helm upgrade --debug --install --atomic kratos ory/kratos -n auth --create-namespace -f $q -f $t \
            --set-file kratos.identitySchemas.identity-schema\\.json=./identity-schema.json \
            --set kratos.config.secrets.default[0]="$default_secret" \
            --set kratos.config.secrets.cookie[0]="$cookie_secret" \
            --set kratos.config.secrets.cipher[0]="$cipher_secret" \
            --set env[0].name=DB_USER --set env[0].value=${DB_USER} \
            --set env[1].name=DB_PASSWORD --set env[1].value=${DB_PASSWORD} > $l 2>&1 && printf "kratos logs: file://$l\n"
    }

    # Wait for Kratos deployment to be ready
    printf "Waiting for Kratos deployment to be ready...\n"
    kubectl wait --for=condition=available deployment/kratos --namespace=auth --timeout=300s

    create_kratos_identities $environment

    verify_jsonnet() {
        kratos help jsonnet lint
        kratos jsonnet lint ./google-oidc-mapper.template.json
    }

    verify()  {
        {
            # manually validate rendered templates and deployment manifest files
            y="$(mktemp).yml" && helm upgrade --dry-run --install kratos ory/kratos -n auth --create-namespace -f $q -f $t \
                --set-file kratos.identitySchemas.identity-schema\\.json=./identity-schema.json \
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

    
    set +e
}
