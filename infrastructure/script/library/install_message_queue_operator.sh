# Strimzi Kafka operator https://strimzi.io/quickstarts/
install.kafka-operator#task@infrastructure() {
    local operator_namespace="kafka-operator"
    # Create the namespace for the Kafka Operator
    kubectl create namespace "$operator_namespace" || true

    kubectl create -f "https://strimzi.io/install/latest?namespace=$operator_namespace" -n $operator_namespace

    # Wait for the operator to be ready
    kubectl wait --for=condition=Ready pod -l name=strimzi-cluster-operator -n $operator_namespace --timeout=300s

    # Patch WATCH_NAMESPACE to watch kafka-ns
    kubectl set env deployment/strimzi-cluster-operator WATCH_NAMESPACE="" -n "$operator_namespace" # watch all namespaces

    echo "Kafka Operator installed successfully in the $operator_namespace namespace, watching all namespaces for Kafka Strimzi CRDs."
}

delete.kafka-operator@infrastructure() {
    local operator_namespace="kafka-operator"

    kubectl -n $operator_namespace delete -f "https://strimzi.io/install/latest?namespace=$operator_namespace"
    kubectl delete namespace $operator_namespace
}

verify.kafka-operator@infrastructure() {
    kubectl get pod -n $operator_namespace --watch

    # follow the operator’s log:
    kubectl logs deployment/strimzi-cluster-operator -n $operator_namespace -f
    
    get_strimzi_version() {
        kubectl get deployment strimzi-cluster-operator -n $operator_namespace -o yaml | grep image: | awk '{print $2}'
    }
}
