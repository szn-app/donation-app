#!/bin/bash
set -e  # Exit immediately if a command exits with a non-zero status

func#predeploy_hook@ory-hydra() {
    local environment=${1:-dev}

    # Check if Hydra service exists in the auth namespace
    if kubectl get service hydra-admin -n auth &>/dev/null; then
        echo "Hydra service already exists, skipping installation"
    else
        source script/install.sh && install@hydra $environment
    fi
}

func#postdeploy_hook@ory-hydra() {
    local environment=${1:-dev}
}

