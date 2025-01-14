#!/bin/bash

# [obsolete]
manual_service_tag_version() { 
    local service="${1:-web-server}" 
    local version="${2:-0.1.0}" 

    # bump package.json version
    set_version() {
        local new_version="$1"

        jq --arg new_version "$new_version" '.version = $new_version' package.json > package.json.tmp
        mv package.json.tmp package.json

        echo "Version set to $new_version"
    }

    pushd ./service/$service
    set_version "$version"

    git add package.json
    git commit -m "$service $version version bump"

    popd
}

# [obsolete]
manual_release_package() {
    local service="${1:-web-server}" 
    local version="${2:-0.1.0}" 

    if ! git symbolic-ref --short HEAD | grep -q '^main$'; then
        echo "error: not on main branch."
        exit 1;
    fi

    if [[ -z "$(git diff --cached --name-only)" ]]; then
        echo "No staged files found. Proceeding..."
        if [[ -n "$(git status --porcelain)" ]]; then
            git stash 
        fi 
    else 
        echo "There are staged files. Please commit or stash them before proceeding."
        exit 1
    fi

    if [[ $# -gt 1 ]]; then
        service_tag_version $service $version
    fi

    git push origin 

    git tag $service-v$version
    git push --tags

    git stash pop > /dev/null 2>&1
}

example_workflow_with_release_please_manually_triggered() { 
    create_feature_pr() {
        feature_branch=$1
        git checkout -b $feature_branch
        git commit --allow-empty -m "commit 1" 
        git commit --allow-empty -m "commit 2" 
        git commit --allow-empty -m "commit 3"
        git push --set-upstream origin $feature_branch
        gh pr create --base main --head $feature_branch --title "feat: adding feacture x to component A" --fill-verbose
    }

    merge_last_pr() { 
        local feature_branch=$1
        git fetch origin 
        git checkout main 
        last_pr_number=$(gh pr list --state open --json number | jq -r '.[0].number') 
        default_branch=$(git remote show origin | grep "HEAD branch:" | awk '{print $3}')
        pr_title=$(gh pr view "$last_pr_number" --json title | jq -r '.title')
        gh pr merge $last_pr_number --squash -t "$pr_title"
        git pull && git push origin main 
    }

    local release_please_workflow=release.yml

    local feature_branch=branch_feature_x
    create_feature_pr $feature_branch

    # create release pr
    merge_last_pr $feature_branch # merge feature pr
    gh workflow run $release_please_workflow # -> release pr is created. 
    gh run list --workflow=$release_please_workflow 

    {
        # new features can be added and will be appended to the existing release PR managed by `release-please``.
        local feature_branch=branch_feature_y
        create_feature_pr $feature_branch
        
        # create release pr
        merge_last_pr $feature_branch # merge feature pr
        gh workflow run $release_please_workflow # -> release pr is created. 
        gh run list --workflow=$release_please_workflow 

        merge_last_pr $feature_branch # merge release pr
    }

    # create a package release on Github
    merge_last_pr $feature_branch # merge release pr
    gh workflow run $release_please_workflow # -> package a github release
}

delete_tag() { 
    tag=${1:-web-server-v0.1.1}
    git push origin :$tag
    git tag -d $tag
}

# NOTE: Dockerfile labels should associate package release to github repo (otherwise a manual web interface association is required)
github_container_registry_deploy() {
    TAG=web-server:latest
    docker tag $TAG ghcr.io/szn-app/donation-app/$TAG
    docker push ghcr.io/szn-app/donation-app/$TAG
}

env_files() { 
    _related_commands() { 
        find . -name '.env.template' 
        sed "s/<username>/your_username/g;s/<password>/your_password/g;s/YOUR_API_KEY/your_actual_api_key/g;s/YOUR_SECRET_KEY/your_actual_secret_key/g" < .env.template > .env
    }

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

    generate_secret_auth_ui() {
    # generate secrets for production
        auth_ui_secret_file="./manifest/auth_ui/production/secret.env"
        if [ ! -f "$auth_ui_secret_file" ]; then
            t=$(mktemp) && cat <<EOF > "$t"
COOKIE_SECRET="$(openssl rand -base64 32)"
CSRF_COOKIE_NAME="$(shuf -n 1 /usr/share/dict/words | tr -d '\n')_csrf"
CSRF_COOKIE_SECRET="$(openssl rand -base64 32)"
EOF

            mv $t $auth_ui_secret_file
            echo "generated secrets file: file://$auth_ui_secret_file" 
        fi
    }

    generate_database_kratos_credentials() {
        db_secret_file="./manifest/auth/db_kratos_secret.env"
        if [ ! -f "$db_secret_file" ]; then
            t=$(mktemp) && cat <<EOF > "$t"
DB_USER="$(shuf -n 1 /usr/share/dict/words | tr -d '\n')"
DB_PASSWORD="$(openssl rand -base64 32 | tr -dc 'A-Za-z0-9')"
EOF

            mv $t $db_secret_file
            echo "generated secrets file: file://$db_secret_file" 
        fi
    }

    generate_database_hydra_credentials() {
        db_secret_file="./manifest/auth/db_hydra_secret.env"
        if [ ! -f "$db_secret_file" ]; then
            t=$(mktemp) && cat <<EOF > "$t"
DB_USER="$(shuf -n 1 /usr/share/dict/words | tr -d '\n')"
DB_PASSWORD="$(openssl rand -base64 32 | tr -dc 'A-Za-z0-9')"
EOF

            mv $t $db_secret_file
            echo "generated secrets file: file://$db_secret_file" 
        fi
    }

    generate_database_kratos_credentials
    generate_database_hydra_credentials
    generate_secret_auth_ui
    create_env_files
}

# https://k8s.ory.sh/helm/
# $`install_ory_stack $kubeconfig`
# $`install_ory_stack $kubeconfig delete`
install_ory_stack() { 
    [ -z "$1" ] && { echo "Error: No arguments provided."; return 1; } || kubeconfig="$1" 
    action=${2:-"install"}

    {
        if [ "$action" == "delete" ]; then
            helm --kubeconfig $kubeconfig uninstall kratos -n auth
            helm --kubeconfig $kubeconfig uninstall postgres-kratos -n auth
            helm --kubeconfig $kubeconfig uninstall hydra -n auth
            helm --kubeconfig $kubeconfig uninstall postgres-hydra -n auth
            return 
        fi
    }
    
    pushd ./manifest/auth

        # ory stack charts
        helm --kubeconfig $kubeconfig repo add ory https://k8s.ory.sh/helm/charts
        # postgreSQL
        helm --kubeconfig $kubeconfig repo add bitnami https://charts.bitnami.com/bitnami 
        helm --kubeconfig $kubeconfig repo update

        {
            # spin database for user accounts
            set -a
            source db_kratos_secret.env
            set +a
            helm --kubeconfig $kubeconfig upgrade --reuse-values --install postgres-kratos bitnami/postgresql -n auth --create-namespace -f postgresql-kratos-values.yml \
                --set auth.username=${DB_USER} \
                --set auth.password=${DB_PASSWORD} \
                --set auth.database=kratos_db
            # this will generate 'postgres-kratos-postgresql' service

            ### install Ory Kratos
            # preprocess file through substituting env values
            t="$(mktemp).yml" && envsubst < ory-kratos-config.yml > $t && printf "replaced env variables in manifest: file://$t\n" 
            default_secret="$(openssl rand -hex 16)"
            cookie_secret="$(openssl rand -hex 16)"
            cipher_secret="$(openssl rand -hex 16)"
            helm --kubeconfig $kubeconfig upgrade --install kratos ory/kratos -n auth --create-namespace -f ory-kratos-values.yml -f $t \
                --set kratos.config.secrets.default[0]="$default_secret" \
                --set kratos.config.secrets.cookie[0]="$cookie_secret" \
                --set kratos.config.secrets.cipher[0]="$cipher_secret" \
                --set env[0].name=DB_USER --set env[0].value=${DB_USER} \
                --set env[0].name=DB_PASSWORD --set env[0].value=${DB_PASSWORD}
        }

        {
            set -a
            source db_hydra_secret.env # DB_USER, DB_PASSWORD
            set +a
            helm --kubeconfig $kubeconfig upgrade --reuse-values --install postgres-hydra bitnami/postgresql -n auth --create-namespace -f postgresql-hydra-values.yml \
                --set auth.username=${DB_USER} \
                --set auth.password=${DB_PASSWORD} \
                --set auth.database=hydra_db
            # this will generate 'postgres-hydra-postgresql' service

            ### install Ory Hydra
            # preprocess file through substituting env values
            t="$(mktemp).yml" && envsubst < ory-hydra-config.yml > $t && printf "replaced env variables in manifest: file://$t\n" 
            system_secret="$(LC_ALL=C tr -dc 'A-Za-z0-9' < /dev/urandom | head -c 32 | base64)" 
            cookie_secret="$(LC_ALL=C tr -dc 'A-Za-z0-9' < /dev/urandom | head -c 32 | base64)" 
            helm --kubeconfig $kubeconfig upgrade --install hydra ory/hydra -n auth --create-namespace -f ory-hydra-values.yml -f $t \
                --set kratos.config.secrets.system[0]="$system_secret" \
                --set kratos.config.secrets.cookie[0]="$cookie_secret" \
                --set env[0].name=DB_USER --set env[0].value=${DB_USER} \
                --set env[0].name=DB_PASSWORD --set env[0].value=${DB_PASSWORD}
        }

    popd

    manual_verify() { 
        kubectl --kubeconfig "$kubeconfig" port-forward -n auth service/kratos-admin 8083:80

        kubectl --kubeconfig $kubeconfig run -it --rm --image=nicolaka/netshoot debug-pod --namespace auth -- /bin/bash 
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
        source manifest/auth/db_kratos_secret.env
        set +a
        kubectl --kubeconfig $kubeconfig run -it --rm --image=postgres debug-pod --namespace auth --env DB_USER=$DB_USER --env DB_PASSWORD=$DB_PASSWORD -- /bin/bash
        {
            export PGPASSWORD=$DB_PASSWORD
            psql -h "postgres-kratos-postgresql" -U "$DB_USER" -d "kratos_db" -p 5432 -c "\dt" 
            psql -h "postgres-kratos-postgresql" -U "$DB_USER" -d "kratos_db" -p 5432 -c "SELECT * FROM identities;" 
        }

        # manage users using Ory Admin API through the CLI tool
        kubectl --kubeconfig $kubeconfig run -it --rm --image=nicolaka/netshoot debug-pod --namespace auth -- /bin/bash
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

# export kubeconfig="$(realpath ~/.ssh)/kubernetes-project-credentials.kubeconfig.yaml"
kustomize_kubectl() {
    [ -z "$1" ] && { echo "Error: No arguments provided."; return 1; } || kubeconfig="$1" 
    action=${2:-"install"}

    {
        if [ "$action" == "delete" ]; then
            kubectl --kubeconfig $kubeconfig delete -k ./manifest/entrypoint/production
            install_ory_stack $kubeconfig delete
            return 
         elif [ "$action" == "kustomize" ]; then
            pushd manifest/entrypoint/production 
            t="$(mktemp).yaml" && kubectl --kubeconfig $kubeconfig kustomize ./ > $t && printf "rendered manifest template: file://$t\n"  # code -n $t
            popd
            return
        fi
    }


    env_files

    install_ory_stack "$kubeconfig"

    pushd ./manifest 
        kubectl --kubeconfig $kubeconfig apply -k ./entrypoint/production
        {
            pushd ./entrypoint/production 
            t="$(mktemp).yaml" && kubectl --kubeconfig $kubeconfig kustomize ./ > $t && printf "rendered manifest template: file://$t\n"  # code -n $t
            popd
        }
    popd 
    
    echo "Services deployed to the cluster. NOTE: wait few minutes to complete startup and propagate TLS certificate generation"

    # verify cluster certificate issued successfully 
    _verify() {
        ### generate combined configuration
        kubectl kustomize ./manifest/gateway/development > ./tmp/combined_manifest.yml
        cat ./tmp/combined_manifest.yml | kubectl apply -f -

        kubectl kustomize ./
        kubectl get -k ./
        kubectl describe -k ./
        kubectl diff -k ./

        kubectl --kubeconfig $kubeconfig get clusterissuer -A # two issuers: staging & production issuers
        kubectl --kubeconfig $kubeconfig describe challenge -A # ephemeral challenge appearing during certificate issuance process
        kubectl --kubeconfig $kubeconfig get order -A # should be STATE = pending → STATE = valid
        kubectl --kubeconfig $kubeconfig get certificate -A # should be READY = True
        kubectl --kubeconfig $kubeconfig get httproute -A
        kubectl --kubeconfig $kubeconfig get gateway -A

        # check dns + web server response with tls staging certificate
        domain_name=""
        curl -I http://$domain_name
        curl --insecure -I https://$domain_name
        cloud_load_balancer_ip=""
        curl --header "Host: donation-app.com" $cloud_load_balancer_ip

        # run ephemeral debug container
        kubectl --kubeconfig $kubeconfig run -it --rm --image=nicolaka/netshoot debug-pod --namespace some_namespace -- /bin/bash 
        
    }

}