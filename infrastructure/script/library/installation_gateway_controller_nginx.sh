# Gateway controller instlalation - https://gateway-api.sigs.k8s.io/implementations/ & https://docs.nginx.com/nginx-gateway-fabric/installation/ 
# This controller could be used in place of the Cilium Gateway API controller 
installation_gateway_controller_nginx() {
  # Gateway API CRD installation - https://gateway-api.sigs.k8s.io/guides/#installing-a-gateway-controller
  kubectl apply -f https://github.com/kubernetes-sigs/gateway-api/releases/download/v1.3.0/standard-install.yaml   

  # Gateway controller instlalation - https://gateway-api.sigs.k8s.io/implementations/ & https://docs.nginx.com/nginx-gateway-fabric/installation/ 
  kubectl apply -f https://raw.githubusercontent.com/nginxinc/nginx-gateway-fabric/v1.6.2/deploy/crds.yaml
  kubectl apply -f https://raw.githubusercontent.com/nginxinc/nginx-gateway-fabric/v1.6.2/deploy/default/deploy.yaml
  kubectl get pods -n nginx-gateway
}


delete.installation_gateway_controller_nginx() { 
  kubectl delete -f https://github.com/kubernetes-sigs/gateway-api/releases/download/v1.3.0/standard-install.yaml   

  kubectl delete -f https://raw.githubusercontent.com/nginxinc/nginx-gateway-fabric/v1.6.2/deploy/crds.yaml
  kubectl delete -f https://raw.githubusercontent.com/nginxinc/nginx-gateway-fabric/v1.6.2/deploy/default/deploy.yaml
}