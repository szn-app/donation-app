#!/bin/bash
set -e

skaffold#task@api-data-database() { 
    pushd "$(dirname "$(dirname "${BASH_SOURCE[0]}")")" # two levels up: from script directory to project root
    
    skaffold dev --module api-data-database --profile development --port-forward --tail

    popd
}

render.skaffold#task@api-data-database() {
    pushd "$(dirname "$(dirname "${BASH_SOURCE[0]}")")" 
    temp_file=$(mktemp) && skaffold render --module api-data-database --profile development > "$temp_file" && echo "$temp_file"
    popd
}

delete.skaffold#task@api-data-database() {(
    pushd "$(dirname "$(dirname "${BASH_SOURCE[0]}")")" 

    skaffold delete --module api-data-database --profile development

    popd
)}

data-setup#test@api-data-database() {(
    pushd "$(dirname "$(dirname "${BASH_SOURCE[0]}")")" 

    local file="test/test-data-schema-setup.autogen.sql"

    PGPASSWORD=postgres psql -h localhost -p 5432 -U postgres -d app -f $file

    popd
)}

cluster#benchmark@api-data-database() {
    # TODO: benchmark CNPG cluster
    echo ''
}

bootstrap@api-data-database() { 
    echo ''
}

generate_dbml_from_sql_migration#task@api-data-database() {(
    pushd "$(dirname "$(dirname "${BASH_SOURCE[0]}")")" 
    sql2dbml --postgres ./k8s/base/init.sql -o ./k8s/base/init-sql-autogen.dbml
    popd
)}

generate_ddl_live_database#task@api-data-database() {(
    pushd "$(dirname "$(dirname "${BASH_SOURCE[0]}")")"

    PGPASSWORD=postgres pg_dump -h localhost -p 5432 -d app -U postgres -s -F p -E UTF-8 -f ./k8s/base/ddl-schema-export-from-live-database-autogen.sql
    
    popd
)}

### option 1.
# manual load sql to .drawio.svg
### option 2: 
view-db-diagram-chartdb#task@api-data-database() {(
    pushd "$(dirname "$(dirname "${BASH_SOURCE[0]}")")" 
    install() {
        pnpm add -g @dbml/cli # fails for sql with extension syntex it seems.
    }
    
    sql2dbml --postgres ./k8s/base/init.sql -o ./k8s/base/init-sql-autogen.dbml

    ### [option 2]
    docker run -p 8080:80 ghcr.io/chartdb/chartdb:latest
    # -v $(pwd)/k8s/base/init.sql:/schema.sql # not working, no way to load on startup :(
    
    # [manual] requires running command against Postgres database or load DBML format from previous stage (copy-paste; file picker not supported for DBML)

    popd
)}


debug-cluster#example@api-data-database() { 
    local cluster_name="api-data--cluster-db"
    local namespace="api-data"

    echo "Describing Cluster resource..."
    kubectl describe cluster/$cluster_name -n $namespace

    echo "Listing Pods with wide output..."
    kubectl get pods -l cnpg.io/cluster=$cluster_name -n $namespace -o wide

    # Get the first Pod name related to the cluster
    pod_name=$(kubectl get pods -l cnpg.io/cluster=$cluster_name -n $namespace -o jsonpath='{.items[0].metadata.name}')

    echo "Describing Pod: $pod_name..."
    kubectl describe pod "$pod_name" -n "$namespace"

    echo "Fetching logs from Pod: $pod_name (all containers)..."
    kubectl logs "$pod_name" -n "$namespace" --all-containers=true

    echo "Fetching recent Events in namespace $namespace..."
    kubectl get events -n "$namespace" --sort-by='.metadata.creationTimestamp'

    kubectl get pvc -n $namespace -l cnpg.io/cluster=$cluster_name
}

test_installation_of_new_extensions#docker@api-data-database() { 
    pushd service/api-data/database
    docker build . --target builder -t api-data-database:builder    
    docker run -it api-data-database:builder bash
    popd
}



### [OBSOLETE] also download https://github.com/jgraph/drawio-desktop/releases/tag/v26.2.2 for local development/testing because it supports sql to diagram directly (copy SQL from LLM to diagram) --- ( actually it is not perfect, it doesn't create relations only tables)

