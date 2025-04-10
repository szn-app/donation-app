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

skaffold@api-data() {     
    skaffold dev --profile development --port-forward --auto-build=false --auto-deploy=false --cleanup=false --tail
    
    skaffold run --profile production --port-forward
}

verify_grpc@api-data() { 
    grpcurl -plaintext -d '{"name": "World"}' localhost:8082 example.Greeter/SayHello
}