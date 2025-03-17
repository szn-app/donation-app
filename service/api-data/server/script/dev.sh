test@api-data() {
    export RUST_LOG=debug
    cargo watch -q -c -w src/ -x run
    cargo watch -q -c -w tests/ -x "test -q test_main -- --nocapture" 
}

single_test@api-data() {
    export RUST_LOG=debug

    cargo watch -q -c -w src/ -x run &
    sleep 1s
    cargo test -q test_main -- --nocapture 
    kill $(jobs -p)
}

bootstrap@api-data() { 
    cargo install cargo-binstall
    
    # cargo binstall cargo-watch
    cargo install cargo-watch --locked
}

# NOTE: used for docker command
hot_reload@api-data() {
    cargo +nightly watch -q -c -w src/ -x run

    verify() {
        {
            # check nightly version
            rustup show
        }
    }
}

dev.skaffold@api-data() {     
    skaffold dev --profile development --port-forward --auto-build=false --auto-deploy=false --cleanup=false --tail
    
    skaffold run --profile production --port-forward
}
