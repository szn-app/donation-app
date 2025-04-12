test@webhook-handler() {
    export RUST_LOG=debug
    cargo watch -q -c -w src/ -x run
    cargo watch -q -c -w tests/ -x "test -q test_main -- --nocapture" 
}

# NOTE: used for docker command
hot_reload@webhook-handler() {
    cargo watch -q -c -w src/ -x run
}

# NOTE: used in github workflows
test#ci-hook#workflow@webhook-handler() {
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

OBSOLETE_symlink_shared_components#setup#symlink@webhook-handler() {(
    pushd "$(realpath "$(dirname "$(dirname "${BASH_SOURCE[0]}")")")"

    TARGET_DIR="../../shared"
    LINK_NAME="shared"

    if [ -d "$TARGET_DIR" ]; then
    if [ ! -L "$LINK_NAME" ]; then
        echo "Creating symlink '$LINK_NAME' pointing to '$TARGET_DIR'"
        ln -s "$TARGET_DIR" "$LINK_NAME"
    else
        echo "Symlink '$LINK_NAME' already exists."
    fi
    else
    echo "Target directory '$TARGET_DIR' does not exist."
    fi

    delete() { 
        unlink $LINK_NAME
    }

    popd   
)}

shared-mount-point#setup#mount-bind@webhook-handler() {(
    pushd "$(realpath "$(dirname "$(dirname "${BASH_SOURCE[0]}")")")"

    local source_dir="../../shared"
    local target="shared" # target mount point

    if mountpoint -q "$target"; then
        echo "Already mounted at $target"
        findmnt $target
    else
        mkdir -p "$target"
        sudo mount --bind "$source_dir" "$target"
        
        echo "Bind mounted $source_dir at $target"
    fi

    delete() {
        sudo umount "$target"
    }

    popd
)}

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

skaffold#task@webhook-handler() {
    skaffold dev --module webhook-handler-generic --profile development --port-forward --auto-build=false --auto-deploy=false --cleanup=false
}

delete.skaffold#task@webhook-handler() {
    skaffold delete --module webhook-handler-generic --profile development
}
