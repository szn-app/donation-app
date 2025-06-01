#!/bin/bash
set -e

skaffold#task@ory-kratos-db() {
    pushd "$(dirname "$(dirname "${BASH_SOURCE[0]}")")"
    
    skaffold dev --profile development --port-forward --tail

    popd
}

render.skaffold#task@ory-kratos-db() {
    pushd "$(dirname "$(dirname "${BASH_SOURCE[0]}")")" 
    temp_file=$(mktemp) && skaffold render --profile development > "$temp_file" && echo "$temp_file"
    popd
}

delete.skaffold@ory-kratos-db() {
    skaffold delete --profile development
}

delete_pvc@ory-kratos-db() {
    # delete protection finalizers
    kubectl get pv -o jsonpath='{range .items[*]}{.metadata.name}{"\t"}{.metadata.finalizers}{"\n"}{end}' | awk '{print $1}' | xargs -I {} kubectl patch pv {} -p '{"metadata":{"finalizers":null}}'

    kubectl delete pvc --all --force
    kubectl delete pv --all --force --ignore-not-found --v=9
    kubectl get pv -o jsonpath='{range .items[*]}{.metadata.name}{"\t"}{.metadata.finalizers}{"\n"}{end}' | awk '{print $1}' | xargs -I {} kubectl delete pv {}

    # Delete all persistent volume claims in the database namespace
    kubectl delete pvc --all -n auth --force

    # Wait to ensure PVCs are fully deleted
    echo "Waiting for PVCs to be deleted..."
    kubectl wait --for=delete pvc --all -n auth --timeout=60s || true

    # List any remaining PVCs that might be stuck
    remaining_pvcs=$(kubectl get pvc -n auth -o name 2>/dev/null)
    if [ -n "$remaining_pvcs" ]; then
        echo "Some PVCs still remain, trying to force delete with finalizers removal"
        kubectl get pvc -n auth -o name | xargs -I {} kubectl patch {} -n auth --type=merge -p '{"metadata":{"finalizers":null}}'
        kubectl delete pvc --all -n auth --force
    fi
}


info.cnpg@ory-kratos-db() { 
    local cluster_name="kratos--cluster-db"
    local namespace="auth"

    kubectl get pvc -n $namespace -l cnpg.io/cluster=$cluster_name

    echo "Describing Cluster resource..."
    kubectl describe cluster/$cluster_name -n $namespace

    echo "Listing Pods with wide output..."
    kubectl get pods -l cnpg.io/cluster=$cluster_name -n $namespace -o wide

    # Get the first Pod name related to the cluster
    pod_name=$(kubectl get pods -l cnpg.io/cluster=$cluster_name -n $namespace -o jsonpath='{.items[0].metadata.name}')

    echo "Describing Pod: $pod_name..."
    kubectl describe pod "$pod_name" -n "$namespace"

    echo "Fetching logs from Pod: $pod_name (all containers)..."
    kubectl logs "$pod_name" -n "$namespace" --all-containers=true

    echo "Fetching recent Events in namespace $namespace..."
    kubectl get events -n "$namespace" --sort-by='.metadata.creationTimestamp'

    # check for pods that may lead to pvc dependency issue
    kubectl get pods -l cnpg.io/cluster=$cluster_name -n $namespace -o wide
    # kubectl describe pod <pod_name> -n auth
}