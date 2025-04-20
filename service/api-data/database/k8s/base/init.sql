
--------- Postgresql sematnics https://www.postgresql.org/docs/current/protocol-flow.html#PROTOCOL-FLOW-MULTI-STATEMENT

-- example table
create table IF NOT EXISTS test (i integer);
insert into test (i) select generate_series(1, 100);
GRANT ALL ON test TO "postgres-user";
---

CREATE SCHEMA IF NOT EXISTS user AUTHORIZATION "postgres-user";
-- app tables: 
CREATE TABLE IF NOT EXISTS user.account (
    id UUID NOT NULL UNIQUE
);
GRANT ALL ON user.account TO "postgres-user";

---

-- TODO: 
-- CREATE EXTENSION postgis;



-- TODO: copy over SQL from ChartDB (export to Postgresql or use LLM to convert generic SQL to postgres)