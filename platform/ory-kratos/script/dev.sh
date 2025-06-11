#!/bin/bash
set -e  # Exit immediately if a command exits with a non-zero status

production.skaffold#task@ory-kratos() {
    pushd "$(dirname "$(dirname "${BASH_SOURCE[0]}")")"
    
    skaffold run --profile prod

    popd
}

delete.production.skaffold#task@ory-kratos() {
    pushd "$(dirname "$(dirname "${BASH_SOURCE[0]}")")"
    
    skaffold delete --profile prod
    helm uninstall kratos -n auth || echo "Failed to uninstall Kratos, may not exist"

    popd
}
