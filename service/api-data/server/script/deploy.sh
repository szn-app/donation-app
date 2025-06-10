#!/bin/bash

misc@api-data() { 
    cargo build 
    cargo run 
    cargo build --release
}

# IMPORTANT! used by release.yaml workflow
build_container#package_hook@api-data-server() {
    if [ "$1" == "development" ]; then
        docker build . --target debug -t api-data:latest # --build-arg ENV=development
    else
        docker build . --target release -t api-data:latest # --build-arg ENV=production
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

func#predeploy-skaffold-hook@api-data-server() {(
    local environment=$1

    generate_config@api-data-server
)}

func#postdeploy-skaffold-hook@api-data-server() {(
    local environment=$1
)}