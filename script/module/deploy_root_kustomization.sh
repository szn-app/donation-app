# using skaffold instead
manual_deploy_application() {
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
