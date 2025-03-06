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

    kubectl_apply_application() {
        local environment="development" # environment = development, production
        local action="install" # action = install, delete

        while [[ "$#" -gt 0 ]]; do
            case $1 in
                --environment) environment="$2"; shift ;;
                --action) action="$2"; shift ;;
                *) echo "Unknown parameter passed: $1"; exit 1 ;;
            esac
            shift
        done

        environment_short=$(if [ "$environment" == "development" ]; then echo "dev"; else echo "prod"; fi)

        {
            if [ "$action" == "delete" ]; then
                kubectl delete -k ./kubernetes/overlays/$environment_short
                return 
            fi
        }

        # DEPRECETED: directory structure changed; the kubernetes/overlays is part of scaffold/k8s/overlays; Only the scaffold services (auth, gateway, metrics, etc) are referenced in the kustomization, which is contrary to the previous structure where all services were referenced in the kustomization.
        pushd ./kubernetes/overlays/
            kubectl apply -k ./$environment_short

            if [ "$environment" == "development" ]; then
                # restart all deployments in any namespace
                kubectl get deployments --all-namespaces -o custom-columns=:.metadata.name,:.metadata.namespace | tail -n +2 | while read -r deployment namespace; do
                    kubectl rollout restart deployment "$deployment" --namespace "$namespace"
                done
            fi

            {
                pushd ./$environment_short 
                t="$(mktemp).yml" && kubectl kustomize ./ > $t && printf "rendered manifest template: file://$t\n"  # code -n $t
                popd
            }
        popd 
    }

    env_files() {
        local environment="$1"

        _related_commands() {
            find . -name '.env.template' 
            sed "s/<username>/your_username/g;s/<password>/your_password/g;s/YOUR_API_KEY/your_actual_api_key/g;s/YOUR_SECRET_KEY/your_actual_secret_key/g" < .env.template > .env
        }

        # create .env files from default template if doesn't exist
        create_env_files() {
                # Find all *.env.template files
                find . -name "*.env.template" | while IFS= read -r template_file; do
                        # Extract filename without extension
                        filename=$(basename "$template_file" | cut -d '.' -f 1)
                        env_file="$(dirname "$template_file")/$filename.env"

                        # Check if .env file already exists
                        if [ ! -f "$env_file" ]; then
                            # Create a new .env file from the template in the same directory
                            cp "$template_file" "$env_file" 
                            echo "created env file file://$env_file from $template_file"
                        fi
                done
        }

        generate_database_kratos_credentials
        generate_default_username_kratos
        generate_database_hydra_credentials
        generate_database_keto_credentials
        generate_secret_auth_ui $environment
        create_env_files
    }

    # set kubectl context based on environment
    if [ "$environment" == "production" ]; then
        kubectl ctx k3s
    elif [ "$environment" == "development" ]; then
        kubectl ctx minikube
    else
        echo "Unknown environment: $environment"
        exit 1
    fi

    env_files $environment

    {
        if [ "$action" == "delete" ]; then
            install_ory_stack --environment "$environment" --action delete 
            kubectl_apply_application --environment "$environment" --action delete
            return 
         elif [ "$action" == "kustomize" ]; then
            pushd kubernetes/overlays/"$environment"
            t="$(mktemp).yml" && kubectl kustomize ./ > $t && printf "rendered manifest template: file://$t\n"  # code -n $t
            popd
            return
         elif [ "$action" == "app" ]; then
            kubectl_apply_application --environment "$environment" --action install
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
        kubectl_apply_application --environment "$environment" --action delete # required to delete postgresql claims of ory stack
        install_ory_stack --environment "$environment" --action delete
    fi

    install_ory_stack --action $environment --action install
    kubectl_apply_application --environment "$environment" --action install
    
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

deploy_local_minikube() {
    sudo echo "elevated permission"
    action=${1:-"install"}

    if [ "$action" == "delete" ]; then
        source ./script/deploy.sh
        deploy --environment development --action delete
        return 
    elif [ "$action" == "kustomize" ]; then
        source ./script/deploy.sh
        deploy --environment development --action kustomize
        return
    fi

    if ! minikube status &> /dev/null; then
        echo "Minikube is not running. Starting Minikube..."
        minikube start
    else
        echo "Minikube is already running."
    fi

    docker_build_to_minikube() {
        # alternative approach to build all containers directly into minikube
        build_all_containers_directly_into_minikube() {
            # bind docker images directly inside minikube
            eval $(minikube docker-env) # bind docker command to minikube docker
            (cd service/web-server && ./script.sh build_container_web_server development)
            (cd service/auth-ui && ./script.sh bulid_container_auth_ui)
            (cd service/auth-token-exchange && ./script.sh build_container_auth_token_exchange)

            { # reverse minikube eval
                unset DOCKER_TLS_VERIFY
                unset DOCKER_HOST
                unset DOCKER_CERT_PATH
            }
        }

        (cd service/web-server && ./script.sh build_container_web_server development)
        docker save web-server:latest | (eval $(minikube docker-env) && docker load)

        (cd service/auth-ui && ./script.sh bulid_container_auth_ui)
        docker save auth-ui:latest | (eval $(minikube docker-env) && docker load)
        
        (cd service/auth-token-exchange && ./script.sh build_container_auth_token_exchange)
        docker save auth-token-exchange:latest | (eval $(minikube docker-env) && docker load)

        (cd service/api-data && ./script.sh build_container_api_data development)
        docker save api-data:latest | (eval $(minikube docker-env) && docker load)
    }

    docker_build_to_minikube
    
    tunnel_minikube_delete
    
    deploy --environment development --action install
    # deploy only app: 
    # deploy --environment development --action app

    read -t 20 -p "Do you want to execute tunnel_minikube? (y/n, default is y after 20 seconds): " choice
    choice=${choice:-y}
    if [[ "$choice" == "y" ]]; then
        tunnel_minikube
    else
        echo "Skipping tunnel_minikube execution."
    fi

    example_scripts() {
        kubectl config view && kubectl get namespace && kubectl config get-contexts

        (cd k8s/development && kubectl apply -k .)
        kubectl get all
    
        minikube ip 
        # expose service to host: 
        minikube tunnel # expose all possible resources (e.g. loadbalancers)
        minikube service dev-web-server --url --namespace=donation-app

        nslookup donation-app.test $(minikube ip) # query dns server running in minikube cluaster
        dig donation-app.test
        export GW=$(minikube ip) # or direct gateway ip exposed using minikube tunnel.
        curl --resolve donation-app.test:80:$GW donation-app.test
        ping donation-app.test

        # using ingress 
        kubectl describe ingress ingress -n donation-app

        # using gateway 
        {
            export GW=$(minikube ip) # or direct gateway ip exposed using minikube tunnel.
            kubectl apply -k ./service/cilium-gateway/k8s/development
            minikube tunnel # otherwise, with ingress-dns and ingress.yml re-route to gateway will make accessing gateway through domain resolution directly with minikube ip
            minikube dashboard
            kubectl describe gateway -n donation-app
            kubectl describe httproute -n donation-app
            dig donation
            curl --resolve donation-app.test:80:$GW donation-app.test
        }

        kubectl apply -k ./kubernetes/overlays/dev

        curl -i --header "Host: donation-app.test" "<ip-of-load-balancer>"
    }
}

