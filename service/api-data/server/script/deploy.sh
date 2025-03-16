misc_api_data() { 
    cargo build 
    cargo run 
    cargo build --release
}

# IMPORTANT! used by release.yml workflow
build_container_api_data() {
    if [ "$1" == "development" ]; then
        docker build . --target development -t api-data:latest # --build-arg ENV=development
    else
        docker build . --target production -t api-data:latest # --build-arg ENV=production
    fi
}

run_docker_api_data() {
    docker run -d -p 80:80 api-data
}

generate_config@api-data-server() {
    pushd "$(dirname "$(dirname "${BASH_SOURCE[0]}")")" # two levels up: from script directory to project root
    pushd k8s/overlays/prod/

    local filename='.env.local'
    
    if [ -f "$filename" ]; then
        echo "config file: $filename already exists"
        return
    fi

    local POSTGRESQL_USERNAME="${POSTGRESQL_USERNAME:-pguser_$(openssl rand -hex 4)}"
    local POSTGRESQL_PASSWORD="${POSTGRESQL_PASSWORD:-$(openssl rand -base64 16 | head -c 16)}"
    
    cat <<EOF > $filename
POSTGRESQL_USERNAME=$POSTGRESQL_USERNAME
POSTGRESQL_PASSWORD=$POSTGRESQL_PASSWORD
EOF
    
    popd
    popd
}

func#predeploy-skaffold-hook@api-data-server() {
    local environment=$1

    generate_config@api-data-server
}

func#postdeploy-skaffold-hook@api-data-server() {
    local environment=$1
}