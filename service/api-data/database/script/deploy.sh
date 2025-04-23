generate_database_credentials@api-data-database() {( # use subshell to avoid change variables
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

validate_sql_syntax#task@api-data-database() {(
    pushd "$(realpath "$(dirname "$(dirname "${BASH_SOURCE[0]}")")")"
    local sql_migration_file="./k8s/base/init.sql"

    if [ -z "$sql_migration_file" ]; then
        echo "Error: No SQL migration file provided."
        exit 1
    fi

    ./script/sqlparser-validate-syntax.script.rs -s -f $sql_migration_file
    {
        exit_code=$?
        if [ $exit_code -eq 0 ]; then
            echo "SQL syntax is valid for $sql_migration_file"
        else
            echo "Error: Invalid SQL syntax or issue with $sql_migration_file"
            return 1
        fi
    }
    popd
)}

func#predeploy-skaffold-hook@api-data-database() {
    local environment=$1

    validate_sql_syntax#task@api-data-database

    if [ "$environment" != "development" ]; then
        generate_database_credentials@api-data-database
    fi
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