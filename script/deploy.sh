#!/bin/bash 

{
    production_deploy() { 
        deploy --environment production --action install
    }
    development_deploy() { 
        deploy --environment development --action install
    }
}
# using kubectl, helm, kustomize
deploy() {
    local environment="development" # environment = development, production
    local action="install" # action = install, delete, kustomize, app

    while [[ "$#" -gt 0 ]]; do
        case $1 in
            --environment) environment="$2"; shift ;;
            --action) action="$2"; shift ;;
            *) echo "Unknown parameter passed: $1"; exit 1 ;;
        esac
        shift
    done

    if ! command -v kubectl-ctx &> /dev/null; then
        echo "kubectl ctx is not installed. Exiting."
        return
    fi

    # set kubectl context based on environment
    if [ "$environment" == "production" ]; then
        kubectl ctx k3s
    elif [ "$environment" == "development" ]; then
        kubectl ctx minikube
    else
        echo "Unknown environment: $environment"
        exit 1
    fi

    # create .env files from default template if doesn't exist
    env_files $environment

    {
        if [ "$action" == "delete" ]; then
            install_ory_stack --environment "$environment" --action delete 
            manual_deploy_application --environment "$environment" --action delete
            return 
         elif [ "$action" == "kustomize" ]; then
            pushd kubernetes/overlays/"$environment"
            t="$(mktemp).yml" && kubectl kustomize ./ > $t && printf "rendered manifest template: file://$t\n"  # code -n $t
            popd
            return
         elif [ "$action" == "app" ]; then
            manual_deploy_application --environment "$environment" --action install
            return
        elif [ "$action" != "install" ]; then
            # Call the function based on the argument
            if declare -f "$action" > /dev/null; then
                "$action" "$@" # Call the function
                return
            else
                echo "Unknown action: $action"
                return 
            fi
        fi
    }

    if [ "$environment" == "development" ]; then
        manual_deploy_application --environment "$environment" --action delete # required to delete postgresql claims of ory stack
        install_ory_stack --environment "$environment" --action delete
    fi

    install_ory_stack --action $environment --action install
    manual_deploy_application --environment "$environment" --action install
    
    echo "Services deployed to the cluster (wait few minutes to complete startup and propagate TLS certificate generation)."

    _fix() { 
        restart_cilinium() {
            kubectl -n kube-system rollout restart deployment/cilium-operator
            kubectl -n kube-system rollout restart ds/cilium
        }

        restart_cilinium  # [issue] restarting fixs gateway has no ip assignment by controller
    }
    # verify cluster certificate issued successfully 
    _verify() {
        ### generate combined configuration
        kubectl kustomize ./service/cilium-gateway/k8s/development > ./tmp/combined_manifest.yml
        cat ./tmp/combined_manifest.yml | kubectl apply -f -

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
        domain_name="donation-app.test"
        curl -i http://$domain_name
        curl --insecure -I https://$domain_name
        cloud_load_balancer_ip=""
        curl -i --header "Host: donation-app.test" $cloud_load_balancer_ip
        kubectl logs -n kube-system deployments/cilium-operator | grep gateway

        # run ephemeral debug container
        kubectl run -it --rm --image=nicolaka/netshoot debug-pod --namespace some_namespace -- /bin/bash 
        kubectl run -it --rm --image=busybox debug-pod-2 --namespace auth -- /bin/bash nslookup oathkeeper-proxy
        
        kubectl -n kube-system edit configmap cilium-config
    }

}

