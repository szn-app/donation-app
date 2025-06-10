generate_config@auth-ui() {(
    pushd "$(realpath "$(dirname "${BASH_SOURCE[0]}")")"

    local secret_file="./.env.local"
    
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
