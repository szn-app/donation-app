#!/bin/bash
set -e

skaffold#task@api-data-database() { 
    pushd "$(dirname "$(dirname "${BASH_SOURCE[0]}")")" # two levels up: from script directory to project root
    
    skaffold dev --profile development --port-forward --tail

    popd
}

render.skaffold#task@api-data-database() {
    pushd "$(dirname "$(dirname "${BASH_SOURCE[0]}")")" 
    temp_file=$(mktemp) && skaffold render --profile development > "$temp_file" && echo "$temp_file"
    popd
}

delete.skaffold#task@api-data-database() {(
    pushd "$(dirname "$(dirname "${BASH_SOURCE[0]}")")" 

    skaffold delete --profile development

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
    sql2dbml ./k8s/base/init.sql -o ./k8s/base/init-sql-autogen.dbml
    popd
)}

### option 1.
# manual load sql to .drawio.svg
### option 2: 
view-db-diagram#task@api-data-database() {(
    pushd "$(dirname "$(dirname "${BASH_SOURCE[0]}")")" 
    install() {
        pnpm add -g @dbml/cli # fails for sql with extension syntex it seems.
    }
    
    sql2dbml ./k8s/base/init.sql -o ./k8s/base/init-sql-autogen.dbml

    ### [option 2]
    docker run -p 8080:80 ghcr.io/chartdb/chartdb:latest
    # -v $(pwd)/k8s/base/init.sql:/schema.sql # not working, no way to load on startup :(
    
    # [manual] requires running command against Postgres database or load DBML format from previous stage (copy-paste; file picker not supported for DBML)

    popd
)}

### [OBSOLETE] also download https://github.com/jgraph/drawio-desktop/releases/tag/v26.2.2 for local development/testing because it supports sql to diagram directly (copy SQL from LLM to diagram) --- ( actually it is not perfect, it doesn't create relations only tables)