func#predeploy-skaffold-hook@kafka-message-queue() {
    local environment=$1

}

func#postdeploy-skaffold-hook@kafka-message-queue() {
    local environment=$1

    {
        # wait till deployment stabilizes as it is controlled by an operator
        local cluster_name="kafka-message-queue-cluster"
        local namespace="kafka-message-queue"
        
        kubectl wait kafka/$cluster_name --for=condition=Ready --timeout=300s -n $namespace \
            && echo "Cluster is ready." \
            || echo "Warning: Timed out waiting for cluster readiness."
    }
}