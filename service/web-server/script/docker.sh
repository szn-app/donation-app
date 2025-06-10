# IMPORTANT! used by release.yaml workflow
build_container#package_hook@web-server() {
    # NOTE: uses buildx (instead of the legacy build)
    if [ "$1" == "development" ]; then
        docker build . --target debug -t web-server:latest # --build-arg ENV=development
    else
        docker build . --target release -t web-server:latest # --build-arg ENV=production
    fi
}

run_container@web-server() {
    docker run -d -p 80:80 web-server
}
