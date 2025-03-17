generate_db_credential@keto() {
    pushd "$(dirname "${BASH_SOURCE[0]}")"

    db_secret_file="./config/db_keto_secret.env"
    if [ ! -f "$db_secret_file" ]; then
        t=$(mktemp) && cat <<EOF > "$t"
DB_USER="$(shuf -n 1 /usr/share/dict/words | tr -d '\n')"
DB_PASSWORD="$(openssl rand -base64 32 | tr -dc 'A-Za-z0-9')"
EOF

        mv $t $db_secret_file
        echo "generated secrets file: file://$(readlink -f $db_secret_file)" 
    else
        echo "using existing credentials file: file://$(readlink -f $db_secret_file)"
    fi

    popd
}

create_policies@keto() {
        local environment=$1

        # https://www.ory.sh/docs/keto#ory-permission-language
        printf "Keto: create relations rules \n"
        
        # [depends on] kratos secret for admin user
        admin_username=$(kubectl get secret default-admin-user-credentials -n auth -o jsonpath='{.data.username}' | base64 -d)
        admin_uuid=$(kubectl get secret default-admin-user-credentials -n auth -o jsonpath='{.data.uuid}' | base64 -d)

        if [ -z "$admin_username" ] || [ -z "$admin_uuid" ]; then
            echo "Error: admin_username or admin_uuid is not set"
        fi

        if [ "$environment" = "production" ]; then
            local sleep_time=60
        else 
            local sleep_time=600000
        fi
        kubectl run --image=debian:latest setup-pod-keto --namespace auth -- /bin/bash -c "while true; do sleep $sleep_time; done"
        kubectl wait --for=condition=ready pod/setup-pod-keto --namespace=auth --timeout=300s

        {
            t="$(mktemp).sh" && cat << 'EOF' > $t
#!/bin/bash

# install keto cli
apt update >/dev/null 2>&1 && apt install curl jq -y >/dev/null 2>&1
bash <(curl https://raw.githubusercontent.com/ory/meta/master/install.sh) -d -b . keto v0.12.0-alpha.0 >/dev/null 2>&1 && chmod +x ./keto && mv ./keto /usr/bin/


verify() {
    alias keto='keto --read-remote keto-read:80 --write-remote keto-write:80'
    alias keto-insecure='keto --insecure-disable-transport-security --insecure-skip-hostname-verification'

    curl -i -X GET http://keto-read:80/health/alive
    curl -i -X GET http://keto-read:80/namespaces

    keto-insecure status 
    keto-insecure relation-tuple get
}
EOF
            kubectl cp $t setup-pod-keto:$t --namespace auth
            kubectl exec setup-pod-keto --namespace auth -- /bin/bash -c "chmod +x $t && $t" >/dev/null 2>&1
        }
        {
            kubectl cp ./config/policies.rts setup-pod-keto:policies.rts --namespace auth
            
            printf "admin_uuid = %s  admin_username = %s \n" "$admin_uuid" "$admin_username"

            p="$(mktemp).rts" && cat << EOF > $p
Group:admin#member@User:$admin_uuid
EOF
            kubectl cp $p setup-pod-keto:admin-user-policies.rts --namespace auth

            t="$(mktemp).sh" && cat << EOF > $t
#!/bin/bash

# relationship-rules
# https://www.ory.sh/docs/keto/concepts/relation-tuples
# https://www.ory.sh/docs/keto/quickstart
# https://www.ory.sh/docs/keto/reference/rest-api#tag/relationship
# https://www.ory.sh/docs/keto/cli/keto-relation-tuple-create

# objects -relationship-> subjects
# translated to Subject is in some-relationship with/of Object

# set remote Keto and security settings
# TODO: fix alias inside script (works in shell directly copy-paste but not when automated)
# alias keto-managed='keto --read-remote keto-read:80 --write-remote keto-write:80'
# alias keto-insecure='keto-managed --insecure-disable-transport-security --insecure-skip-hostname-verification'

# TODO: important to solve issue of policies added false-positively (prints that relations created but not actually created)
sleep 15

# apply relation-tuple policies
echo "Creating Keto policy tuples"
keto relation-tuple parse admin-user-policies.rts policies.rts --format json | keto relation-tuple create --read-remote keto-read:80 --write-remote keto-write:80 --insecure-disable-transport-security - && echo "Successfully Keto created tuple" || echo "Error Keto creating admin tuple"

example() {
    keto relation-tuple get --namespace="Group" --relation="member" \
        --read-remote keto-read:80 --write-remote keto-write:80 --insecure-disable-transport-security --insecure-skip-hostname-verification --format json | jq

    keto relation-tuple get --namespace="Endpoint" --subject-set="Group:admin#member" \
        --read-remote keto-read:80 --write-remote keto-write:80 --insecure-disable-transport-security --insecure-skip-hostname-verification --format json | jq

    keto expand member Group admin \
        --read-remote keto-read:80 --write-remote keto-write:80 --insecure-disable-transport-security --insecure-skip-hostname-verification  --format yaml

    keto expand access Endpoint k8s \
        --read-remote keto-read:80 --write-remote keto-write:80 --insecure-disable-transport-security --insecure-skip-hostname-verification --format yaml

    curl -k -X POST http://keto-read:80/relation-tuples/check -H "Content-Type: application/json" -d \
        '{ "namespace": "Endpoint", "object": "test", "relation": "access", "subject_set": { "namespace": "Group", "object": "admin", "relation": "" } }'
    keto check Group:admin access Endpoint test \
        --read-remote keto-read:80 --write-remote keto-write:80 --insecure-disable-transport-security --insecure-skip-hostname-verification --format json | jq
    keto check User:$admin_uuid access Endpoint test \
        --read-remote keto-read:80 --write-remote keto-write:80 --insecure-disable-transport-security --insecure-skip-hostname-verification --format json | jq
        
    keto check User:b@b.com access Endpoint k8s \
        --read-remote keto-read:80 --write-remote keto-write:80 --insecure-disable-transport-security --insecure-skip-hostname-verification --format json | jq
}
EOF
            kubectl cp $t setup-pod-keto:$t --namespace auth
            kubectl exec setup-pod-keto --namespace auth -- /bin/bash -c "chmod +x $t && $t"
        }

        # if [ "$environment" = "production" ]; then
        kubectl delete --force pod setup-pod-keto -n auth > /dev/null 2>&1 || true
        # fi
}

# create .env files from default template if doesn't exist
create_env_files@keto() {
    # Find all *.env.template files
    find ./config -name "*.env.template" | while IFS= read -r template_file; do
            # Extract filename without extension
            filename=$(basename "$template_file" | cut -d '.' -f 1)
            env_file="$(dirname "$template_file")/$filename.env"

            # Check if .env file already exists
            if [ ! -f "$env_file" ]; then
                # Create a new .env file from the template in the same directory
                cp "$template_file" "$env_file" 
                echo "created env file file://$(readlink -f $env_file) from $template_file"
            else
                echo "env file already exists: file://$(readlink -f $env_file)"
            fi
    done
}

install@keto() {
    set -e
    local environment="$1"
    
    # ory stack charts
    helm repo add ory https://k8s.ory.sh/helm/charts > /dev/null 2>&1
    # postgreSQL
    helm repo add bitnami https://charts.bitnami.com/bitnami > /dev/null 2>&1 
    helm repo update > /dev/null 2>&1 

    generate_db_credential@keto
    create_env_files@keto

    if helm list -n auth | grep -q 'postgres-keto' && [ "$environment" = "development" ]; then
        upgrade_db=false
    else
        upgrade_db=true
    fi

    if [ "$upgrade_db" = true ]; then
        printf "install Postgresql for Ory Keto \n"

        set -a
            source ./config/db_keto_secret.env # DB_USER, DB_PASSWORD
        set +a
        l="$(mktemp).log" && helm upgrade --debug --reuse-values --install postgres-keto bitnami/postgresql -n auth --create-namespace -f ./postgresql-values.yml \
            --set auth.username=${DB_USER} \
            --set auth.password=${DB_PASSWORD} \
            --set auth.database=keto_db > $l 2>&1 && printf "Keto database logs: file://$l\n"
        # this will generate 'postgres-keto-postgresql' service
    fi

    printf "install Ory Keto \n"
    {
        # preprocess file through substituting env values
        t="$(mktemp).yml" && cargo run --release --bin render-template -- --environment $environment < ./keto-config.yaml.tera > $t && printf "generated manifest with replaced env variables: file://$t\n" 
        set -a
            source ./config/db_keto_secret.env # DB_USER, DB_PASSWORD
        set +a
        l="$(mktemp).log" && helm upgrade --debug --install --atomic keto ory/keto -n auth --create-namespace -f ./helm-values.yml -f $t \
            --set env[0].name=DB_USER --set env[0].value=${DB_USER} \
            --set env[1].name=DB_PASSWORD --set env[1].value=${DB_PASSWORD} > $l 2>&1 && printf "Keto logs: file://$l\n"
    }

    # Wait for Keto deployments to be ready
    printf "Waiting for Keto deployment to be ready...\n"
    kubectl wait --for=condition=available deployment/keto --namespace=auth --timeout=300s


    create_policies@keto $environment

    verify() {
        alias keto="docker run -it --network cat-videos-example_default -e KETO_READ_REMOTE=\"keto:4466\" oryd/keto:v0.7.0-alpha.1"

        http PUT http://keto.example.com/write/relation-tuples namespace=access object=administration relation=access subject_id=admin
        http PUT http://keto.example.com/write/relation-tuples namespace=access object=application relation=access subject_id=admin
        http PUT http://keto.example.com/write/relation-tuples namespace=access object=application relation=access subject_id=user

        # check
        http -b http://keto.example.com/read/check namespace=access object=administration relation=access subject_id=admin
        # {
        #     "allowed": true
        # }

        # check ongoing installs 
        helm history keto -n auth
    }

    set +e
}
