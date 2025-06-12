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


