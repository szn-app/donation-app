#!/bin/bash
# Script for deploying Ory security stack (Kratos, Hydra, Keto, Oathkeeper)
# This script manages installation, verification, and deletion of the Ory security stack in Kubernetes

set -e  # Exit immediately if a command exits with a non-zero status

func#predeploy_hook@auth-ory-stack() {
    local environment=${1:-development}
    
    # Check if Kratos service exists in the auth namespace
    if kubectl get service kratos-admin -n auth &>/dev/null; then
        echo "Kratos service already exists, skipping installation"
    else 
        pushd ory-kratos && source script.sh && install@kratos $environment && popd
    fi
    
    # Check if Hydra service exists in the auth namespace
    if kubectl get service hydra-admin -n auth &>/dev/null; then
        echo "Hydra service already exists, skipping installation"
    else
        pushd ory-hydra && source script.sh && install@hydra $environment && popd
    fi
    
    # Check if Keto service exists in the auth namespace
    if kubectl get service keto-read -n auth &>/dev/null; then
        echo "Keto service already exists, skipping installation"
    else
        pushd ory-keto && source script.sh && install@keto $environment && popd
    fi

    # Check if Oathkeeper service exists in the auth namespace
    if kubectl get service oathkeeper-api -n auth &>/dev/null; then
        echo "Oathkeeper service already exists, skipping installation"
    else
        pushd ory-oathkeeper && source script.sh && install@oathkeeper $environment && popd
    fi

    manual_verify() {
        # use --debug with `helm` for verbose output
        
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
}

func#postdeploy_hook@auth-ory-stack() {
    local environment=${1:-development}
    echo "Posthook $environment"
}

# https://k8s.ory.sh/helm/
delete@auth-ory-stack() {
    local environment="${1:-development}" # environment = development, production

    helm uninstall kratos -n auth
    helm uninstall hydra -n auth
    helm uninstall keto -n auth
    helm uninstall oathkeeper -n auth

    helm uninstall postgres-kratos -n auth
    helm uninstall postgres-hydra -n auth
    helm uninstall postgres-keto -n auth


    if [ "$environment" == "development" ]; then
        minikube ssh -- "sudo rm -rf /tmp/hostpath-provisioner/auth/"

        kubectl delete secret ory-hydra-client--frontend-client-oauth -n auth
        kubectl delete secret ory-hydra-client--frontend-client -n auth
        kubectl delete secret ory-hydra-client--internal-communication -n auth
        kubectl delete secret ory-hydra-client--oathkeeper-introspection -n auth

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
    fi
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
