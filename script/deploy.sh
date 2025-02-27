#!/bin/bash

load_scripts_recursive "./manifest/" 

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
    local environment="$1"

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

    generate_database_kratos_credentials
    generate_default_username_kratos
    generate_database_hydra_credentials
    generate_database_keto_credentials
    generate_secret_auth_ui $environment
    create_env_files
}

source ./script/library/install_ory_stack.sh

deploy_application() {
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
            kubectl delete -k ./manifest/entrypoint/$environment
            return 
        fi
    }

    pushd ./manifest 
        kubectl apply -k ./entrypoint/$environment

        if [ "$environment" == "development" ]; then
            # restart all deployments in any namespace
            kubectl get deployments --all-namespaces -o custom-columns=:.metadata.name,:.metadata.namespace | tail -n +2 | while read -r deployment namespace; do
                kubectl rollout restart deployment "$deployment" --namespace "$namespace"
            done
        fi

        {
            pushd ./entrypoint/$environment 
            t="$(mktemp).yml" && kubectl kustomize ./ > $t && printf "rendered manifest template: file://$t\n"  # code -n $t
            popd
        }
    popd 
}

restart_cilinium() {
    kubectl -n kube-system rollout restart deployment/cilium-operator
    kubectl -n kube-system rollout restart ds/cilium
}

{
    production_deploy() { 
        deploy --environment production --action install
    }
    development_deploy() { 
        deploy --environment development --action install
    }
}
# using kubectl, helm, kustomize
deploy() {
    local environment="development" # environment = development, production
    local action="install" # action = install, delete, kustomize, app

    while [[ "$#" -gt 0 ]]; do
        case $1 in
            --environment) environment="$2"; shift ;;
            --action) action="$2"; shift ;;
            *) echo "Unknown parameter passed: $1"; exit 1 ;;
        esac
        shift
    done

    if ! command -v kubectl-ctx &> /dev/null; then
        echo "kubectl ctx is not installed. Exiting."
        return
    fi

    # set kubectl context based on environment
    if [ "$environment" == "production" ]; then
        kubectl ctx k3s
    elif [ "$environment" == "development" ]; then
        kubectl ctx minikube
    else
        echo "Unknown environment: $environment"
        exit 1
    fi

    # create .env files from default template if doesn't exist
    env_files $environment

    {
        if [ "$action" == "delete" ]; then
            install_ory_stack --environment "$environment" --action delete 
            deploy_application --environment "$environment" --action delete
            return 
         elif [ "$action" == "kustomize" ]; then
            pushd manifest/entrypoint/"$environment"
            t="$(mktemp).yml" && kubectl kustomize ./ > $t && printf "rendered manifest template: file://$t\n"  # code -n $t
            popd
            return
         elif [ "$action" == "app" ]; then
            deploy_application --environment "$environment" --action install
            return
        elif [ "$action" != "install" ]; then
            # Call the function based on the argument
            if declare -f "$action" > /dev/null; then
                "$action" "$@" # Call the function
                return
            else
                echo "Unknown action: $action"
                return 
            fi
        fi
    }

    if [ "$environment" == "development" ]; then
        deploy_application --environment "$environment" --action delete # required to delete postgresql claims of ory stack
        install_ory_stack --environment "$environment" --action delete
    fi

    install_ory_stack --action $environment --action install
    deploy_application --environment "$environment" --action install
    
    echo "Services deployed to the cluster (wait few minutes to complete startup and propagate TLS certificate generation)."

    _fix() { 
        restart_cilinium  # [issue] restarting fixs gateway has no ip assignment by controller
    }
    # verify cluster certificate issued successfully 
    _verify() {
        ### generate combined configuration
        kubectl kustomize ./manifest/gateway/development > ./tmp/combined_manifest.yml
        cat ./tmp/combined_manifest.yml | kubectl apply -f -

        kubectl kustomize ./
        kubectl get -k ./
        kubectl --kubeconfig $kubeconfig  get -k ./
        kubectl describe -k ./
        kubectl diff -k ./

        kubectl get nodes --show-labels

        # cert-manager related 
        # two issuers: staging & production issuers 
        # ephemeral challenge appearing during certificate issuance process 
        # certificate should be READY = True
        # order: should be STATE = pending → STATE = valid
        kubectl get clusterissuer,certificate,order,challenge -A 
        kubectl get gateway,httproute,crds,securitypolicy -A 
        kubectl describe gateway -n gateway

        # check dns + web server response with tls staging certificate
        domain_name="donation-app.test"
        curl -i http://$domain_name
        curl --insecure -I https://$domain_name
        cloud_load_balancer_ip=""
        curl -i --header "Host: donation-app.test" $cloud_load_balancer_ip
        kubectl logs -n kube-system deployments/cilium-operator | grep gateway

        # run ephemeral debug container
        kubectl run -it --rm --image=nicolaka/netshoot debug-pod --namespace some_namespace -- /bin/bash 
        kubectl run -it --rm --image=busybox debug-pod-2 --namespace auth -- /bin/bash nslookup oathkeeper-proxy
        
        kubectl -n kube-system edit configmap cilium-config
    }

}

