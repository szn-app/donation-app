/*
 Postgresql sematnics https://www.postgresql.org/docs/current/protocol-flow.html#PROTOCOL-FLOW-MULTI-STATEMENT
 */
-- example table
create table test (i integer);
insert into test (
        select generate_series(1, 10000)
    );
---
-- example extensions: 
-- CREATE EXTENSION postgis;