generate_config@api-data-server() {(
    pushd "$(realpath  "$(dirname "${BASH_SOURCE[0]}")")"

    local filename='./.env.local'
    local secret_name='postgresql-credentials-user'
    local namespace='api-data'

    # Ensure secret exists
    if ! kubectl get secret $secret_name -n $namespace &>/dev/null; then
        echo "❌ Secret '$secret_name' not found in namespace '$namespace'"
        popd >/dev/null && popd >/dev/null
        return 1
    fi

    echo "✅ Secret '$secret_name' found."
    echo "ℹ️ Extracting and decoding PostgreSQL credentials from secret..."
    
    # Extract and decode values from secret
    local POSTGRESQL_USERNAME=$(kubectl get secret $secret_name -n $namespace -o jsonpath='{.data.username}' | base64 -d)
    local POSTGRESQL_PASSWORD=$(kubectl get secret $secret_name -n $namespace -o jsonpath='{.data.password}' | base64 -d)
    # local POSTGRESQL_USERNAME="${POSTGRESQL_USERNAME:-pguser_$(openssl rand -hex 4)}"
    # local POSTGRESQL_PASSWORD="${POSTGRESQL_PASSWORD:-$(openssl rand -base64 16 | head -c 16)}"

    mkdir -p $(dirname "$filename")
    cat <<EOF > $filename # overrides file
POSTGRESQL_USERNAME=$POSTGRESQL_USERNAME
POSTGRESQL_PASSWORD=$POSTGRESQL_PASSWORD
EOF
    
    popd
)}
