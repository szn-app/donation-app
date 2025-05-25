-- TODO:

-- Start a transaction
BEGIN;

-- Statement 1: Add a new item to the 'listing.item' table
-- Let's assume $1 (type), $2 (intent_action), $3 (status), $4 (title), 
-- $5 (description), $6 (category_id), $7 (condition), $8 (location_id), $9 (created_by_user_id)
-- are placeholders for the actual values.
INSERT INTO "listing"."item" (
    type, intent_action, status, title, description, category, condition, location, created_by
) VALUES (
    'Donation', 'Offer', 'Active', 'Vintage Lamp', 'A beautiful vintage lamp, fully functional.', 1, 'UsedGood', 101, 55
) RETURNING id AS new_item_id;

-- At this point, if the application code were executing this, 
-- it would capture the 'new_item_id' returned by the previous statement.
-- For this SQL-only example, let's imagine 'new_item_id' is 123.
-- (In a real application, you'd use the actual returned ID.)

-- Statement 2: Add media associated with the new item to 'listing.media'
-- Let's assume $1 (item_id), $2 (url), $3 (type), $4 (position)
-- are placeholders. We use the 'new_item_id' (e.g., 123) from the previous step.
INSERT INTO "listing"."media" (
    id_item, url, type, position
) VALUES (
    123, -- This would be the actual new_item_id returned above
    'http://example.com/images/vintage_lamp.jpg', 
    'Image', 
    1
);

-- If both INSERT statements are successful, commit the transaction to make the changes permanent.
COMMIT;

-- If an error had occurred in any of the above statements (e.g., a NOT NULL constraint violation, 
-- a foreign key violation, or the media URL was invalid and caused an application-level error before commit),
-- the application logic would issue a ROLLBACK instead of a COMMIT.
-- Example (conceptual, as it wouldn't be reached if COMMIT was successful):
-- ROLLBACK; 