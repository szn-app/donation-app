#!/bin/bash
# Script for deploying Ory security stack (Kratos, Hydra, Keto, Oathkeeper)
# This script manages installation, verification, and deletion of the Ory security stack in Kubernetes

set -e  # Exit immediately if a command exits with a non-zero status

# NOTE: in hetzner environment deleting pvc with this apporach causes resource cleanup issues 
delete.pvc#pvc-manual-delete@auth-ory-stack() { 
    minikube ssh -- "sudo rm -rf /tmp/hostpath-provisioner/auth/"

    # delete protection finalizers
    kubectl get pv -o jsonpath='{range .items[*]}{.metadata.name}{"\t"}{.metadata.finalizers}{"\n"}{end}' | awk '{print $1}' | xargs -I {} kubectl patch pv {} -p '{"metadata":{"finalizers":null}}'

    kubectl delete pvc --all --force
    kubectl delete pv --all --force --ignore-not-found --v=9
    kubectl get pv -o jsonpath='{range .items[*]}{.metadata.name}{"\t"}{.metadata.finalizers}{"\n"}{end}' | awk '{print $1}' | xargs -I {} kubectl delete pv {}

    # Delete all persistent volume claims in the auth namespace
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

    delete_pvc#task@auth-ory-stack
}

delete_pvc#task@auth-ory-stack() {
    delete_pvc@ory-kratos-db
    delete_pvc@ory-keto-db
    delete_pvc@ory-hydra-db
}

# https://k8s.ory.sh/helm/
delete.skaffold#task#manual-delete@auth-ory-stack() {
    set e+

    # Uninstall Ory components
    helm uninstall kratos -n auth || echo "Failed to uninstall Kratos, may not exist"
    helm uninstall hydra -n auth || echo "Failed to uninstall Hydra, may not exist"
    helm uninstall keto -n auth || echo "Failed to uninstall Keto, may not exist" 
    helm uninstall oathkeeper -n auth || echo "Failed to uninstall Oathkeeper, may not exist"

    # Delete Hydra OAuth client secrets - continue even if they don't exist
    kubectl delete secret ory-hydra-client--frontend-client-oauth -n auth 2>/dev/null || echo "Secret ory-hydra-client--frontend-client-oauth not found or couldn't be deleted"
    kubectl delete secret ory-hydra-client--frontend-client -n auth 2>/dev/null || echo "Secret ory-hydra-client--frontend-client not found or couldn't be deleted"
    kubectl delete secret ory-hydra-client--internal-communication -n auth 2>/dev/null || echo "Secret ory-hydra-client--internal-communication not found or couldn't be deleted"
    kubectl delete secret ory-hydra-client--oathkeeper-introspection -n auth 2>/dev/null || echo "Secret ory-hydra-client--oathkeeper-introspection not found or couldn't be deleted"
}

manual_verify#example@auth-ory-stack() {
    # https://k8s.ory.sh/helm/
    helm_example() {
        # use --debug with `helm` for verbose output
        helm uninstall kratos -n auth
        
        delete_persistent_data_postgres() { 
            minikube ssh -- "sudo rm -rf /tmp/hostpath-provisioner/auth/"
            
            # delete protection finalizers
            kubectl get pv -o jsonpath='{range .items[*]}{.metadata.name}{"\t"}{.metadata.finalizers}{"\n"}{end}' | awk '{print $1}' | xargs -I {} kubectl patch pv {} -p '{"metadata":{"finalizers":null}}'

            kubectl delete pvc --all --force
            kubectl delete pv --all --force --ignore-not-found --v=9
            kubectl get pv -o jsonpath='{range .items[*]}{.metadata.name}{"\t"}{.metadata.finalizers}{"\n"}{end}' | awk '{print $1}' | xargs -I {} kubectl delete pv {}
        }
    }

    # tunnel to remote service 
    kubectl port-forward -n auth service/kratos-admin 8083:80

    kubectl run -it --rm --image=nicolaka/netshoot debug-pod --namespace auth -- /bin/bash 
    {
        $(nslookup kratos-admin)
        
        # execute from within `auth` cluster namespace
        # get an example payload from login and registration
        flow_id=$(curl -s -X GET -H "Accept: application/json" http://kratos-public/self-service/login/api  | jq -r '.id')
        curl -s -X GET -H "Accept: application/json" "http://kratos-public/self-service/login/flows?id=$flow_id" | jq

        flow_id=$(curl -s -X GET -H "Accept: application/json" http://kratos-public/self-service/registration/api | jq -r '.id')
        curl -s -X GET -H "Accept: application/json" "http://kratos-public/self-service/registration/flows?id=$flow_id" | jq
    }

    # verify database:
    set -a
    source service/auth-ory-stack/ory-kratos/db_kratos_secret.env
    set +a
    kubectl run -it --rm --image=postgres debug-pod --namespace auth --env DB_USER=$DB_USER --env DB_PASSWORD=$DB_PASSWORD -- /bin/bash
    {
        export PGPASSWORD=$DB_PASSWORD
        psql -h "postgres-kratos-postgresql" -U "$DB_USER" -d "kratos_db" -p 5432 -c "\dt" 
        psql -h "postgres-kratos-postgresql" -U "$DB_USER" -d "kratos_db" -p 5432 -c "SELECT * FROM identities;" 
    }

    # manage users using Ory Admin API through the CLI tool
    kubectl run -it --rm --image=nicolaka/netshoot debug-pod --namespace auth -- /bin/bash
    {
        export KRATOS_ADMIN_URL="http://kratos-admin" 
        # https://www.ory.sh/docs/kratos/reference/api
        curl -X GET "$KRATOS_ADMIN_URL/admin/health/ready"
        curl -X GET "$KRATOS_ADMIN_URL/admin/identities" -H "Content-Type: application/json" | jq
        list_all_sessions() {
            for identity_id in $(curl -X GET "$KRATOS_ADMIN_URL/admin/identities" -H "Content-Type: application/json" | jq -r '.[].id'); do
                echo "Sessions for Identity: $identity_id"
                curl -X GET "$KRATOS_ADMIN_URL/admin/identities/$identity_id/sessions" -H "Content-Type: application/json" | jq
                echo ""
            done
        }
        list_all_sessions

    }
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