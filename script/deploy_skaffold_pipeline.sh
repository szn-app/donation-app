#!/bin/bash

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
    
    # expose gateway with minimum scaffold services
    {
        source ./script.sh 
        pushd scaffold 
        skaffold run --profile development
        tunnel_minikube
        popd
    }

    skaffold dev --tail --profile development --port-forward --auto-build=false --auto-deploy=false --cleanup=false

    other() {
        skaffold run --profile production --tail
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
        skaffold diagnose
        
        skaffold delete --profile development
        skaffold build -v debug 
        skaffold dev --tail --profile development --port-forward --auto-build=false --auto-deploy=false --cleanup=false 
        skaffold dev --port-forward -v debug
        skaffold debug
        skaffold run
        skaffold run --profile production --port-forward
    }
}
