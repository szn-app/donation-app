skaffold@auth-ui() {     
    skaffold dev --profile development --port-forward --auto-build=true --auto-deploy=true --cleanup=false
    
    skaffold run --profile production --port-forward --tail

    skaffold render --profile production | grep -C 10 auth-ui
}

generate_config@auth-ui() {(
    pushd "$(realpath "$(dirname "$(dirname "${BASH_SOURCE[0]}")")")"
    local environment=$1
    local environment_short=$(if [ "$environment" == "development" ]; then echo "dev"; else echo "prod"; fi)

    if [ "$environment" != "production" ]; then
        popd
        return
    fi

    local secret_file="./k8s/overlays/${environment_short}/.env.local"
    
    # generate secrets
    if [ ! -f "$secret_file" ]; then
        t=$(mktemp) && cat <<EOF > "$t"
COOKIE_SECRET=$(LC_ALL=C tr -dc 'A-Za-z0-9' < /dev/urandom | head -c 32)
CSRF_COOKIE_NAME=$(shuf -n 1 /usr/share/dict/words | tr -d '\n')_csrf
CSRF_COOKIE_SECRET=$(LC_ALL=C tr -dc 'A-Za-z0-9' < /dev/urandom | head -c 32)
EOF

        mv $t $secret_file
        echo "generated secrets file: file://$secret_file"
    else
        echo "secrets file already exists: file://$secret_file"
    fi

    popd
)}

func#predeploy-skaffold-hook@auth-ui() {
    local environment=${1:-development}

    generate_config@auth-ui $1
}

func#postdeploy-skaffold-hook@auth-ui() {
    local environment=${1:-development}
    echo "$environment"
}