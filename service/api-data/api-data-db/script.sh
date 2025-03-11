# 1. create db operator 
# 2. create supabase stack connected to remote db operator 
# 3. create a new db schema

# [recommendations AI chat for] best practices for database management and application architecture to avoid common pitfalls
# Use Kubernetes jobs for database migrations and backups
# Regularly update your images and apply security patches.

# enabling Point in Time Recovery (PITR) can also enhance performance.
# check Fluentd or Logstash to collect logs from the services and api gateway
# Postgres's https://www.postgresql.org/docs/current/ddl-rowsecurity.html. https://supabase.com/docs/guides/database/postgres/row-level-security

# production checklist https://supabase.com/docs/guides/deployment/going-into-prod

# docker examples: https://github.com/supabase/supabase/tree/master/docker
# env file example https://github.com/supabase/supabase/blob/master/docker/.env.example

# generate typescript types for database frontend workflow https://supabase.com/docs/guides/deployment/ci/generating-types
# backup databse workflow https://supabase.com/docs/guides/deployment/ci/backups

# migration DDL statements

# local node storage cannot be used for db data as if the pod restarts on a different node, the persistent volume will not be available

deploy_api_data_db() {
    upgrade() { 
        # set to maintenance mode
        kubectl cnpg maintenance set --all-namespaces

        # do the upgrade TODO: 

        # After upgrade is complete, unset maintenance mode
        kubectl cnpg maintenance unset --all-namespaces
    }

    backups() { 
        # https://cloudnative-pg.io/documentation/1.22/backup_barmanobjectstore/
    }

    pushd service/api-data/api-data-db
    
    kubectl apply -k ./k8s/overlays/dev

    popd
    verify() {
        kubectl get pods -n database -o wide
        kubectl get service -n database -o wide
        kubectl get pvc -n database -o wide
        kubectl get pods -A -l cnpg.io/cluster=cluster-example
        kubectl cnpg status cluster-example -n database --verbose
        kubectl get pdb -n database

        {
            # Get the first pod from the cluster and run psql on it
            CLUSTER_POD=$(kubectl get pods -A -l cnpg.io/cluster=cluster-example -o jsonpath='{.items[0].metadata.name}' -n database)
            kubectl exec -it $CLUSTER_POD -n database -- psql -U postgres
            # then run commands: \l, \c app, \dt
        }

        {
            # username and password info
            kubectl get secret cluster-example-app -n database -o yaml
            local PASSWORD=$(kubectl get secret cluster-example-app -n database -o jsonpath='{.data.password}' | base64 --decode)
        }
        
        psql postgresql://postgres:postgres@127.0.0.1:54322/postgres
    }
}
