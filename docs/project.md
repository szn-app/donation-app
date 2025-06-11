

## Monorepo file organization: 
- `/kubernetes`/`k8s`: Kubernetes manifests for orchestration-specific configuration files. 
  - following base + overlays Kustomize folder structure https://kubernetes.io/docs/tasks/manage-kubernetes-objects/kustomization/#bases-and-overlays
- `/script` 
  - naming convension for script function: <function>.<module>#<label>@<namespace>  These are used to filter functions such as those of specific module or namespace or label defined as hooks
- `platform` tools layered on top of infrastructure to support app services - shared kubernetes controllers, gateways, etc. Service register their resources with the platform resources. 
  - `scaffold` services supporting the app services
- `infrastructure` cloud resources, terraform. 

### workspaces
- root project can be opened to access entire project files
- project can be openned as workspaces using the .code-workspace config file: allows developing and configuring plugins for sub-folder services as root workspaces. 

## Kubernetes HA for production
- 3 control plane nodes + 2 worker agent nodes should be the minimum
- Each microservice is deployed within its own namespace.