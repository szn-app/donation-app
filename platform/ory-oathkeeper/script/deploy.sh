#!/bin/bash
set -e  # Exit immediately if a command exits with a non-zero status

func#predeploy_hook@ory-oathkeeper() {
    local environment=${1:-dev}
    
    # Check if Oathkeeper service exists in the auth namespace
    if kubectl get service oathkeeper-api -n auth &>/dev/null; then
        echo "Oathkeeper service already exists, skipping installation"
    else
        source script/install.sh && install@oathkeeper $environment
    fi
}

func#postdeploy_hook@ory-oathkeeper() {
    local environment=${1:-dev}
}

