#!/bin/bash
set -e  # Exit immediately if a command exits with a non-zero status

func#predeploy_hook@ory-keto() {
    local environment=${1:-dev}
    
    # Check if Keto service exists in the auth namespace
    if kubectl get service keto-read -n auth &>/dev/null; then
        echo "Keto service already exists, skipping installation"
    else
        source script/install.sh && install@keto $environment
    fi
}

func#postdeploy_hook@ory-keto() {
    local environment=${1:-dev}
}

