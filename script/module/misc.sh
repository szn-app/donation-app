misc_() {
    # modify permission
    find ./ -maxdepth 4 -name "script.sh" -exec chmod +x {} \;
}

record_version#bootstrap#task@monorepo() { 
    NODE_VERSION=$(node -v | cut -d 'v' -f2)
    PNPM_VERSION=$(pnpm --version | cut -d ' ' -f2)
    RUST_VERSION=$(rustc --version | awk '{print $2}') 
    CARGO_VERSION=$(cargo --version | awk '{print $2}')
    DOCKER_VERSION=$(docker version --format '{{.Server.Version}}')
    MINIKUBE_VERSION=$(/usr/bin/minikube version --short)  
    KUBERNETES_VERSION=$(kubectl version | awk '{printf "\t%s:\t%s\n", $1" "$2, $3}')
    KUSTOMIZE_VERSION=$(kustomize version)
    KOPS_VERSION=$(kops version --short)
    CILIUM_VERSION=$(cilium version --client)
    SKAFFOLD_VERSION=$(skaffold version)
    CONNTRACK_VERSION=$(conntrack -V | awk '{print $2}')
    POSTGRESQL_VERSION=$(psql --version | awk '{print $3}')

    echo "Node.js version: ${NODE_VERSION}" > version.txt
    echo "pnpm version: ${PNPM_VERSION}" >> version.txt
    echo "Rust version: ${RUST_VERSION}" >> version.txt
    echo "Cargo version: ${CARGO_VERSION}" >> version.txt
    echo "Docker version: ${DOCKER_VERSION}" >> version.txt
    echo "Minikube version: ${MINIKUBE_VERSION}" >> version.txt
    printf "Kubernetes version: \n%s\n" "$KUBERNETES_VERSION" >> version.txt
    echo "Kustomize version: ${KUSTOMIZE_VERSION}" >> version.txt
    echo "kOps version: ${KOPS_VERSION}" >> version.txt
    echo "---\nCilium version: ${CILIUM_VERSION}\n" >> version.txt
    echo "Skaffold version: ${SKAFFOLD_VERSION}" >> version.txt
    echo "Conntrack version: ${CONNTRACK_VERSION}" >> version.txt
    echo "PostgreSQL version: ${POSTGRESQL_VERSION}" >> version.txt
    yq --version >> version.txt
    mc --version >> version.txt

    cat ./version.txt
}

# Wait for terminating resources to complete
wait_for_terminating_resources() {
    echo "Checking for terminating resources..."
    while kubectl get pods --all-namespaces | grep -q Terminating || \
                kubectl get services --all-namespaces | grep -q Terminating || \
                kubectl get deployments --all-namespaces | grep -q Terminating || \
                kubectl get statefulsets --all-namespaces | grep -q Terminating; do
        echo "Waiting for resources to finish terminating..."
        sleep 2
    done
    echo "All terminating resources have been cleaned up"
}
