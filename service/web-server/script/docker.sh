# IMPORTANT! used by release.yml workflow
build_container_web_server() {
    # NOTE: uses buildx (instead of the legacy build)
    if [ "$1" == "development" ]; then
        docker build . --target development -t web-server:latest # --build-arg ENV=development
    else
        docker build . --target production -t web-server:latest # --build-arg ENV=production
    fi
}

run_web_server() {
    docker run -d -p 80:80 web-server
}
