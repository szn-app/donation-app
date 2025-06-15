#!/bin/bash
# set -e

setup_minikube() {
    wait_for_terminating_resources.kubernetes#utility
    setup.minikube#bootstrap#task@monorepo
}

expose_domain() {
    start.tunnel.minikube#task@monorepo -v
}

### 

dev-rebuild.run.skaffold#task@monorepo() {
    setup_minikube
    skaffold run --profile dev-rebuild --module monorepo
}

prod.run.skaffold#task@monorepo() {
    setup_minikube
    skaffold run --profile prod --module monorepo
}

delete.dev-rebuild.run.skaffold#task@monorepo() {
    skaffold delete --profile dev-rebuild --module monorepo
    setup_minikube
}

dev-watch.skaffold#task@monorepo() {    
    setup_minikube
    skaffold dev --profile dev-watch --module monorepo --port-forward --auto-build=false --auto-deploy=false --cleanup=false --tail
}

staging-rebuild.skaffold#task@monorepo() {
    setup_minikube
    skaffold run --profile staging-rebuild  --module monorepo # --port-forward --tail
}

delete.staging-rebuild.skaffold#task@monorepo() {
    skaffold delete --profile staging-rebuild
}

service-only.staging.skaffold#task@monorepo() {
    setup_minikube
    skaffold run --profile staging-rebuild --module monorepo-service-only --port-forward --cleanup=false
}

platform-only.staging.skaffold#task@monorepo() {
    setup_minikube
    skaffold run --profile staging-rebuild --module monorepo-platform-only --port-forward --cleanup=false
}

force_delete_all.skaffold#task@monorepo() {
    skaffold delete --profile dev-watch
    skaffold delete --profile dev-rebuild
    skaffold delete --profile staging-rebuild
    skaffold delete --profile staging-prebuilt
    skaffold delete --profile prod

    execute.util '#task' '#manual-delete'
    execute.util '#task' '#pvc-manual-delete'
}

production.skaffold#hetzner#task@monorepo() {
    kubectl ctx k3s

    setup_repo_files@monorepo
    wait_for_terminating_resources.kubernetes#utility
    skaffold run --profile prod --module monorepo
}

force_delete.production.skaffold#task@monorepo() {
    skaffold delete --profile prod --module monorepo

    execute.util '#task' '#manual-delete'
}

