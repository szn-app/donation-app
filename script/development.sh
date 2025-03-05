#!/bin/bash

misc_() {
    # modify permission
    find ./ -maxdepth 4 -name "script.sh" -exec chmod +x {} \;
}

# for feature branches and hotfixes.
feature_pull_request() {  
    if [[ $# -lt 1 ]]; then
        exit 1; 
    fi

    local feature_branch="${1:-feature/example}"
    git push origin $feature_branch
    
    # PR to trigger CI test
    gh pr create --head $feature_branch --base main --title "feat(frontend): new implementation feature" --fill-verbose
    # or merges but without triggering CI test
    {
        git checkout main
        git merge --squash $feature_branch -m "feat(frontend): new implementation feature"
    }

    # NOTE: automerge is applied only on PRs from branches that are prefix with "feature/*" or "hotfix/*".
}

deploy_local_minikube() {
    sudo echo "elevated permission"
    action=${1:-"install"}

    if [ "$action" == "delete" ]; then
        source ./script/deploy.sh
        deploy --environment development --action delete
        return 
    elif [ "$action" == "kustomize" ]; then
        source ./script/deploy.sh
        deploy --environment development --action kustomize
        return
    fi

    if ! minikube status &> /dev/null; then
        echo "Minikube is not running. Starting Minikube..."
        minikube start
    else
        echo "Minikube is already running."
    fi

    # manual_build_all_containers_with_load
    
    terminate_background_jobs
    
    deploy --environment development --action install
    # deploy only app: 
    # deploy --environment development --action app

    read -t 20 -p "Do you want to execute tunnel_minikube? (y/n, default is y after 20 seconds): " choice
    choice=${choice:-y}
    if [[ "$choice" == "y" ]]; then
        tunnel_minikube
    else
        echo "Skipping tunnel_minikube execution."
    fi

    example_scripts() {
        kubectl config view && kubectl get namespace && kubectl config get-contexts

        (cd k8s/development && kubectl apply -k .)
        kubectl get all
    
        minikube ip 
        # expose service to host: 
        minikube tunnel # expose all possible resources (e.g. loadbalancers)
        minikube service dev-web-server --url --namespace=donation-app

        nslookup donation-app.test $(minikube ip) # query dns server running in minikube cluaster
        dig donation-app.test
        export GW=$(minikube ip) # or direct gateway ip exposed using minikube tunnel.
        curl --resolve donation-app.test:80:$GW donation-app.test
        ping donation-app.test

        # using ingress 
        kubectl describe ingress ingress -n donation-app

        # using gateway 
        {
            export GW=$(minikube ip) # or direct gateway ip exposed using minikube tunnel.
            kubectl apply -k ./service/cilium-gateway/k8s/development
            minikube tunnel # otherwise, with ingress-dns and ingress.yml re-route to gateway will make accessing gateway through domain resolution directly with minikube ip
            minikube dashboard
            kubectl describe gateway -n donation-app
            kubectl describe httproute -n donation-app
            dig donation
            curl --resolve donation-app.test:80:$GW donation-app.test
        }

        kubectl apply -k ./kubernetes/overlays/dev

        curl -i --header "Host: donation-app.test" "<ip-of-load-balancer>"
    }
}

# TODO: alternative to above script
deploy_skaffold() {
    {
        minikube start --profile minikube --namespace donation-app # set default namespace for minikube
        skaffold config set --global local-cluster true
        eval $(minikube --profile minikube docker-env) # use docker daemon inside minikube
    }

    fix_sync_issue() {
        # fixes issue with namespaces affecting skaffold sync - https://github.com/GoogleContainerTools/skaffold/issues/1668#issuecomment-595752550
        # namespace config when set to undefined prevents sync errors
        # local current_ns; 
        current_ns="$(kubectl config view --minify --output 'jsonpath={..namespace}')"
        kubectl config set-context --current --namespace=
        (sleep 3 && kubectl config set-context --current --namespace="${current_ns}" ) &
    }

    set_user_inotify_limit() {
        # increase inotify limit for user if not already set
        if ! grep -q "fs.inotify.max_user_instances" /etc/sysctl.conf; then
            echo fs.inotify.max_user_instances=524288 | sudo tee -a /etc/sysctl.conf && sudo sysctl -p
            echo "Increased inotify limit for user"
        else
            echo "inotify limit already configured"
        fi
    }

    set_user_inotify_limit
    fix_sync_issue
    skaffold dev --profile development --port-forward --cleanup=false

    root_kustomize_skaffold() {
        skaffold run --filename skaffold-root-kustomize.yml --profile development --port-forward --cleanup=false
        
        skaffold run --filename skaffold-root-kustomize.yml --profile production --tail
    }

    # TODO:
    deploy_all() {
        services=("service-1" "service-2")

        for service in "${services[@]}"; do
            echo "Deploying $service..."
            cd "$service" && skaffold dev && cd ..
        done
    }

    freeup_minikube_space() { 
        minikube ssh df
        minikube ssh 'docker image prune -a -f'
        docker system prune --all --force
    }

    verify() { 
        kubectl config view
        skaffold config list
        skaffold render
        
        skaffold delete --profile development
        skaffold build -v debug 
        skaffold dev --port-forward -v debug
        skaffold debug
        skaffold run
        skaffold run --profile production --port-forward
    }
}
