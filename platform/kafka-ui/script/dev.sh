skaffold#task@kafka-ui() { 
    pushd "$(dirname "$(dirname "${BASH_SOURCE[0]}")")" # two levels up: from script directory to project root
    
    skaffold dev --profile dev-watch --port-forward --tail

    verify() {
        skaffold render --profile prod
        skaffold delete --profile dev-watch
    }

    popd
}

render.skaffold#task@kafka-ui() {
    pushd "$(dirname "$(dirname "${BASH_SOURCE[0]}")")" 
    temp_file=$(mktemp) && skaffold render --profile dev-watch > "$temp_file" && echo "$temp_file"
    popd
}

diagnose.skaffold@kafka-ui() {
    skaffold diagnose --module kafka-ui --profile prod

    skaffold render --module kafka-ui --profile staging-rebuild | grep -C 10 kafka-ui

    kubectl kustomize ./k8s/overlays/staging
}