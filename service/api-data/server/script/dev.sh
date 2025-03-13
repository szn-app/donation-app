test() {
    export RUST_LOG=debug
    cargo watch -q -c -w src/ -x run
    cargo watch -q -c -w tests/ -x "test -q test_main -- --nocapture" 
}

single_test() {
    export RUST_LOG=debug

    cargo watch -q -c -w src/ -x run &
    sleep 1s
    cargo test -q test_main -- --nocapture 
    kill $(jobs -p)
}

bootstrap() { 
    cargo install cargo-binstall
    
    # cargo binstall cargo-watch
    cargo install cargo-watch --locked
}

# NOTE: used for docker command
hot_reload_api_data() {
    cargo +nightly watch -q -c -w src/ -x run

    verify() {
        {
            # check nightly version
            rustup show
        }
    }
}

dev_api_data_skaffold() {     
    skaffold dev --profile development --port-forward --auto-build=false --auto-deploy=false --cleanup=false --tail
    
    skaffold run --profile production --port-forward
}
