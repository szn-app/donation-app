skaffold#task@kafka-ui() { 
    pushd "$(dirname "$(dirname "${BASH_SOURCE[0]}")")" # two levels up: from script directory to project root
    
    skaffold dev --profile development --port-forward --tail

    verify() {
        skaffold render --profile production
        skaffold delete --profile development
    }

    popd
}

render.skaffold#task@kafka-ui() {
    pushd "$(dirname "$(dirname "${BASH_SOURCE[0]}")")" 
    temp_file=$(mktemp) && skaffold render --profile development > "$temp_file" && echo "$temp_file"
    popd
}
