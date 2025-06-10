generate-config@webhook-handler() {(
    pushd "$(realpath "$(dirname "$(dirname "${BASH_SOURCE[0]}")")")"

    local secret_name=kafka-user
    local source_namespace=kafka-message-queue
    local destination_namespace=donation-app

    printf "copy kafka-message-queue secret to webhook-handler namespace"
    # copy secret to service namespace

    # always overwrite secret
    kubectl delete secret "$secret_name" -n "$destination_namespace" --ignore-not-found=true

    kubectl get secret "$secret_name" -n "$source_namespace" -o yaml | \
        yq eval '. | { "apiVersion": "v1", "kind": "Secret", "metadata": { "name": .metadata.name, "namespace": "'"$destination_namespace"'" }, "data": .data }' - | \
        kubectl apply -f - \
    || true 
    
    popd
)}

func#predeploy_hook@webhook-handler() {
    echo "prehook with $1"
    
    generate-config@webhook-handler
}

func#postdeploy_hook@webhook-handler() {
    echo "prehook with $1"
}

# IMPORTANT! used by release.yaml workflow
build_container#package_hook@webhook-handler() {
    # NOTE: uses buildx (instead of the legacy build)
    docker build . -t webhook-handler:latest
}

verify.container@webhook-handler() {
    eval $(minikube --profile minikube docker-env) # use docker daemon inside minikube

    docker build -t webhook-handler:latest --target release . 

    # check if build works with current Cargo.toml options
    docker run -it -v $(pwd):/app -w /app rust /bin/bash

    docker run -it --entrypoint "/bin/ls" -v $(pwd):/app -w /app webhook-handler:latest -al
}

diagnose.ldd_discover_binary_dependencies@webhook-handler(){
    ldd ./target/debug/webhook-handler
}


# IMPORTANT! used by docker image build; & github workflow
install.system-dependency@webhook-handler() {
    # Rust protobuf compiler dependency
    apt update && apt upgrade -y && apt install -y protobuf-compiler libprotobuf-dev apt-utils
    # Kafka Rust dependency compiler requirements
    apt update && apt install -y cmake libsasl2-modules-gssapi-mit libsasl2-dev
}
