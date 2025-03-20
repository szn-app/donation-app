example@cnpg() {
    upgrade() { 
        # set to maintenance mode
        kubectl cnpg maintenance set --all-namespaces

        # do the upgrade TODO: 

        # After upgrade is complete, unset maintenance mode
        kubectl cnpg maintenance unset --all-namespaces
    }

    backups() {
        # https://cloudnative-pg.io/documentation/1.22/backup_barmanobjectstore/
        echo ''
    }

    pushd service/api-data/database

    if ! kubectl get secret postgresql-superuser-credentials -n database &>/dev/null; then
        kubectl create secret generic postgresql-superuser-credentials -n database \
            --from-literal=username=user-postgres \
            --from-literal=password=pass-postgres
    fi

    # TODO: get minio credentilas 

    popd
    
    verify() {
        kubectl describe clusters.postgresql.cnpg.io -n database
        kubectl logs -l cnpg.io/cluster=cluster-app-data -n database
        watch kubectl get pods,service,pvc,pdb -n database -o wide -w
        kubectl get pods -A -l cnpg.io/cluster=cluster-app-data
        kubectl cnpg status cluster-app-data -n database --verbose

        connect_to_db_from_pod(){
            # Get the first pod from the cluster and run psql on it
            CLUSTER_POD=$(kubectl get pods -A -l cnpg.io/cluster=cluster-app-data -o jsonpath='{.items[0].metadata.name}' -n database)
            kubectl exec -it $CLUSTER_POD -n database -- psql -U postgres
            # SELECT usename, passwd FROM pg_shadow WHERE usename = 'user-postgres';
            # then run commands: \l, \c database_name, \dt
        }

        connect_to_db() {
            # username and password info
            kubectl get secret postgresql-credentials -n database -o json | jq -r '.data | to_entries[] | "\(.key): \(.value|@base64d)"'
            local PASSWORD=$(kubectl get secret postgresql-credentials -n database -o jsonpath='{.data.password}' | base64 --decode)
            local USERNAME=$(kubectl get secret postgresql-credentials -n database -o jsonpath='{.data.username}' | base64 --decode)
            local SERVICE_IP=$(kubectl get svc dev--cluster-app-data -n database -o jsonpath='{.status.loadBalancer.ingress[0].ip}')

            # [manual] port-forward database
            {
                echo 'minikube tunnel'
            }

            psql postgresql://$USERNAME:$PASSWORD@$SERVICE_IP:5432/app
        }

        create_backups() {
            kubectl cnpg backup cluster-app-data --backup-name example -n database # using Barman to S3
            kubectl cnpg backup cluster-app-data --backup-name example -n database --method volumeSnapshot
            kubectl get backup -n database -o wide
        }
    }
}
