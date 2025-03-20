# generating config must happen before skaffold runs because of limitation with skaffold's lifecycle hooks
generate_config#predeploy-hook@api-data-object-store() {(
    pushd "$(realpath "$(dirname "$(dirname "${BASH_SOURCE[0]}")")")"
    local filename="config/values.prod.yaml.local"

    if [ -f $filename ]; then
        echo "$filename already exists, skipping generation"
        popd
        return
    fi

    local ACCESS_KEY="${ACCESS_KEY:-$(openssl rand -hex 8)}"
    local SECRET_KEY="${SECRET_KEY:-$(openssl rand -hex 16)}"

    mkdir -p "$(dirname $filename)"
    cat <<EOF > $filename
tenant:
      configSecret:
        existingSecret: false
        accessKey: $ACCESS_KEY
        secretKey: $SECRET_KEY
EOF

    echo "generated config file: $filename"
    
    popd
)}

func#predeploy-skaffold-hook@api-data-object-store() {
    local environment=$1
}

func#postdeploy-skaffold-hook@api-data-object-store() {
    local environment=$1
    local namespace='api-data'

    # wait till deployment stabilizes as it is controlled by an operator
    {
        echo "Waiting for minio-operator deployment to stabilize..."
        kubectl wait --for=condition=Available deployment/minio-operator -n minio-operator --timeout=300s

        echo "Checking status of minio tenants..."
        kubectl get tenants.minio.min.io -n $namespace -o wide

        echo "Waiting for minio tenant pods to be ready..."
        for i in $(seq 1 10); do
            TENANT_PODS=$(kubectl get pods -n $namespace -l minio.min.io/tenant=minio-object-store -o name)
            if [ -n "$TENANT_PODS" ]; then
                echo "Found minio tenant pods"
                break
            fi

            echo "No minio tenant pods found yet, waiting 5 seconds..."
            sleep 5
        done

        for pod in $TENANT_PODS; do
            kubectl wait --for=condition=Ready $pod -n $namespace --timeout=300s || echo "Warning: $pod not ready"
        done

        echo "MinIO deployment complete."
    }
}