# https://k8s.ory.sh/helm/
# $`install_ory_stack
# $`install_ory_stack --action delete
install_ory_stack() {
    local environment="development" # environment = development, production
    local action="install" # action = install, delete

    while [[ "$#" -gt 0 ]]; do
        case $1 in
            --environment) environment="$2"; shift ;;
            --action) action="$2"; shift ;;
            *) echo "Unknown parameter passed: $1"; exit 1 ;;
        esac
        shift
    done

    {
        if [ "$action" == "delete" ]; then
            helm uninstall kratos -n auth
            helm uninstall hydra -n auth
            helm uninstall keto -n auth
            helm uninstall oathkeeper -n auth

            helm uninstall postgres-kratos -n auth
            helm uninstall postgres-hydra -n auth
            helm uninstall postgres-keto -n auth


            if [ "$environment" == "development" ]; then
                minikube ssh -- "sudo rm -rf /tmp/hostpath-provisioner/auth/"

                kubectl delete secret ory-hydra-client--frontend-client-oauth -n auth
                kubectl delete secret ory-hydra-client--frontend-client -n auth
                kubectl delete secret ory-hydra-client--internal-communication -n auth
                kubectl delete secret ory-hydra-client--oathkeeper-introspection -n auth

                # delete protection finalizers
                kubectl get pv -o jsonpath='{range .items[*]}{.metadata.name}{"\t"}{.metadata.finalizers}{"\n"}{end}' | awk '{print $1}' | xargs -I {} kubectl patch pv {} -p '{"metadata":{"finalizers":null}}'

                kubectl delete pvc --all --force
                kubectl delete pv --all --force --ignore-not-found --v=9
                kubectl get pv -o jsonpath='{range .items[*]}{.metadata.name}{"\t"}{.metadata.finalizers}{"\n"}{end}' | awk '{print $1}' | xargs -I {} kubectl delete pv {}
            fi

            return 
        fi
    }

    load_scripts_recursive.util() {
        SCRIPT_DIR="$1"  # Get directory from argument

        # Validate input
        if [[ -z "$SCRIPT_DIR" ]]; then
            echo "Usage: $0 <script_directory>"
            return 1
        elif [[ ! -d "$SCRIPT_DIR" ]]; then
            echo "Error: '$SCRIPT_DIR' is not a valid directory."
            return 1
        fi

        # Find and source all .sh scripts recursively
        for script in $(find "$SCRIPT_DIR" -type f -name "*.sh"); do
            echo "Sourcing $script..."
            source "$script"
        done
    }

    load_scripts_recursive.util .

    env_files() {
        local environment="$1"


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
                            echo "created env file file://$env_file from $template_file"
                        fi
                done
        }

        generate_database_kratos_credentials
        generate_default_username_kratos
        generate_database_hydra_credentials
        generate_database_keto_credentials
        create_env_files
    }

    # ory stack charts
    helm repo add ory https://k8s.ory.sh/helm/charts > /dev/null 2>&1
    # postgreSQL
    helm repo add bitnami https://charts.bitnami.com/bitnami > /dev/null 2>&1 
    helm repo update > /dev/null 2>&1 

    env_files $environment
    install_kratos $environment
    install_hydra $environment
    install_keto # depends on `install_kratos`
    create_oauth2_client_for_trusted_app $environment
    install_oathkeeper # depends on `create_oauth2_client_for_trusted_app`

    manual_verify() {
        # use --debug with `helm` for verbose output
        
        # tunnel to remote service 
        kubectl port-forward -n auth service/kratos-admin 8083:80

        kubectl run -it --rm --image=nicolaka/netshoot debug-pod --namespace auth -- /bin/bash 
        {
            $(nslookup kratos-admin)
            
            # execute from within `auth` cluster namespace
            # get an example payload from login and registration
            flow_id=$(curl -s -X GET -H "Accept: application/json" http://kratos-public/self-service/login/api  | jq -r '.id')
            curl -s -X GET -H "Accept: application/json" "http://kratos-public/self-service/login/flows?id=$flow_id" | jq

            flow_id=$(curl -s -X GET -H "Accept: application/json" http://kratos-public/self-service/registration/api | jq -r '.id')
            curl -s -X GET -H "Accept: application/json" "http://kratos-public/self-service/registration/flows?id=$flow_id" | jq
        }

        # verify database:
        set -a
        source service/auth-ory-stack/ory-kratos/db_kratos_secret.env
        set +a
        kubectl run -it --rm --image=postgres debug-pod --namespace auth --env DB_USER=$DB_USER --env DB_PASSWORD=$DB_PASSWORD -- /bin/bash
        {
            export PGPASSWORD=$DB_PASSWORD
            psql -h "postgres-kratos-postgresql" -U "$DB_USER" -d "kratos_db" -p 5432 -c "\dt" 
            psql -h "postgres-kratos-postgresql" -U "$DB_USER" -d "kratos_db" -p 5432 -c "SELECT * FROM identities;" 
        }

        # manage users using Ory Admin API through the CLI tool
        kubectl run -it --rm --image=nicolaka/netshoot debug-pod --namespace auth -- /bin/bash
        {
            export KRATOS_ADMIN_URL="http://kratos-admin" 
            # https://www.ory.sh/docs/kratos/reference/api
            curl -X GET "$KRATOS_ADMIN_URL/admin/health/ready"
            curl -X GET "$KRATOS_ADMIN_URL/admin/identities" -H "Content-Type: application/json" | jq
            list_all_sessions() {
                for identity_id in $(curl -X GET "$KRATOS_ADMIN_URL/admin/identities" -H "Content-Type: application/json" | jq -r '.[].id'); do
                    echo "Sessions for Identity: $identity_id"
                    curl -X GET "$KRATOS_ADMIN_URL/admin/identities/$identity_id/sessions" -H "Content-Type: application/json" | jq
                    echo ""
                done
            }
            list_all_sessions

        }
    }
}
