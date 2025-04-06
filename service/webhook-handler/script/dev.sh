test@webhook-handler() {
    export RUST_LOG=debug
    cargo watch -q -c -w src/ -x run
    cargo watch -q -c -w tests/ -x "test -q test_main -- --nocapture" 
}

# NOTE: used for docker command
hot_reload@webhook-handler() {
    cargo watch -q -c -w src/ -x run
}

single_test@webhook-handler() {
    export RUST_LOG=debug

    cargo watch -q -c -w src/ -x run &
    sleep 1s
    cargo test -q test_main -- --nocapture 
    kill $(jobs -p)
}

bootstrap@webhook-handler() {
    cargo install cargo-binstall
    
    # cargo binstall cargo-watch
    cargo install cargo-watch --locked
}

skaffold@webhook-handler() {     
    skaffold dev --profile development --port-forward --auto-build=false --auto-deploy=false --cleanup=false
    
    skaffold run --profile production --port-forward
}
