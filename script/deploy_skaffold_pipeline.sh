#!/bin/bash
# set -e

stop_minikube#task() {
    minikube stop --profile minikube
    minikube status

    sudo systemctl stop docker
    sudo systemctl status docker
}

start_minikube_services_only#task() {
    minikube start --profile minikube
    minikube status

    sudo systemctl start docker
    sudo systemctl status docker
}

# run & expose gateway with minimum scaffold services
setup.minikube#bootstrap#task@monorepo() {
    # sudo echo "" # prompt for sudo password

    run_minikube() {
        # Check if minikube is already running
        if ! minikube status --profile minikube &>/dev/null; then
            echo "Starting minikube..."
            minikube start --profile minikube --namespace donation-app
        else
            echo "Minikube is already running"
        fi

        skaffold config set --global local-cluster true
        eval $(minikube --profile minikube docker-env) # use docker daemon inside minikube
    }

    fix_sync_issue() {
        # fixes issue with namespaces affecting skaffold sync - https://github.com/GoogleContainerTools/skaffold/issues/1668#issuecomment-595752550
        # namespace config when set to undefined prevents sync errors
        # local current_ns; 
        current_ns="$(kubectl config view --minify --output 'jsonpath={..namespace}')"
        kubectl config set-context --current --namespace=
        # (sleep 3 && kubectl config set-context --current --namespace="${current_ns}" ) &
    }

    set_user_inotify_limit() {
        # increase inotify limit for user if not already set
        if ! grep -q "fs.inotify.max_user_instances" /etc/sysctl.conf; then
            echo fs.inotify.max_user_instances=524288 | sudo tee -a /etc/sysctl.conf && sudo sysctl -p
            echo "Increased inotify limit for user"
        else
            echo "inotify limit already configured"
        fi
    }

    run_scaffold_only() {
        pushd scaffold && skaffold run --profile development && popd
    }

    run_minikube
    
    set_user_inotify_limit
    fix_sync_issue
    
    execute.util '#setup' '#mount-bind' # ensure mounts are setup
    execute.util '#predeploy-hook' # prepare for deployment

    echo "successfully setup Minikube vM"
}

# remove docker images and cleanup disk space
freeup_space.minikube#cleanup#task@monorepo() {
    minikube ssh df
    minikube ssh 'docker image prune -a -f'
    minikube ssh -- docker system prune
    docker system prune --all --force
    sudo docker system prune --volumes

    verify() {
        minikube ssh -- df -h
        df -h
        df -h /var
    }
}

minikube#aggregate_cleanup#task@monorepo() {
    execute.util '#minikube_cleanup' '#task'
}

development_mode.skaffold#task@monorepo() {
    delete() {
        skaffold delete --profile development 
    }

    # run on separate shell
    expose_domain() {
        start.tunnel.minikube#task@monorepo -v
    }
    
    wait_for_terminating_resources.kubernetes#utility
    # setup.minikube#bootstrap#task@monorepo

    skaffold dev --profile development  --module monorepo --port-forward --auto-build=false --auto-deploy=false --cleanup=false --tail

    dev_expose_service() { 
        source ./script.sh
        start.tunnel.minikube#task@monorepo_delete # if already running will case connection issues, thus requires deletion
        start.tunnel.minikube#task@monorepo -v
    }
}

delete.production_mode.skaffold#task@monorepo() {
    skaffold delete --profile local-production
}

production_mode.skaffold#task@monorepo() {
    setup.minikube#bootstrap#task@monorepo
    
    wait_for_terminating_resources.kubernetes#utility

    skaffold run --profile local-production  --module monorepo # --port-forward --tail

    start.tunnel.minikube#task@monorepo -v
}

services.production_mode.skaffold#task@monorepo() {
    wait_for_terminating_resources.kubernetes#utility
    skaffold run --profile development --module monorepo-services-only --port-forward --cleanup=false
}

scaffold.production_mode.skaffold#task@monorepo() {
    wait_for_terminating_resources.kubernetes#utility
    skaffold run --profile local-production --module monorepo-scaffold-only --port-forward --cleanup=false
}

production_mode.local.skaffold#minikube#task@monorepo() {
    setup.minikube#bootstrap#task@monorepo
    
    wait_for_terminating_resources.kubernetes#utility

    skaffold run --profile local-production  --module monorepo --cleanup=false # --port-forward --tail

    start.tunnel.minikube#task@monorepo -v
}

production.skaffold#minikube#task@monorepo() {
    setup.minikube#bootstrap#task@monorepo
    
    wait_for_terminating_resources.kubernetes#utility

    skaffold run --profile production --module monorepo --cleanup=false # --port-forward
    # --profile prod-env --profile prebuilt

    start.tunnel.minikube#task@monorepo -v
}

delete.skaffold#task@monorepo() {
    skaffold delete --profile development
    skaffold delete --profile local-production
    skaffold delete --profile production

    execute.util '#task' '#manual-delete'
    execute.util '#task' '#pvc-manual-delete'
}

production.skaffold#hetzner#task@monorepo() {
    kubectl ctx k3s

    execute.util '#setup' '#mount-bind' # ensure mounts are setup
    execute.util '#predeploy-hook' # prepare for deployment

    wait_for_terminating_resources.kubernetes#utility
    skaffold run --profile production --module monorepo
}

delete.production.skaffold#task@monorepo() {
    skaffold delete --profile production --module monorepo

    execute.util '#task' '#manual-delete'
}

