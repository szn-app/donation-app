func#predeploy-skaffold-hook@api-data-database() {
    local environment=$1
}

func#postdeploy-skaffold-hook@api-data-database() {
    local environment=$1

    # wait till deployment stabilizes as it is controlled by an operator
    kubectl wait --for=condition=Ready --timeout=120s Cluster/cluster-app-data -n database \
        && echo "Cluster is ready." \
        || echo "Warning: Timed out waiting for cluster readiness."

    kubectl wait --for=condition=Ready pods -l cnpg.io/cluster=cluster-app-data -n database --timeout=120s \
        && echo "All CNPG pods are ready." \
        || echo "Warning: Timed out waiting for CNPG pods readiness."
}