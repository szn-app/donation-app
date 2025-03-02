generate_secret_auth_ui() {
    local environment=$1
    local auth_ui_secret_file="./service/auth-ui/k8s/${environment}/secret.env"
    
    # generate secrets 
    if [ ! -f "$auth_ui_secret_file" ]; then
        t=$(mktemp) && cat <<EOF > "$t"
COOKIE_SECRET=$(LC_ALL=C tr -dc 'A-Za-z0-9' < /dev/urandom | head -c 32)
CSRF_COOKIE_NAME=$(shuf -n 1 /usr/share/dict/words | tr -d '\n')_csrf
CSRF_COOKIE_SECRET=$(LC_ALL=C tr -dc 'A-Za-z0-9' < /dev/urandom | head -c 32)
EOF

        mv $t $auth_ui_secret_file
        echo "generated secrets file: file://$auth_ui_secret_file" 
    fi
}
