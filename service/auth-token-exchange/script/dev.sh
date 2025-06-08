test@auth-token-exchange() { 
    export RUST_LOG=debug
    cargo watch -q -c -w src/ -x run
    cargo watch -q -c -w tests/ -x "test -q test_main -- --nocapture" 
}

# NOTE: used for docker command
hot_reload@auth-token-exchange() {
    cargo watch -q -c -w src/ -x run
}

# NOTE: used in github workflows
test#ci-hook#workflow@auth-token-exchange() {
    set -e

    echo "Starting the server..."
    cargo run --release &
    SERVER_PID=$!  # Capture the process ID of the server

    sleep 2 

    echo "Running tests..."
    cargo test -q test_main -- --nocapture 
    TEST_STATUS=$?

    echo "Shutting down the server..."
    kill $SERVER_PID
    # Wait for the server to terminate
    wait $SERVER_PID 2>/dev/null || true

    if [ $TEST_STATUS -eq 0 ]; then
        echo "Tests passed successfully!"
        exit 0
    else
        echo "Tests failed!"
        exit 1
    fi
}

single_test@auth-token-exchange() { 
    export RUST_LOG=debug

    HYDRA_CLIENT_SECRET="dummy" cargo watch -q -c -w src/ -x run &
    sleep 1s
    cargo test -q test_main -- --nocapture 
    kill $(jobs -p)
}

bootstrap@auth-token-exchange() { 
    cargo install cargo-binstall
    
    # cargo binstall cargo-watch
    cargo install cargo-watch --locked
}

skaffold@auth-token-exchange() {     
    skaffold dev --profile development --port-forward --auto-build=false --auto-deploy=false --cleanup=false
    
    skaffold run --profile production --port-forward
}

diagnose.skaffold@auth-token-exchange() {
    skaffold diagnose --module auth-token-exchange --profile prod

    skaffold render --module auth-token-exchange --profile staging-rebuild | grep -C 10 auth-token-exchange

    kubectl kustomize ./k8s/overlays/staging
}

misc@auth-token-exchange() { 
    cargo build 
    cargo run 
    cargo build --release
}

run_container@auth-token-exchange() {
    docker run -d -p 80:3000 auth-token-exchange
}
