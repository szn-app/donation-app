#!/bin/bash

diagnose.skaffold@scaffold() {
    skaffold diagnose --module scaffold --profile prod

    skaffold render --module scaffold --profile staging-rebuild | grep -C 10 scaffold

    kubectl kustomize ./k8s/overlays/staging
}