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

OBSOLETE_symlink_shared_components#setup#symlink@api-data() {(
    pushd "$(realpath "$(dirname "$(dirname "${BASH_SOURCE[0]}")")")"

    TARGET_DIR="../../../shared"
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

shared-mount-point#setup#mount-bind@api-data() {(
    pushd "$(realpath "$(dirname "$(dirname "${BASH_SOURCE[0]}")")")"

    local source_dir="../../../shared"
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

skaffold#task@api-data-server() {(
    pushd "$(dirname "$(dirname "${BASH_SOURCE[0]}")")" # two levels up: from script directory to project root

    skaffold dev --module api-data-server --profile development --port-forward --auto-build=false --auto-deploy=false --cleanup=false --tail

    no-dep() {
        skaffold dev --module api-data-server--no-dep --profile development --port-forward --auto-build=false --auto-deploy=false --cleanup=false --tail
    }

    verify() {
        skaffold render --profile production
        skaffold delete --profile development
        skaffold run --profile production --port-forward
    }

    popd
)}

delete.skaffold#task@api-data-server() {(
    pushd "$(dirname "$(dirname "${BASH_SOURCE[0]}")")"
    
    skaffold delete --profile development
    
    popd
)}

verify_grpc@api-data() { 
    grpcurl -plaintext -d '{"name": "World"}' localhost:8082 example.Greeter/SayHello
}

cargo@api-data() { 
    cargo test -q test_main -- --nocapture
}