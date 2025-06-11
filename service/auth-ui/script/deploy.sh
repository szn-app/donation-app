## IMPORTANT! used in .github/workflows/*
build_container#package_hook@auth-ui() {(
    pushd kratos-selfservice-ui-node
    
    docker build . -t auth-ui:latest --target release

    popd
)}

func#predeploy-skaffold-hook@auth-ui() {
    local environment=${1:-dev}

    if [ "$environment" == "prod" ] || [ "$environment" = "staging" ]; then
        generate_config@auth-ui
    fi
}

func#postdeploy-skaffold-hook@auth-ui() {
    local environment=${1:-dev}
}