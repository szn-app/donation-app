#!/bin/bash

skaffold#task@auth-ory-stack() {
    skaffold dev -p local-production --port-forward
}

delete.skaffold#task@auth-ory-stack() {
    skaffold delete -p local-production
}

delete.pvc@auth-ory-stack() {
    kubectl get pvc -n auth --field-selector=status.phase=Terminating
    
    force_delete_pods_holding_pvc() {
        local namespace="auth"
        local pod_name="" # TODO: 
        
        kubectl delete pod $pod_name -n $namespace --grace-period=0 --force
    }

    force_delete_pods_holding_pvc
    
    for pvc in kratos--cluster-db-1 kratos--cluster-db-1-wal; do
        kubectl patch pvc "$pvc" -n auth -p '{"metadata":{"finalizers":[]}}' --type=merge
        kubectl delete pvc "$pvc" -n auth --force --grace-period=0
    done

}