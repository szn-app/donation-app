-- Schema Types:
-- id_item: BIGINT (references listing.item(id))
-- id_collection: BIGINT (references listing.collection(id))
-- note: TEXT (max length 10000)
-- position: INT4 NOT NULL
-- added_by: UUID NOT NULL
-- posted_on: TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
-- Note: PRIMARY KEY (id_item, id_collection)

SELECT 
    id_item,
    id_collection,
    note,
    position,
    added_by,
    posted_on
FROM "listing"."publish"
ORDER BY posted_on DESC; 