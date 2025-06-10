/*
 Postgresql sematnics https://www.postgresql.org/docs/current/protocol-flow.html#PROTOCOL-FLOW-MULTI-STATEMENT
 */
-- example table
create table configmaps (i integer);
insert into configmaps (
        select generate_series(1, 10000)
    );
---
-- example extensions: 
-- CREATE EXTENSION postgis;