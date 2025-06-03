
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

- Elastic search integration with Postgresql. 

- migrate github workflow tests to within docker containers for an exact environment replica
- use .psql extension instead of .sql 
- Improve sharing of common scripts such as sqlparser-validate-syntax#task@api-data-database.script.rs to services or components of microservices that use them. Create a global modular appraoch to share files accorss services.
- restruture init.sql to use migration tool that manages migration changes over time and dump a current schema state of the database. 
- In rust use crate to define custom app errors which encompass generic and crate specific errors
- GraphQL optimization using Dataloader (N+1 problem)
- for a decoupled microservices api, use GraphQL federation as the gateway entrypoint for all the other microsrevices exposed GraphQL endpoints. 
- [drawback] Move from single initialization script (single sql migraiton) to a version-controlled structured system. 
- Production-ready: increase volume Gibibyte storage (look for commented "Gi")
- setup proper resource limitations on deployments for production to avoid CPU/Memory starvation and cascading failure when a service/deployment is fails or misconfigured. 