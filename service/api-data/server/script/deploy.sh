#!/bin/bash

misc@api-data() { 
    cargo build 
    cargo run 
    cargo build --release
}

# IMPORTANT! used by release.yaml workflow
build_container#package_hook@api-data-server() {
    if [ "$1" == "development" ]; then
        docker build . --target development -t api-data:latest # --build-arg ENV=development
    else
        docker build . --target production -t api-data:latest # --build-arg ENV=production
    fi
}

# IMPORTANT! used by docker image build; & github workflow
install.system-dependency@api-data-server() {
    # Rust protobuf compiler dependency
    apt update && apt upgrade -y && apt install -y protobuf-compiler libprotobuf-dev
}

run_container@api-data() {
    docker run -d -p 80:80 api-data
}

generate_config@api-data-server() {(
    pushd "$(realpath "$(dirname "$(dirname "${BASH_SOURCE[0]}")")")"
    pushd k8s/overlays/prod/

    local filename='config/.env.local'
    local secret_name='postgresql-credentials-user'
    local namespace='api-data'

    # Ensure secret exists
    if ! kubectl get secret $secret_name -n $namespace &>/dev/null; then
        echo "❌ Secret '$secret_name' not found in namespace '$namespace'"
        popd >/dev/null && popd >/dev/null
        return 1
    fi

    # Extract and decode values from secret
    local POSTGRESQL_USERNAME=$(kubectl get secret $secret_name -n $namespace -o jsonpath='{.data.username}' | base64 -d)
    local POSTGRESQL_PASSWORD=$(kubectl get secret $secret_name -n $namespace -o jsonpath='{.data.password}' | base64 -d)
    # local POSTGRESQL_USERNAME="${POSTGRESQL_USERNAME:-pguser_$(openssl rand -hex 4)}"
    # local POSTGRESQL_PASSWORD="${POSTGRESQL_PASSWORD:-$(openssl rand -base64 16 | head -c 16)}"

    mkdir -p $(dirname "$filename")
    cat <<EOF > $filename # overrides file
POSTGRESQL_USERNAME=$POSTGRESQL_USERNAME
POSTGRESQL_PASSWORD=$POSTGRESQL_PASSWORD
EOF
    
    popd
    popd
)}

func#predeploy-skaffold-hook@api-data-server() {(
    local environment=$1

    generate_config@api-data-server
)}

func#postdeploy-skaffold-hook@api-data-server() {(
    local environment=$1
)}