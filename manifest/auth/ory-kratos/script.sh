generate_database_kratos_credentials() {
    db_secret_file="./manifest/auth/ory-kratos/db_kratos_secret.env"
    if [ ! -f "$db_secret_file" ]; then
        t=$(mktemp) && cat <<EOF > "$t"
DB_USER="$(shuf -n 1 /usr/share/dict/words | tr -d '\n')"
DB_PASSWORD="$(openssl rand -base64 32 | tr -dc 'A-Za-z0-9')"
EOF

        mv $t $db_secret_file
        echo "generated secrets file: file://$db_secret_file" 
    fi
}

install_kratos() {
    environment=$1

    pushd ./manifest/auth/ory-kratos

    check_kratos_secret_env() {
        # generate secrets for production
        secret_file="secret.env"
        if [ ! -f "$secret_file" ]; then
            prinf "./secret.env must exist ! otherwise OIDC for external providers will fail" 
        fi
    }

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
        echo "generated env file: file://$env_file_name" 
    }

    check_kratos_secret_env
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
        
        helm upgrade --debug --reuse-values --install postgres-kratos bitnami/postgresql -n auth --create-namespace -f ./postgresql-values.yml \
            --set auth.username=${DB_USER} \
            --set auth.password=${DB_PASSWORD} \
            --set auth.database=kratos_db
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
        helm upgrade --debug --install --atomic kratos ory/kratos -n auth --create-namespace -f $q -f $t \
            --set-file kratos.identitySchemas.identity-schema\\.json=./identity-schema.json \
            --set kratos.config.secrets.default[0]="$default_secret" \
            --set kratos.config.secrets.cookie[0]="$cookie_secret" \
            --set kratos.config.secrets.cipher[0]="$cipher_secret" \
            --set env[0].name=DB_USER --set env[0].value=${DB_USER} \
            --set env[1].name=DB_PASSWORD --set env[1].value=${DB_PASSWORD}
    }
    
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
            }
        }

        # registration flow 
        registration_flow() {
            flowId=$(curl -k -s -X GET -H "Accept: application/json" https://auth.donation-app.test/authenticate/self-service/registration/api | jq -r '.id')
            curl -k -s -X GET -H "Accept: application/json" "https://auth.donation-app.test/authenticate/self-service/registration/flows?id=$flowId" | jq
        }

    }

    popd
}
