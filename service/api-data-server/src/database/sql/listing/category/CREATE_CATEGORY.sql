-- Create a new listing category
INSERT INTO "listing"."category" (
    title,
    description,
    category_parent
)
VALUES (
    $1, -- title (VARCHAR(150))
    $2, -- description (TEXT)
    $3  -- category_parent (BIGINT)
)
RETURNING 
    id,
    title,
    description,
    category_parent; 