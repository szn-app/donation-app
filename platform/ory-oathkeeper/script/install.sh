#!/bin/bash
set -e

generate_config#bootstrap@oathkeeper() {(
    pushd "$(realpath "$(dirname "$(dirname "${BASH_SOURCE[0]}")")")"

    CLIENT_NAME="oathkeeper-introspection"
    SECRET_NAME="ory-hydra-client--oathkeeper-introspection"
    CLIENT_SECRET=$(kubectl get secret "$SECRET_NAME" -n "auth" -o jsonpath='{.data.client_secret}')

    if [[ -z "$CLIENT_SECRET" ]]; then
        printf "Error: generate_config#bootstrap@oathkeeper function depends on create_oauth2_client_for_trusted_app@hydra"
    fi

    # create authorization header value
    OATHKEEPER_CLIENT_CREDENTIALS=$(printf "${CLIENT_NAME}:${CLIENT_SECRET}" | base64 -w 0)

    secret="./config/.env"

    t=$(mktemp) && cat <<EOF > "$t"
OATHKEEPER_CLIENT_CREDENTIALS="${OATHKEEPER_CLIENT_CREDENTIALS}"
EOF
    mv $t $secret
    echo "generated secrets file: file://$(readlink -f $secret)"
    
    popd
)}

### usecases: 
# Examples of Oathkeeper validation setups possible: 
#   a. JWT stored in a cookie
#   b. Kratos session token
#   c. Hydra OAuth2 access token in a cookie
install@oathkeeper() {
    set -e
    local environment="$1"

    # ory stack charts
    helm repo add ory https://k8s.ory.sh/helm/charts > /dev/null 2>&1
    # postgreSQL
    helm repo add bitnami https://charts.bitnami.com/bitnami > /dev/null 2>&1 
    helm repo update > /dev/null 2>&1 

    # used also to update access rules
    helm.install@oathkeeper() {
        local environment="$1"
        PROPERTIES_FILE="./config/.properties"
        HOSTNAME=$(grep -E '^HOSTNAME=' "$PROPERTIES_FILE" | awk -F'=' '{print $2}' | tr -d '\r')

        # Check if HOSTNAME was found
        if [ -z "$HOSTNAME" ]; then
            echo "Error: HOSTNAME not found or is empty in $PROPERTIES_FILE"
            return 1
        fi

        # t="$(mktemp).pem" && openssl genrsa -out "$t" 2048 # create private key
        # # Generate a JWKs file (if needed) - basic example using OpenSSL:
        # y="$(mktemp).json" && openssl rsa -in "$t" -pubout -outform PEM > $y 
        # echo "jwt file created file://$y"

        t="$(mktemp).yaml" && ./script/render-template.script.rs --environment $environment < ./oathkeeper-config.yaml.tera > $t && printf "generated manifest with replaced env variables: file://$t\n" 

        j="$(mktemp)-combined-access-rules.json"
        if [ "$environment" = "prod" ]; then
            jq -s '.[0]' ./config/access-rules.json > $j
            sed -i "s/donation-app\.local/$HOSTNAME/g" "$j"
            printf "combined json access-rules: file://$j\n" 
        elif [ "$environment" = "staging" ]; then 
            jq -s '.[0] + .[1]' ./config/access-rules.json ./config/test-access-rules.json > $j
            printf "combined json access-rules: file://$j\n" 
        else
            jq -s '.[0] + .[1]' ./config/access-rules.json ./config/test-access-rules.json > $j
            printf "combined json access-rules: file://$j\n" 
        fi
        
        l="$(mktemp).log" && helm upgrade --debug --install oathkeeper ory/oathkeeper -n auth --create-namespace -f ./helm-values.yaml -f $t \
                --set-file oathkeeper.accessRules=$j > $l 2>&1 && printf "Oathkeeper database logs: file://$l\n"
                # --set-file "oathkeeper.mutatorIdTokenJWKs=$y" 
    }

    printf "install Ory Aothkeeper \n"
    generate_config#bootstrap@oathkeeper
    helm.install@oathkeeper $environment

    # Wait for Oathkeeper deployment to be ready
    printf "Waiting for Oathkeeper deployment to be ready...\n"
    kubectl wait --for=condition=available deployment/oathkeeper --namespace=auth --timeout=300s

    verify() { 
        {
            # manually validate rendered deployment manifest files
            t="$(mktemp).yaml" && helm upgrade --dry-run --debug --install oathkeeper ory/oathkeeper -n auth --create-namespace -f ./helm-values.yaml -f ./oathkeeper-config.yaml --set-file 'oathkeeper.accessRules=./config/access-rules.json' > $t && printf "generated manifest with replaced env variables: file://$t\n"
        }

        oathkeeper rules validate --file ./config/access-rules.json

        curl -i https://auth.donation-app.local/authorize/health/alive
        curl -i https://auth.donation-app.local/authorize/health/ready
        curl https://auth.donation-app.local/authorize/.well-known/jwks.json | jq

        kubectl run -it --rm --image=nicolaka/netshoot debug-pod --namespace auth -- /bin/bash
        {
            curl  http://hydra-public/.well-known/jwks.json | jq # public keys used to verify JWT tokens
            curl http://oathkeeper-api/rules | jq "."
            curl http://hydra-admin/admin/clients | jq
        }

        curl -k -i https://test.donation-app.local/allow/ 
        curl -k -i -H "Accept: text/html" -X GET https://test.donation-app.local/allow/ 
        
        curl -k -i https://test.donation-app.local/deny
        
        curl -k -i https://test.donation-app.local/anonymous
        
        ACCESS_TOKEN=""
        curl -k -i -H "Authorization: Bearer $ACCESS_TOKEN" https://test.donation-app.local/jwt 
        curl -k -i -H "Authorization Bearer $ACCESS_TOKEN" https://test.donation-app.local/oauth-header
        
        # session cookie is tested by visiting the frontend application (can be done in cli with proper cookie handling)
        # + check also returned headers from mutators in oathkeeper service logs
        # https://test.donation-app.local/session-cookie
        # https://test.donation-app.local/keto-session
        # https://test.donation-app.local/keto-token

        # test authorizer handler
        curl -k -i https://test.donation-app.local/keto-static


        # NOTE: gaining authorization code process requires a browser or tool that handles consent; SDK libraries for Oauth and OIDC compose requests better
        oauth2_flow() {
            # initiate login flow and redirect to login page
            # following the process should redirect after login with the authorization code provided in the URL
            {
                {
                    printf "visit in browser %s" "https://auth.donation-app.local/authorize/oauth2/auth?client_id=frontend-client&response_type=code%20id_token&scope=offline_access%20openid&redirect_uri=https://donation-app.local&state=some_random_string&nonce=some_other_random_string"

                    # typically would run from within the cluster using the backend server of the frontend ui application (must be secure as it contains client secret)
                    # EXAMPLE for usage with client_secret_post

                    CLIENT_ID="frontend-client"
                    # CLIENT_SECRET=""
                    CLIENT_SECRET="$(kubectl get secret "ory-hydra-client--$CLIENT_ID" -n auth -o jsonpath='{.data.client_secret}' | base64 -d)"
                    # [manually eplace this] update the code from the result redirect url parameter after login
                    AUTHORIZATION_CODE=""
                    REDIRECT_URI="https://donation-app.local"
                }
                # or 
                {
                    printf "visit in browser %s" "https://auth.donation-app.local/authorize/oauth2/auth?client_id=frontend-client-oauth&response_type=code&scope=offline_access%20openid&redirect_uri=https://donation-app.local&state=some_random_string&nonce=some_random_str"

                    # typically would run from within the cluster using the backend server of the frontend ui application (must be secure as it contains client secret)
                    # EXAMPLE for usage with client_secret_post

                    CLIENT_ID="frontend-client-oauth"
                    CLIENT_SECRET="$(kubectl get secret "ory-hydra-client--$CLIENT_ID" -n auth -o jsonpath='{.data.client_secret}' | base64 -d)"
                    # CLIENT_SECRET=""
                    # [manually eplace this] update the code from the result redirect url parameter after login
                    AUTHORIZATION_CODE="ory_ac_qdyriUfO1jyHatQzcjZ4oTvqei-aB5BRREoY-XwAB2o.jrTz5KqJ_wZzbCTMWf0Gl4tyTnyJwz6c66Zyhd-YKHc"
                    REDIRECT_URI="https://donation-app.local"
                }
                # or 
                {
                    printf "visit in browser %s" "https://auth.donation-app.local/authorize/oauth2/auth?client_id=frontend-client-oauth-consent&response_type=code%20id_token&scope=offline_access%20openid&redirect_uri=https://donation-app.local&state=some_random_string&nonce=some_other_random_string"

                    # typically would run from within the cluster using the backend server of the frontend ui application (must be secure as it contains client secret)
                    # EXAMPLE for usage with client_secret_post

                    # [manually eplace this] update the code from the result redirect url parameter after login
                    AUTHORIZATION_CODE="ory_ac_FbhszJNnQ94P_Du28iCx75mVjM2LwJuheZtds3KEHCs.QCw9XsW1gDX04EtrEfGNwqbJ3hcoTETDBuvk9d1sFO4"
                    CLIENT_ID="frontend-client-oauth-consent"
                    CLIENT_SECRET="$(kubectl get secret "ory-hydra-client--$CLIENT_ID" -n auth -o jsonpath='{.data.client_secret}' | base64 -d)" # NOTE: the secret is retreived by kubectl and base64 is applied thus decoding is required
                    # CLIENT_SECRET=""
                    REDIRECT_URI="https://donation-app.local"
                }

            }

            # Execute the curl request
            # -v -s -o /dev/null  -k
            tokens_payload=$(curl -k -s --request POST --url https://auth.donation-app.local/authorize/oauth2/token --header "accept: application/x-www-form-urlencoded" \
                --form "grant_type=authorization_code" \
                --form "code=${AUTHORIZATION_CODE}" \
                --form "redirect_uri=${REDIRECT_URI}" \
                --form "client_id=${CLIENT_ID}" \
                --form "client_secret=${CLIENT_SECRET}" \
                --form "scope=offline_access openid" | jq)
            
            ACCESS_TOKEN=$(echo $tokens_payload | jq -r '.access_token')
            REFRESH_TOKEN=$(echo $tokens_payload | jq -r '.refresh_token')
            ID_TOKEN=$(echo $tokens_payload | jq -r '.id_token')

            # verify tokens
            kubectl run -it --rm --image=nicolaka/netshoot debug-pod --namespace auth -- /bin/bash
            {
                curl -k -i --request POST --url http://hydra-admin/admin/oauth2/introspect --header "accept: application/x-www-form-urlencoded" --form "token=$ACCESS_TOKEN" 
                # access restricted endpoint through Envoy Gateway + Oauthkeeper as introspection (calls http://oathkeeper-admin:80/decisions)
                curl -i -k -H "Authorization: Bearer $ACCESS_TOKEN" https://test.donation-app.local/oauth-header

                # request refresh token 
                curl -k -i --request POST --url https://auth.donation-app.local/authorize/oauth2/token --header "accept: application/x-www-form-urlencoded" \
                    --form "grant_type=refresh_token" \
                    --form "refresh_token=${REFRESH_TOKEN}" \
                    --form "redirect_uri=${REDIRECT_URI}" \
                    --form "client_id=${CLIENT_ID}"  \
                    --form "client_secret=${CLIENT_SECRET}" \
                    --form "scope=offline_access openid"
            }
            {
                # decode JWT id_token
                echo -n "$ID_TOKEN" | cut -d "." -f2 | base64 -d | jq .
            }
        }

        # directly request token with Hydra Oauth2.0 client id+secret
        client_secret_basic_method() {
            CLIENT_ID="internal-communication"
            CLIENT_SECRET="$(kubectl get secret "ory-hydra-client--$CLIENT_ID" -n auth -o jsonpath="{.data.client_secret}" | base64 -d)"
            # CLIENT_SECRET=""
            REDIRECT_URI="https://donation-app.local"
            # Base64 encode the client ID and secret
            BASE64_CREDENTIALS=$(echo -n "${CLIENT_ID}:${CLIENT_SECRET}" | base64 -w 0)
            echo $BASE64_CREDENTIALS
            # Construct the curl command

            tokens_payload=$(curl -s -k -X POST -H "Content-Type: application/x-www-form-urlencoded" \
                -H "Authorization: Basic ${BASE64_CREDENTIALS}" \
                -d "grant_type=client_credentials" \
                https://auth.donation-app.local/authorize/oauth2/token | jq)
            echo $tokens_payload
            
            ACCESS_TOKEN=$(echo $tokens_payload | jq -r '.access_token')
            echo $ACCESS_TOKEN

            # verify tokens
            {
                # access restricted endpoint through Envoy Gateway + Oauthkeeper as introspection (calls http://oathkeeper-admin:80/decisions)
                curl -i -k -H "Authorization: Bearer $ACCESS_TOKEN" https://test.donation-app.local/oauth-header
                # run within cluster
                kubectl run -it --rm --image=nicolaka/netshoot debug-pod-auth --namespace auth -- /bin/bash
                {
                    curl -k -i --request POST --url http://hydra-admin/admin/oauth2/introspect --header "accept: application/x-www-form-urlencoded" --form "token=$ACCESS_TOKEN" 
                }
                # check as JWT
                curl -i -k -H "Authorization: Bearer $ACCESS_TOKEN" https://test.donation-app.local/jwt
            }
        }
        
    }

    set +e
}

