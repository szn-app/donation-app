-- Schema Types:
-- id: BIGINT (GENERATED ALWAYS AS IDENTITY PRIMARY KEY)
-- id_item: BIGINT (references listing.item(id))
-- caption: VARCHAR(150)
-- url: VARCHAR(2048)
-- type: media_type ENUM ('image', 'video')
-- created_at: TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL

INSERT INTO "listing"."media" (id_item, caption, url, type)
VALUES ($1, $2, $3, $4)
RETURNING id, id_item, caption, url, type, created_at; 