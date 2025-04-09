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