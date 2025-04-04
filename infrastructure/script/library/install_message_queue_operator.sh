# Strimzi Kafka operator https://strimzi.io/quickstarts/
install_kafka_operator() {
    local operator_namespace="message-queue"
\
    # Create the namespace for the Kafka Operator
    kubectl create namespace "$operator_namespace" || true

    kubectl create -f "https://strimzi.io/install/latest?namespace=$operator_namespace" -n $operator_namespace

    echo "Kafka Operator installed successfully in the $operator_namespace namespace."

    verify() {
        kubectl get pod -n $operator_namespace --watch

        # follow the operator’s log:
        kubectl logs deployment/strimzi-cluster-operator -n $operator_namespace -f
    }
}