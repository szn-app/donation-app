#!/bin/bash
# set -e

# run & expose gateway with minimum scaffold services
dev.local#bootstrap#task@monorepo() {
    sudo echo "" # prompt for sudo password

    {
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

    set_user_inotify_limit
    fix_sync_issue
    execute.util '#predeploy-hook'

    # pushd scaffold && skaffold run --profile development && popd        
    tunnel_minikube -v
}

dev.skaffold#task@monorepo() {
    delete() {
        skaffold delete --profile development 
    }

    # run on separate shell
    expose_domain() {
        tunnel_minikube -v
    }
    
    wait_for_terminating_resources
    # dev.local#bootstrap#task@monorepo

    skaffold dev --profile development --port-forward --auto-build=false --auto-deploy=false --cleanup=false --tail

    dev_expose_service() { 
        source ./script.sh
        tunnel_minikube_delete # if already running will case connection issues, thus requires deletion
        tunnel_minikube -v
    }

    freeup_minikube_space() {
        minikube ssh df
        minikube ssh 'docker image prune -a -f'
        docker system prune --all --force
    }

    verify() { 
        kubectl config view
        skaffold config list
        TEMP_FILE=$(mktemp -t skaffold_render_XXXXXX.log) && skaffold render --profile development > "$TEMP_FILE" 2>&1 && echo "Skaffold render output saved to: $TEMP_FILE"
        TEMP_FILE=$(mktemp -t skaffold_diagnose_XXXXXX.log) && skaffold diagnose > "$TEMP_FILE" 2>&1 && echo "Skaffold diagnose output saved to: $TEMP_FILE"
        
        skaffold delete --profile development
        skaffold build -v debug 
        skaffold dev --tail --profile development --port-forward --auto-build=false --auto-deploy=false --cleanup=false 
        skaffold dev --port-forward -v debug
        skaffold debug
        skaffold run
    }

    example_minikube_scripts() {
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

dev_production_mode.skaffold#task@monorepo() {
    delete() {
        skaffold delete --profile local-production
    }

    expose_domain() {
        tunnel_minikube -v
    }

    wait_for_terminating_resources
    # dev.local#bootstrap#task@monorepo

    skaffold dev --profile local-production --port-forward --tail

    verify() { 
        skaffold run --profile production --tail

    }
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
    skaffold dev --profile volume-development --port-forward --auto-build=false --auto-deploy=false --cleanup=false --tail
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
