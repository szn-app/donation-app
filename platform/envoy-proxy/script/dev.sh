diagnose.skaffold@envoy-proxy() { 
    skaffold diagnose --module envoy-proxy --profile prod
    skaffold render --module envoy-proxy --profile prod | grep -C 10 envoy-proxy
    kubectl kustomize ./k8s/overlays/prod
}