skaffold#task@api-data-object-store() { 
    pushd "$(dirname "$(dirname "${BASH_SOURCE[0]}")")" # two levels up: from script directory to project root
    
    skaffold dev --profile development --port-forward --tail

    verify() {
        skaffold render --profile production
        skaffold delete --profile development
    }

    popd
}

example@api-data-object-store() {
    watch kubectl get all -n object-store

    {
        mc --help # https://min.io/docs/minio/linux/reference/minio-mc.html#minio-mc-commands
        
        kubectl port-forward svc/minio-object-store-hl 9000 -n object-store

        mc alias set minio-object-store https://localhost:9000 ACCESS_KEY SECRET_KEY --insecure

        mc admin info minio-object-store

        # create bucket
        mc mb minio-object-store/mybucket --insecure
    }

    # ssh into minikube and navigate to the corresponding persistent volume to view the files
}

