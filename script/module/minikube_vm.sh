stop_minikube#task() {
    minikube stop --profile minikube
    minikube status

    sudo systemctl stop docker
    sudo systemctl status docker
}

start_minikube_services_only#task() {
    minikube start --profile minikube
    minikube status

    sudo systemctl start docker
    sudo systemctl status docker
}

# remove docker images and cleanup disk space
freeup_space.minikube#cleanup#task@monorepo() {
    minikube ssh df
    minikube ssh 'docker image prune -a -f'
    minikube ssh -- docker system prune
    docker system prune --all --force
    sudo docker system prune --volumes

    verify() {
        minikube ssh -- df -h
        df -h
        df -h /var
    }
}

minikube#aggregate_cleanup#task@monorepo() {
    execute.util '#minikube_cleanup' '#task'
}
