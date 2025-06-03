# [task update version] https://github.com/kubernetes-sigs/gateway-api
# https://docs.cilium.io/en/stable/network/servicemesh/gateway-api/gateway-api/
# NOTE: contents of script function is manually kept synchronized with init.tf.patch
install_gateway_api_crds() {
    echo cilium
    kubectl apply -f https://raw.githubusercontent.com/kubernetes-sigs/gateway-api/v1.3.0/config/crd/standard/gateway.networking.k8s.io_gatewayclasses.yaml
    kubectl apply -f https://raw.githubusercontent.com/kubernetes-sigs/gateway-api/v1.3.0/config/crd/standard/gateway.networking.k8s.io_gateways.yaml
    kubectl apply -f https://raw.githubusercontent.com/kubernetes-sigs/gateway-api/v1.3.0/config/crd/standard/gateway.networking.k8s.io_httproutes.yaml
    kubectl apply -f https://raw.githubusercontent.com/kubernetes-sigs/gateway-api/v1.3.0/config/crd/standard/gateway.networking.k8s.io_referencegrants.yaml
    kubectl apply -f https://raw.githubusercontent.com/kubernetes-sigs/gateway-api/v1.3.0/config/crd/standard/gateway.networking.k8s.io_grpcroutes.yaml
    kubectl apply -f https://raw.githubusercontent.com/kubernetes-sigs/gateway-api/v1.3.0/config/crd/experimental/gateway.networking.k8s.io_tlsroutes.yaml
    echo cert-manager crds
    kubectl apply -f https://github.com/kubernetes-sigs/gateway-api/releases/download/v1.3.0/standard-install.yaml
}
