create_secret#predeploy-hook@pgadmin4() {(
    pushd "$(realpath "$(dirname "$(dirname "${BASH_SOURCE[0]}")")")"

    local filename="config/.env.dev"
    local secret_name="pgadmin4-credentials"
    local namespace="default"
    if [ ! -f "$filename" ]; then
        echo "Error: File $filename not found"
        return 1
    fi

    # Check if the secret already exists and delete it if it does
    if kubectl get secret $secret_name -n $namespace >/dev/null 2>&1; then
        echo "Secret $secret_name already exists"
        return 0
    fi

    # Create the secret from file
    echo "Creating secret $secret_name from $filename"
    kubectl create secret generic $secret_name -n $namespace --from-env-file="$filename"
    
    popd
)}


func#predeploy-skaffold-hook@pgadmin4() {
    local environment=$1

    # create_secret#predeploy-hook@pgadmin4

    helm repo add runix https://helm.runix.net
}

func#postdeploy-skaffold-hook@pgadmin4() {
    local environment=$1
}