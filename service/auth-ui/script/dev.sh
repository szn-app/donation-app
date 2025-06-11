#!/bin/bash

skaffold@auth-ui() {   
    skaffold dev --module auth-ui --profile dev-watch --port-forward --auto-build=true --auto-deploy=true --cleanup=false --namespace=auth
    skaffold run --module auth-ui --profile dev-rebuild --port-forward --tail --namespace=auth
    skaffold build --module auth-ui --profile staging-rebuild --namespace=auth
    skaffold delete --module auth-ui --profile staging-prebuilt --namespace=auth
}

diagnose.skaffold@auth-ui() {
    skaffold diagnose --module auth-ui --profile dev-watch
    skaffold diagnose --module auth-ui --profile dev-rebuild
    skaffold diagnose --module auth-ui --profile staging-rebuild
    skaffold diagnose --module auth-ui --profile staging-prebuilt
    skaffold diagnose --module auth-ui --profile prod

    skaffold render --module auth-ui --profile staging-rebuild | grep -C 10 auth-ui

    kubectl kustomize ./k8s/overlays/staging
}