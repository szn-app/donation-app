# skaffold takes care of building instead.
manual_build_all_containers_with_load() {
    # alternative approach to build all containers directly into minikube
    build_all_containers_directly_into_minikube() {
        # bind docker images directly inside minikube
        eval $(minikube docker-env) # bind docker command to minikube docker
        (cd service/web-server && ./script.sh build_container_web_server development)
        (cd service/auth-ui && ./script.sh bulid_container_auth_ui)
        (cd service/auth-token-exchange && ./script.sh build_container_auth_token_exchange)

        { # reverse minikube eval
            unset DOCKER_TLS_VERIFY
            unset DOCKER_HOST
            unset DOCKER_CERT_PATH
        }
    }

    (cd service/web-server && ./script.sh build_container_web_server development)
    docker save web-server:latest | (eval $(minikube docker-env) && docker load)

    (cd service/auth-ui && ./script.sh bulid_container_auth_ui)
    docker save auth-ui:latest | (eval $(minikube docker-env) && docker load)
    
    (cd service/auth-token-exchange && ./script.sh build_container_auth_token_exchange)
    docker save auth-token-exchange:latest | (eval $(minikube docker-env) && docker load)

    (cd service/api-data && ./script.sh build_container_api_data development)
    docker save api-data:latest | (eval $(minikube docker-env) && docker load)
}

