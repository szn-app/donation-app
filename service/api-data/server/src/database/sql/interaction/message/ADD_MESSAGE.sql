-- Create a new message
INSERT INTO "interaction"."message" (
    id_item,
    content,
    created_by
)
VALUES (
    $1, -- id_item (BIGINT)
    $2, -- content (TEXT)
    $3  -- created_by (BIGINT)
)
RETURNING 
    id,
    id_item,
    content,
    created_by,
    created_at; 