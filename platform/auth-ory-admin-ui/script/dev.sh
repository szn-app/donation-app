#!/bin/bash

diagnose.skaffold@auth-ory-admin-ui() {
    skaffold diagnose --module auth-ory-admin-ui --profile prod

    skaffold render --module auth-ory-admin-ui --profile staging-rebuild | grep -C 10 auth-ory-admin-ui

    kubectl kustomize ./k8s/overlays/staging
}