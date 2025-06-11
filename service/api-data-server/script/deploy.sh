#!/bin/bash

misc@api-data-server() { 
    cargo build 
    cargo run 
    cargo build --release
}

# IMPORTANT! used by release.yaml workflow
production.build_container#package_hook@api-data-server() {
    docker build . --target release -t api-data-server:latest # --build-arg ENV=production
}

build_container#package_hook@api-data-server() {
    docker build . --target debug -t api-data-server:latest # --build-arg ENV=development
}

# IMPORTANT! used by docker image build; & github workflow
install.system-dependency@api-data-server() {
    # Rust protobuf compiler dependency
    apt update && apt upgrade -y && apt install -y protobuf-compiler libprotobuf-dev
}

run_container@api-data-server() {
    docker run -d -p 80:80 api-data-server
}

func#predeploy-skaffold-hook@api-data-server() {(
    local environment=$1

    generate_config@api-data-server
)}

func#postdeploy-skaffold-hook@api-data-server() {(
    local environment=$1
)}