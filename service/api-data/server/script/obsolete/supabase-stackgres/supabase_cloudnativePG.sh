db_cloudnative_postgres() {
    # https://github.com/jniclas/supabase-cloudnativedb
    # https://github.com/orgs/supabase/discussions/31147
    #       https://github.com/voltade/cnpg-supabase/tree/main
    # https://github.com/jniclas/supabase-cloudnativedb/issues/1
    echo ''
}

# CNPG + Supabase
# 1. create db operator 
# 2. create supabase stack connected to remote db operator 
# 3. create a new db schema

# Supabase links:
# production checklist https://supabase.com/docs/guides/deployment/going-into-prod
# docker examples: https://github.com/supabase/supabase/tree/master/docker
# env file example https://github.com/supabase/supabase/blob/master/docker/.env.example
# generate typescript types for database frontend workflow https://supabase.com/docs/guides/deployment/ci/generating-types
