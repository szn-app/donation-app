generate_db_credentials#example@hydra() {(
    pushd "$(realpath "$(dirname "${BASH_SOURCE[0]}")")"

    db_secret_file="./config/db_hydra_secret.env"
    if [ ! -f "$db_secret_file" ]; then
        t=$(mktemp) && cat <<EOF > "$t"
DB_USER="$(shuf -n 1 /usr/share/dict/words | tr -d '\n')"
DB_PASSWORD="$(openssl rand -base64 32 | tr -dc 'A-Za-z0-9')"
EOF

        mv $t $db_secret_file
        echo "generated secrets file: file://$(readlink -f $db_secret_file)" 
    else
        echo "db secret file already exists: file://$(readlink -f $db_secret_file)"
    fi

    popd
)}

database.install#example@hydra() {( 
    set -e
    local environment=$1

    generate_db_credentials@hydra
    
    # postgreSQL
    helm repo add bitnami https://charts.bitnami.com/bitnami > /dev/null 2>&1 
    helm repo update > /dev/null 2>&1 

    if helm list -n auth | grep -q 'postgres-hydra' && [ "$environment" = "development" ]; then
        upgrade_db=false
    else
        upgrade_db=true
    fi

    if [ "$upgrade_db" = true ]; then
        printf "install Postgresql for Ory Hydra \n"

        set -a
            source ./config/db_hydra_secret.env # DB_USER, DB_PASSWORD
        set +a
        l="$(mktemp).log" && helm upgrade --debug --reuse-values --install postgres-hydra bitnami/postgresql -n auth --create-namespace -f ./postgresql-values.yml \
            --set auth.username=${DB_USER} \
            --set auth.password=${DB_PASSWORD} \
            --set auth.database=hydra_db > $l 2>&1 && printf "Hydra database logs: file://$l\n"
        # this will generate 'postgres-hydra-postgresql' service
    fi
)}
