# generating config must happen before skaffold runs because of limitation with skaffold's lifecycle hooks
generate_config#predeploy-hook@api-data-object-store() {
    pushd "$(dirname "$(dirname "${BASH_SOURCE[0]}")")" # two levels up: from script directory to project root

    local ACCESS_KEY="${ACCESS_KEY:-$(openssl rand -hex 8)}"
    local SECRET_KEY="${SECRET_KEY:-$(openssl rand -hex 16)}"

    mkdir -p "$(dirname "values.prod.yaml.local")"
    cat <<EOF > values.prod.yaml.local
tenant:
      configSecret:
        existingSecret: false
        accessKey: $ACCESS_KEY
        secretKey: $SECRET_KEY
EOF
    
    popd
}

func#predeploy-skaffold-hook@api-data-object-store() {
    local environment=$1
}

func#postdeploy-skaffold-hook@api-data-object-store() {
    local environment=$1

    # wait till deployment stabilizes as it is controlled by an operator
    {
        echo "Waiting for minio-operator deployment to stabilize..."
        kubectl wait --for=condition=Available deployment/minio-operator -n minio-operator --timeout=300s

        echo "Checking status of minio tenants..."
        kubectl get tenants.minio.min.io -n object-store -o wide

        echo "Waiting for minio tenant pods to be ready..."
        while true; do
            TENANT_PODS=$(kubectl get pods -n object-store -l minio.min.io/tenant=minio-object-store -o name)
            if [ -n "$TENANT_PODS" ]; then
                echo "Found minio tenant pods"
                break
            fi
            echo "No minio tenant pods found yet, waiting 5 seconds..."
            sleep 5
        done

        for pod in $TENANT_PODS; do
            kubectl wait --for=condition=Ready $pod -n object-store --timeout=300s || echo "Warning: $pod not ready"
        done

        echo "MinIO deployment complete."
    }
}