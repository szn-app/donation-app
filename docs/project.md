

## Monorepo file organization: 
- `/kubernetes`: Kubernetes manifests for orchestration-specific configuration files. 
  - following base + overlays Kustomize folder structure https://kubernetes.io/docs/tasks/manage-kubernetes-objects/kustomization/#bases-and-overlays
- `/script` 

### workspaces
- root project can be opened to access entire project files
- project can be openned as workspaces using the .code-workspace config file: allows developing and configuring plugins for sub-folder services as root workspaces. 

## Kubernetes HA for production
- 3 control plane nodes + 2 worker agent nodes should be the minimum