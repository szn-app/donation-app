skaffold#task@pgadmin4() { 
    pushd "$(dirname "$(dirname "${BASH_SOURCE[0]}")")" # two levels up: from script directory to project root
    
    skaffold dev --profile dev-watch --port-forward --tail

    verify() {
        skaffold render --profile prod
        skaffold delete --profile dev-watch
    }

    popd
}

delete@pgadmin4() { 
    helm delete --purge pgadmin4
}

dev_expose_service@pgadmin4() {
    export POD_NAME=$(kubectl get pods --namespace default -l "app.kubernetes.io/name=pgadmin4,app.kubernetes.io/instance=pgadmin4" -o jsonpath="{.items[0].metadata.name}")
    echo "Visit http://127.0.0.1:8080 to use your application"
    kubectl port-forward $POD_NAME 8080:80
}

delete.skaffold#task#manual-delete@pgadmin4() {
    local secret_name="pgadmin4-credentials"
    local namespace="default"
    printf "deleting secret $secret_name"
    kubectl delete secret $secret_name -n $namespace || true
}

diagnose.skaffold@pgadmin4() { 
    skaffold diagnose --module pgadmin4 --profile prod

    skaffold render --module pgadmin4 --profile staging-rebuild | grep -C 10 auth-ui

    kubectl kustomize ./k8s/overlays/staging
}
