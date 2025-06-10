#!/bin/bash
set -e  # Exit immediately if a command exits with a non-zero status

func#predeploy_hook@ory-kratos() {
    local environment=${1:-dev}
    
    # Check if Kratos service exists in the auth namespace
    if kubectl get service kratos-admin -n auth &>/dev/null; then
        echo "Kratos service already exists, skipping installation"
    else 
        source script/install.sh && install@kratos $environment
    fi
}

func#postdeploy_hook@ory-kratos() {
    local environment=${1:-dev}
}

