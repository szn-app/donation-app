
--------- Postgresql sematnics https://www.postgresql.org/docs/current/protocol-flow.html#PROTOCOL-FLOW-MULTI-STATEMENT

-- example table
create table test (i integer);
insert into test (
    select generate_series(1, 100)
);
GRANT ALL ON test TO "postgres-user";
---

-- app tables: 
CREATE TABLE users (
    id_kratos UUID NOT NULL UNIQUE
);
GRANT ALL ON users TO "postgres-user";

---

--- TODO: 
-- install pgvector in postgresql
CREATE EXTENSION IF NOT EXISTS vector;

CREATE TABLE example_vector (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    title TEXT,
    embedding vector(4) -- 4-dimensional vector (usually in the thousands)
);
GRANT ALL ON example_vector TO "postgres-user";

INSERT INTO example_vector (embedding, title) VALUES -- usually embeddings for contents are generated using an API such as OpenAI
    ('[1,2,3,4]', 'example 1'),
    ('[5,6,7,8]', 'example 2'),
    ('[9,10,11,12]', 'example 3');
SELECT * FROM example_vector ORDER BY embedding<->'[1,2,3,4]'; -- distance of the vector to the point (get related content)


-- TODO: 
-- CREATE EXTENSION postgis;


