dev_auth_ui_skaffold() {     
    skaffold dev --profile development --port-forward --auto-build=true --auto-deploy=true --cleanup=false
    
    skaffold run --profile production --port-forward --tail
}

prehook_auth_ui() {
    local environment=${1:-development}

    generate_secret() {
        local environment=$1
        
        local environment_short=$(if [ "$environment" == "development" ]; then echo "dev"; else echo "prod"; fi)
        local secret_file="./k8s/overlays/${environment_short}/secret.env"
        
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
    }

    generate_secret $1
}

posthook_auth_ui() {
    local environment=${1:-development}
    echo "$environment"
}