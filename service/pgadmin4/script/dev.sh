skaffold#task@pgadmin4() { 
    pushd "$(dirname "$(dirname "${BASH_SOURCE[0]}")")" # two levels up: from script directory to project root
    
    skaffold dev --profile development --port-forward --tail

    verify() {
        skaffold render --profile production
        skaffold delete --profile development
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
