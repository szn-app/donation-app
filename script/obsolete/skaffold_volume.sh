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

    pushd service/api-data-server
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
