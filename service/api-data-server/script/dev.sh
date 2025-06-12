#!/bin/bash

# IMPORTANT: used in github workflow
test#ci-hook#workflow@api-data-server() {
    echo "Running tests..."
    cargo test -q server::connection::postgresql::tests::test_postgres_config_new
    TEST_STATUS=$?
}

test@api-data-server() { 
    cargo test -- --list
    cargo test -q test_main -- --nocapture
    cargo test -q tests::test_db_connection_and_query -- --nocapture
    cargo test --features run_dev_test -q tests::test_db_connection_and_query -- --nocapture
}

test_watch@api-data-server() {
    export RUST_LOG=debug
    cargo watch -q -c -w tests/ -x "test -q test_main -- --nocapture" 

    cargo watch -q -c -w src/ -x run
    cargo watch -q -c -w src/ -x "run --bin mocked_isolated_graphql"
}

single_test@api-data-server() {
    export RUST_LOG=debug

    cargo watch -q -c -w src/ -x run &
    sleep 1s
    cargo test -q test_main -- --nocapture 
    kill $(jobs -p)
}

run_specific_binary@api-data-server() { 
    cargo run # main.rs
    cargo run --bin mocked_isolated_graphql
}

check@api-data-server(){ 
    check_without_warnings() {
        RUSTFLAGS="-A warnings" cargo check
    }

    echo ''
}

force.generate_graphql_sdl_schema#task@api-data-server(){
    pushd "$(dirname "$(dirname "${BASH_SOURCE[0]}")")"

    rust-script --force ./script/generate_graphql_sdl_schema#task@api-data-server.script.rs

    popd
}

# NOTE: used for docker command
hot_reload@api-data-server() {
    cargo +nightly watch -q -c -w src/ -x run

    verify() {
        {
            # check nightly version
            rustup show
        }
    }
}

OBSOLETE_symlink_shared_components#setup#symlink@api-data-server() {(
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

# IMPORTANT: used in github workflow (in addition to local monorepo setup)
shared-mount-point#setup#mount-bind@api-data-server() {(
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

skaffold#task@api-data-server() {(
    pushd "$(dirname "$(dirname "${BASH_SOURCE[0]}")")" # two levels up: from script directory to project root

    skaffold dev --module api-data-server --profile dev-watch --port-forward --auto-build=false --auto-deploy=false --cleanup=false --tail

    no-dep() {
        skaffold dev --module api-data-server--no-dep --profile dev-watch --port-forward --auto-build=false --auto-deploy=false --cleanup=false --tail
    }

    verify() {
        skaffold render --profile prod
        skaffold delete --profile dev-watch
        skaffold run --profile prod --port-forward
    }

    popd
)}

delete.skaffold#task@api-data-server() {(
    pushd "$(dirname "$(dirname "${BASH_SOURCE[0]}")")"
    
    skaffold delete --profile dev-watch
    
    popd
)}

verify_grpc@api-data-server() { 
    grpcurl -plaintext -d '{"name": "World"}' localhost:8082 example.Greeter/SayHello
}

cargo@api-data-server() { 
    cargo test -q test_main -- --nocapture
}

graphql_curl@api-data-server() {
    schema_introspect() { 
        curl -X POST http://localhost:8081/graphql \
            -H "Content-Type: application/json" \
            -d '{"query":"{ __schema { types { name } } }"}'
    }

    query_example() {
        URL="http://localhost:8081/graphql"

        read -r -d '' QUERY <<EOF
{
  "query": "query { dummyTest }"
}
EOF

        curl -X POST "$URL" -H "Content-Type: application/json" -d "$QUERY"
    }

    query_header_example() {
        URL="http://localhost:8081/graphql"

        read -r -d '' QUERY <<EOF
{
  "query": "query { dummyTestRequestHeader }"
}
EOF

        curl -i -k -X POST "$URL" -H "Content-Type: application/json" -d "$QUERY" -H "app-user-id: anonymous"
    }

    query_secure_example() {
        URL="http://localhost:8081/graphql"

        read -r -d '' QUERY <<EOF
{
  "query": "query { dummyTestSecure { s } }"
}
EOF

        # request should be denied with anonymous user
        curl -X POST "$URL" -H "Content-Type: application/json" -H "app-user-id: anonymous" -d "$QUERY"

        read -r -d '' QUERY <<EOF
{
  "query": "query { dummyTestSecurePermissionCheck { s } }"
}
EOF

        # request should be denied with anonymous user
        curl -X POST "$URL" -H "Content-Type: application/json" -H "app-user-id: anonymous" -d "$QUERY"


        read -r -d '' QUERY <<EOF
{
  "query": "query { dummyTestSecureGuard { s } }"
}
EOF

        # request should be denied with anonymous user
        curl -X POST "$URL" -H "Content-Type: application/json" -H "app-user-id: anonymous" -d "$QUERY"

    }

    verify_CORS_headers_returned() {
        URL="http://localhost:8081/graphql"
        curl -i -X OPTIONS "${URL}/not-found" -H "app-user-id: anonymous"

        # verify the request through the gateway does indeed return the same access-control-allow-origin header
        URL="https://api.donation-app.local/graphql" # endpoints relating to graphql rounter which is nested under /graphql 
        curl -i -k -X OPTIONS "${URL}/any-not-found-page" -H "app-user-id: anonymous"
        curl -k -i -X OPTIONS "${URL}/graphql" -H "Origin: https://donation-app.local"   -H "Access-Control-Request-Method: POST"
    }
}


misc@api-data-server() {
    check_if_grpc_services_listening() { 
        netstat -tulnp | grep 446 # 4466 and 4467
        grpcurl -plaintext localhost:4467 list
    }


    display_port_forward (){
        ps aux | grep 'kubectl port-forward'
    }
}



