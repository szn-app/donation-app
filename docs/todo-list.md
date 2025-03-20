
## CNPG setup recommendations - best practices for database management and application architecture to avoid common pitfalls (ask AI chat for more)
- Use Kubernetes jobs for database migrations and backups
- Regularly update your images and apply security patches.
- enabling Point in Time Recovery (PITR) can also enhance performance.
- check Fluentd or Logstash to collect logs from the services and api gateway
- Postgres's https://www.postgresql.org/docs/current/ddl-rowsecurity.html. https://supabase.com/docs/guides/database/postgres/row-level-security
- production checklist https://supabase.com/docs/guides/deployment/going-into-prod
- backup databse workflow https://supabase.com/docs/guides/deployment/ci/backups
- migration DDL statements
- local node storage cannot be used for db data as if the pod restarts on a different node, the persistent volume will not be available (doesn't apply for CNPG as the policy dicrates the volume creation or transfer)