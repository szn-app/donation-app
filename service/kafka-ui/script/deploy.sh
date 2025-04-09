generate_config#predeploy-skaffold-hook@kafka-ui() {(
    pushd "$(realpath "$(dirname "$(dirname "${BASH_SOURCE[0]}")")")"

    local template_file="values.template.yaml"
    local filename="config/values.yaml.local"

    # if [ -f $filename ]; then
    #     echo "$filename already exists, skipping generation"
    #     popd
    #     return
    # fi

    local secret_name="kafka-user"
    local source_namespace="kafka-message-queue"

    export KAFKA_USERNAME=$(kubectl get secret "$secret_name" -n "$source_namespace" -o yaml | yq eval '.data."sasl.jaas.config"' - | base64 -d | grep -o 'username="[^"]*"' | cut -d'"' -f2)
    export KAFKA_PASSWORD=$(kubectl get secret "$secret_name" -n "$source_namespace" -o yaml | yq eval '.data.password' - | base64 -d)

    mkdir -p "$(dirname $filename)"
    envsubst < $template_file > $filename

    echo "generated config file: $filename"

    popd
)}

func#predeploy-skaffold-hook@kafka-ui() {
    local environment=$1

    helm repo add kafka-ui https://provectus.github.io/kafka-ui-charts
    
}

func#postdeploy-skaffold-hook@kafka-ui() {
    local environment=$1

}
