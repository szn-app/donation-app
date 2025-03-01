misc_api_data() { 
    cargo build 
    cargo run 
    cargo build --release
}

# IMPORTANT! used by release.yml workflow
build_container_api_data() {
    docker build . -t api-data:latest
}

run_docker_api_data() {
    docker run -d -p 80:3000 api-data
}
