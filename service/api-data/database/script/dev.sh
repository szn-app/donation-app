#!/bin/bash
set -e

skaffold#task@api-data-database() { 
    pushd "$(dirname "$(dirname "${BASH_SOURCE[0]}")")" # two levels up: from script directory to project root
    
    skaffold dev --profile development --port-forward --tail

    popd
}

render.skaffold#task@api-data-database() {
    pushd "$(dirname "$(dirname "${BASH_SOURCE[0]}")")" 
    temp_file=$(mktemp) && skaffold render --profile development > "$temp_file" && echo "$temp_file"
    popd
}

delete.skaffold#task@api-data-database() {(
    pushd "$(dirname "$(dirname "${BASH_SOURCE[0]}")")" 

    skaffold delete --profile development

    popd
)}

cluster#benchmark@api-data-database() {
    # TODO: benchmark CNPG cluster
    echo ''
}

bootstrap@api-data-database() { 
    echo ''
}