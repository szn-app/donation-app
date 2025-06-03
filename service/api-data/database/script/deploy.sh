generate_database_credentials#predeploy-hook@api-data-database() {( # use subshell to avoid change variables
    pushd "$(realpath "$(dirname "$(dirname "${BASH_SOURCE[0]}")")")"

    local filename="./k8s/overlays/prod/user.env.local"

    if [ ! -f "$filename" ]; then
        t=$(mktemp) && cat <<EOF > "$t"
# must be same as username for database defined in cluster yaml config
username=postgres-user
# username="${POSTGRESQL_USERNAME:-pguser_$(openssl rand -hex 4)}"
password=$(openssl rand -base64 32 | tr -dc 'A-Za-z0-9')
EOF

        mv $t $filename
        echo "generated secrets file: file://$(readlink -f $filename)"
    else
        echo "Secrets file file://$(readlink -f $filename) already exists."
    fi

    popd
)}

func#predeploy-skaffold-hook@api-data-database() {
    local environment=$1

    ./script.sh sqlparser-validate-syntax#task@api-data-database.script.rs -s
    {
        exit_code=$?
        if [ $exit_code -eq 0 ]; then
            echo "SQL syntax is valid for $sql_migration_file"
        else
            echo "Error: Invalid SQL syntax or issue with $sql_migration_file"
            return 1
        fi
    }

    # if [ "$environment" != "development" ]; then
    #     generate_database_credentials@api-data-database
    # fi
}

func#postdeploy-skaffold-hook@api-data-database() {
    local environment=$1

    {
        # wait till deployment stabilizes as it is controlled by an operator
        local cluster_name="api-data--cluster-db"
        local namespace="api-data"
        
        kubectl wait --for=condition=Ready --timeout=120s Cluster/$cluster_name -n $namespace \
            && echo "Cluster is ready." \
            || echo "Warning: Timed out waiting for cluster readiness."

        kubectl wait --for=condition=Ready pods -l cnpg.io/cluster=$cluster_name -n $namespace --timeout=120s \
            && echo "All CNPG pods are ready." \
            || echo "Warning: Timed out waiting for CNPG pods readiness."
    }
}

# IMPORTANT! used in github workflow
build_container#package_hook@api-data-database() {(
    pushd "$(realpath "$(dirname "$(dirname "${BASH_SOURCE[0]}")")")"

    docker build . --target final -t api-data-database:latest

    popd
)}