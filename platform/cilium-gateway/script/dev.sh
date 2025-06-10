diagnose.skaffold@cilium-gateway() { 
    skaffold diagnose --module cilium-gateway --profile prod
    
    kubectl kustomize ./k8s/overlays/prod
}
