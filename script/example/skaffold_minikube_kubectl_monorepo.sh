verify#example@monorepo() {
    ### generate combined configuration
    kubectl kustomize ./service/cilium-gateway/k8s/development > ./tmp/combined_manifest.yaml
    cat ./tmp/combined_manifest.yaml | kubectl apply -f -

    # replace variables and deploy with kustomize
    export $(cat .env | xargs) && kustomize build . | envsubst | kubectl apply -f -

    kubectl kustomize ./
    kubectl get -k ./
    kubectl --kubeconfig $kubeconfig  get -k ./
    kubectl describe -k ./
    kubectl diff -k ./

    kubectl get nodes --show-labels

    # cert-manager related 
    # two issuers: staging & production issuers 
    # ephemeral challenge appearing during certificate issuance process 
    # certificate should be READY = True
    # order: should be STATE = pending → STATE = valid
    kubectl get clusterissuer,certificate,order,challenge -A 
    kubectl get gateway,httproute,crds,securitypolicy -A 
    kubectl describe gateway -n gateway

    # check dns + web server response with tls staging certificate
    domain_name="donation-app.local"
    curl -i http://$domain_name
    curl --insecure -I https://$domain_name
    cloud_load_balancer_ip=""
    curl -i --header "Host: donation-app.local" $cloud_load_balancer_ip
    kubectl logs -n kube-system deployments/cilium-operator | grep gateway

    # run ephemeral debug container
    kubectl run -it --rm --image=nicolaka/netshoot debug-pod --namespace some_namespace -- /bin/bash 
    kubectl run -it --rm --image=busybox debug-pod-2 --namespace auth -- /bin/bash nslookup oathkeeper-proxy
    
    kubectl -n kube-system edit configmap cilium-config
}

skaffold_scripts#example@monorepo() {
    kubectl config view
    skaffold config list
    TEMP_FILE=$(mktemp -t skaffold_render_XXXXXX.log) && skaffold render --profile dev-watch > "$TEMP_FILE" 2>&1 && echo "Skaffold render output saved to: $TEMP_FILE"
    TEMP_FILE=$(mktemp -t skaffold_diagnose_XXXXXX.log) && skaffold diagnose > "$TEMP_FILE" 2>&1 && echo "Skaffold diagnose output saved to: $TEMP_FILE"
    skaffold inspect profile list | jq 
    skaffold diagnose --module scaffold-generic --profile staging-rebuild | grep -C 10 scaffold-k8s
    skaffold render --module scaffold-generic --profile staging-rebuild

    skaffold delete --profile dev-watch
    skaffold build -v debug 
    skaffold dev --tail --profile dev-watch --module monorepo --port-forward --auto-build=false --auto-deploy=false --cleanup=false 
    skaffold dev --port-forward -v debug
    skaffold debug
    skaffold run
}

minikube_scripts#example@monorepo() {
    kubectl config view && kubectl get namespace && kubectl config get-contexts

    (cd k8s/development && kubectl apply -k .)
    kubectl get all

    minikube ip 
    # expose service to host: 
    minikube tunnel # expose all possible resources (e.g. loadbalancers)
    minikube service dev-web-server --url --namespace=donation-app

    nslookup donation-app.local $(minikube ip) # query dns server running in minikube cluaster
    dig donation-app.local
    export GW=$(minikube ip) # or direct gateway ip exposed using minikube tunnel.
    curl --resolve donation-app.local:80:$GW donation-app.local
    ping donation-app.local

    # using ingress 
    kubectl describe ingress ingress -n donation-app

    # using gateway 
    {
        export GW=$(minikube ip) # or direct gateway ip exposed using minikube tunnel.
        kubectl apply -k ./service/cilium-gateway/k8s/development
        minikube tunnel # otherwise, with ingress-dns and ingress.yaml re-route to gateway will make accessing gateway through domain resolution directly with minikube ip
        minikube dashboard
        kubectl describe gateway -n donation-app
        kubectl describe httproute -n donation-app
        dig donation
        curl --resolve donation-app.local:80:$GW donation-app.local
    }

    kubectl apply -k ./kubernetes/overlays/dev

    curl -i --header "Host: donation-app.local" "<ip-of-load-balancer>"
}
