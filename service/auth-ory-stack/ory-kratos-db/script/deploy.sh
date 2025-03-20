generate_database_credentials@ory-kratos-db() {( # use subshell to avoid change variables
    pushd "$(realpath "$(dirname "$(dirname "${BASH_SOURCE[0]}")")")"

    local filename="./k8s/overlays/prod/user.env.local"

    if [ ! -f "$filename" ]; then
        t=$(mktemp) && cat <<EOF > "$t"
# must be same as username for database defined in cluster yaml config
username=postgres-user
password=$(openssl rand -base64 32 | tr -dc 'A-Za-z0-9')
EOF

        mv $t $filename
        echo "generated secrets file: file://$(readlink -f $filename)"
    else
        echo "Secrets file file://$(readlink -f $filename) already exists."
    fi

    popd
)}

func#predeploy-skaffold-hook@ory-kratos-db() {
    local environment=$1

    if [ "$environment" != "development" ]; then
        generate_database_credentials@ory-kratos-db
    fi
}

func#postdeploy-skaffold-hook@ory-kratos-db() {
    local environment=$1

    {
        # wait till deployment stabilizes as it is controlled by an operator
        local cluster_name="kratos--cluster"
        local namespace="auth"
        
        kubectl wait --for=condition=Ready --timeout=120s Cluster/$cluster_name -n $namespace \
            && echo "Cluster is ready." \
            || echo "Warning: Timed out waiting for cluster readiness."

        kubectl wait --for=condition=Ready pods -l cnpg.io/cluster=$cluster_name -n $namespace --timeout=120s \
            && echo "All CNPG pods are ready." \
            || echo "Warning: Timed out waiting for CNPG pods readiness."
    }
}