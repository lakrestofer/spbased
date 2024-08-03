CREATE TRIGGER IF NOT EXISTS  review_item_trigger__updated_field AFTER UPDATE ON "review_item"
BEGIN
    UPDATE "review_item"
    SET "updated" = CURRENT_TIMESTAMP
    WHERE id = NEW.id;
END;
