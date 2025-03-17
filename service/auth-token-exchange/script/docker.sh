misc@auth-token-exchange() { 
    cargo build 
    cargo run 
    cargo build --release
}

# IMPORTANT! used by release.yml workflow
build_container#package_hook@auth-token-exchange() { 
    # NOTE: uses buildx (instead of the legacy build)
    docker build . -t auth-token-exchange:latest
}

run_container@auth-token-exchange() {
    docker run -d -p 80:3000 auth-token-exchange
}
