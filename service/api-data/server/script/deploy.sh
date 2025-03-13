misc_api_data() { 
    cargo build 
    cargo run 
    cargo build --release
}

# IMPORTANT! used by release.yml workflow
build_container_api_data() {
    if [ "$1" == "development" ]; then
        docker build . --target development -t api-data:latest # --build-arg ENV=development
    else
        docker build . --target production -t api-data:latest # --build-arg ENV=production
    fi
}

run_docker_api_data() {
    docker run -d -p 80:80 api-data
}
