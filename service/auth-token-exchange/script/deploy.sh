func#predeploy_hook@auth-token-exchange() {
    echo "prehook with $1"
}

func#postdeploy_hook@auth-token-exchange() {
    echo "prehook with $1"
}

# IMPORTANT! used by release.yaml workflow
build_container#package_hook@auth-token-exchange() { 
    # NOTE: uses buildx (instead of the legacy build)
    docker build . -t auth-token-exchange:latest
}