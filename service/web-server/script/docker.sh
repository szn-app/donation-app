# IMPORTANT! used by release.yaml workflow
production.build_container#package_hook@web-server() {
    # NOTE: uses buildx (instead of the legacy build)
    docker build . --target release -t web-server:latest # --build-arg ENV=production
}

build_container#package_hook@web-server() {
    docker build . --target debug -t web-server:latest # --build-arg ENV=development
}

run_container@web-server() {
    docker run -d -p 80:80 web-server
}
