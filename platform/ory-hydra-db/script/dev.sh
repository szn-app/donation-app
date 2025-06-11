#!/bin/bash
set -e

skaffold#task@ory-hydra-db() {
    pushd "$(dirname "$(dirname "${BASH_SOURCE[0]}")")"
    
    skaffold dev --profile dev-watch --port-forward --tail

    popd
}

render.skaffold#task@ory-hydra-db() {
    pushd "$(dirname "$(dirname "${BASH_SOURCE[0]}")")" 
    temp_file=$(mktemp) && skaffold render --profile dev-watch > "$temp_file" && echo "$temp_file"
    popd
}

delete.skaffold@ory-hydra-db() {
    skaffold delete --profile dev-watch
}

delete_pvc@ory-hydra-db() {
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
