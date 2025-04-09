misc@webhook-handler() { 
    cargo build 
    cargo run 
    cargo build --release
}

# IMPORTANT! used by release.yaml workflow
build_container#package_hook@webhook-handler() { 
    # NOTE: uses buildx (instead of the legacy build)
    docker build . -t webhook-handler:latest

    verify() {
        # check if build works with current Cargo.toml options
        docker run -it -v $(pwd):/app -w /app rust /bin/bash
    }
}

run_container@webhook-handler() {
    docker run -d -p 80:3010 webhook-handler
}
