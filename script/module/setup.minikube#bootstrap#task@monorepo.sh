# run & expose gateway with minimum scaffold services
setup.minikube#bootstrap#task@monorepo() {
    # sudo echo "" # prompt for sudo password

    run_minikube() {
        # Check if minikube is already running
        if ! minikube status --profile minikube &>/dev/null; then
            echo "Starting minikube..."
            minikube start --profile minikube --namespace donation-app
        else
            echo "Minikube is already running"
        fi

        skaffold config set --global local-cluster true
        eval $(minikube --profile minikube docker-env) # use docker daemon inside minikube
    }

    fix_sync_issue() {
        # fixes issue with namespaces affecting skaffold sync - https://github.com/GoogleContainerTools/skaffold/issues/1668#issuecomment-595752550
        # namespace config when set to undefined prevents sync errors
        # local current_ns; 
        current_ns="$(kubectl config view --minify --output 'jsonpath={..namespace}')"
        kubectl config set-context --current --namespace=
        # (sleep 3 && kubectl config set-context --current --namespace="${current_ns}" ) &
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

    run_scaffold_only() {
        pushd scaffold && skaffold run --profile dev-watch && popd
    }

    run_minikube
    
    set_user_inotify_limit
    fix_sync_issue
    
    setup_repo_files@monorepo

    echo "successfully setup Minikube vM"
}