# Nuke stuck resources
# TODO: diagnose stuck resources issue pvc/pv
pvc.delete.production#example#task@monorepo() {
    NAMESPACE="auth"
    echo "💥 FORCEFULLY DESTROYING MinIO + Longhorn PVCs, PVs, and volumes in namespace: $NAMESPACE"

    # Step 1: Kill all MinIO workloads
    kubectl delete all --all -n "$NAMESPACE" --wait=false --grace-period=0 --force 2>/dev/null

    # Step 2: Force delete PVCs
    kubectl get pvc -n "$NAMESPACE" -o name | while read pvc; do
    pvc_name=${pvc##*/}
    echo "🔥 Wiping PVC finalizers: $pvc_name"
    kubectl patch "$pvc" -n "$NAMESPACE" -p '{"metadata":{"finalizers":[]}}' --type=merge 2>/dev/null
    kubectl delete "$pvc" -n "$NAMESPACE" --grace-period=0 --force --wait=false 2>/dev/null
    done

    # Step 3: Force delete related PVs
    kubectl get pv -o json | jq -r '.items[] | select(.spec.claimRef.namespace=="'"$NAMESPACE"'") | .metadata.name' | while read pv; do
    echo "💣 Nuking PV: $pv"
    kubectl patch pv "$pv" -p '{"metadata":{"finalizers":[]}}' --type=merge 2>/dev/null
    kubectl delete pv "$pv" --grace-period=0 --force --wait=false 2>/dev/null
    done

    # Step 4: Destroy Longhorn volumes linked to the namespace
    kubectl get volumes.longhorn.io -n longhorn-system -o json | jq -r '.items[] | select(.status.kubernetesStatus.namespace=="'"$NAMESPACE"'") | .metadata.name' | while read lv; do
    echo "🧨 Deleting Longhorn volume: $lv"
    kubectl patch volume -n longhorn-system "$lv" -p '{"metadata":{"finalizers":[]}}' --type=merge 2>/dev/null
    kubectl delete volume "$lv" -n longhorn-system --force --grace-period=0 2>/dev/null
    done

    {
        # kubectl delete validatingwebhookconfiguration longhorn-webhook-validator
        # kubectl delete mutatingwebhookconfiguration longhorn-webhook-mutator

        kubectl get pvc -n api-data -o jsonpath='{.items[*].metadata.name}' | tr ' ' '\n' | while read pvc; do
            kubectl patch pvc "$pvc" -n api-data --type=merge -p '{"metadata":{"finalizers":[]}}' || true
        done

        kubectl get pvc -n api-data -o jsonpath='{.items[*].metadata.name}' | tr ' ' '\n' | while read pvc; do
            kubectl delete pvc "$pvc" -n api-data --force --grace-period=0 || true
        done
    }

    {
        echo ""
        # kubectl delete volume.longhorn.io <volume-name> -n longhorn-system --grace-period=0 --force
    }

    {
        echo ""
        # for vol in $(kubectl get volumes.longhorn.io -n longhorn-system -o name); do
        # name=$(basename $vol)
        # kubectl get $vol -n longhorn-system -o json \
        #     | jq '.metadata.finalizers=[]' \
        #     > temp.json
        # kubectl replace --raw "/apis/longhorn.io/v1beta1/namespaces/longhorn-system/volumes/$name" -f temp.json
        # kubectl delete $vol -n longhorn-system --grace-period=0 --force
        # done

        # kubectl delete ns longhorn-system --grace-period=0 --force
    }


    echo "✅ Full force cleanup completed."
}

# NOTE: ABANDANONED DUE TO ISSUES WITH NONE DRIVER when running baremetal to solve inotify issues
ABANDANONED_dev_skaffold_inotify_volume() {
    source ./script.sh
    ABANDONED_bootstrap_minikube_baremetal

    minikube_mount_root() {
        # https://stackoverflow.com/questions/38682114/hostpath-with-minikube-kubernetes
        # https://minikube.sigs.k8s.io/docs/handbook/persistent_volumes/
        minikube mount .:/mnt/repo-root

        verfiy() { 
            minikube ssh "ls /mnt/repo-root"
        }
        delete() {
            # Kill all background jobs when cleaning up mount
            jobs -p | xargs -r kill
        }
    }

    minikube_mount_root &

    pushd service/api-data/server
    skaffold dev --profile volume-development --module monorepo --port-forward --auto-build=false --auto-deploy=false --cleanup=false --tail
    popd

    {
        # analyze inotify limit between minikube mount of host directory:
        # notify-forwarder binary: https://itnext.io/inotify-minikube-watch-feature-in-minikube-aef6edeb6f08
        # https://minikube.sigs.k8s.io/docs/handbook/filesync/
        # https://minikube.sigs.k8s.io/docs/handbook/mount/
        # https://minikube.sigs.k8s.io/docs/drivers/none/
        # https://github.com/kubernetes/minikube/issues/1551
        minikube status
        minikube config view
        minikube config set driver none # runs on bare-metal without virtualization and provides minikube direct access to host resources
    }
}

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
    TEMP_FILE=$(mktemp -t skaffold_render_XXXXXX.log) && skaffold render --profile development > "$TEMP_FILE" 2>&1 && echo "Skaffold render output saved to: $TEMP_FILE"
    TEMP_FILE=$(mktemp -t skaffold_diagnose_XXXXXX.log) && skaffold diagnose > "$TEMP_FILE" 2>&1 && echo "Skaffold diagnose output saved to: $TEMP_FILE"
    skaffold inspect profile list | jq 
    skaffold diagnose --module scaffold-generic --profile local-production | grep -C 10 scaffold-k8s
    skaffold render --module scaffold-generic --profile local-production

    skaffold delete --profile development
    skaffold build -v debug 
    skaffold dev --tail --profile development --module monorepo --port-forward --auto-build=false --auto-deploy=false --cleanup=false 
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
