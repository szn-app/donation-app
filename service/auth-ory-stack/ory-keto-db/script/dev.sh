#!/bin/bash
set -e

skaffold#task@ory-keto-db() {
    pushd "$(dirname "$(dirname "${BASH_SOURCE[0]}")")"
    
    skaffold dev --profile development --port-forward --tail

    popd
}

render.skaffold#task@ory-keto-db() {
    pushd "$(dirname "$(dirname "${BASH_SOURCE[0]}")")" 
    temp_file=$(mktemp) && skaffold render --profile development > "$temp_file" && echo "$temp_file"
    popd
}

delete.skaffold@ory-keto-db() {
    skaffold delete --profile development
}

delete_pvc#task@ory-keto-db() {
    # delete protection finalizers
    kubectl get pv -o jsonpath='{range .items[*]}{.metadata.name}{"\t"}{.metadata.finalizers}{"\n"}{end}' | awk '{print $1}' | xargs -I {} kubectl patch pv {} -p '{"metadata":{"finalizers":null}}'

    kubectl delete pvc --all --force
    kubectl delete pv --all --force --ignore-not-found --v=9
    kubectl get pv -o jsonpath='{range .items[*]}{.metadata.name}{"\t"}{.metadata.finalizers}{"\n"}{end}' | awk '{print $1}' | xargs -I {} kubectl delete pv {}

    # Delete all persistent volume claims in the database namespace
    kubectl delete pvc --all -n database --force

    # Wait to ensure PVCs are fully deleted
    echo "Waiting for PVCs to be deleted..."
    kubectl wait --for=delete pvc --all -n database --timeout=60s || true

    # List any remaining PVCs that might be stuck
    remaining_pvcs=$(kubectl get pvc -n database -o name 2>/dev/null)
    if [ -n "$remaining_pvcs" ]; then
        echo "Some PVCs still remain, trying to force delete with finalizers removal"
        kubectl get pvc -n database -o name | xargs -I {} kubectl patch {} -n database --type=merge -p '{"metadata":{"finalizers":null}}'
        kubectl delete pvc --all -n database --force
    fi

}
