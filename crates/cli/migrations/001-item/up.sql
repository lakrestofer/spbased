--- ============================ Item ============================ 
--- Content agnostic container for scheduling data and model data.
CREATE TABLE IF NOT EXISTS item (
    id INTEGER PRIMARY KEY,
    maturity TEXT NOT NULL DEFAULT "NEW",
    stability REAL NOT NULL DEFAULT 0.0,                      -- sra parameter
    difficulty REAL NOT NULL DEFAULT 0.0,                     -- sra parameter
    last_review_date TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP, -- sra parameter
    model TEXT NOT NULL,                                      -- the model, tells us how data is to be interpreted
    data TEXT NOT NULL,                                       -- untyped text field, usually json data
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,       -- metadata
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP        -- metadata
);
CREATE TRIGGER IF NOT EXISTS 
update_at_field_trigger__item 
AFTER UPDATE ON 
item 
BEGIN
    UPDATE item SET updated_at = datetime('now');
END;
--- --------------------------------------------------------------------------


--- ============================ Tag ============================ 
CREATE TABLE IF NOT EXISTS tag (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,                          -- name of tag
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP, -- metadata
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP  -- metadata
);
CREATE TRIGGER IF NOT EXISTS 
update_at_field_trigger__tag 
AFTER UPDATE ON 
tag 
BEGIN
    UPDATE tag SET updated_at = datetime('now');
END;
CREATE TABLE IF NOT EXISTS tag_item_map (
    id INTEGER PRIMARY KEY,
    tag_id INTEGER NOT NULL,
    item_id INTEGER NOT NULL, 
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP, -- metadata
    FOREIGN KEY(tag_id) REFERENCES tag(id),
    FOREIGN KEY(item_id) REFERENCES item(id),
    UNIQUE(tag_id, item_id)
);
--- --------------------------------------------------------------------------


--- ============================ Due items ============================ 
CREATE VIEW IF NOT EXISTS due_items AS
SELECT 
    *
FROM 
    item
WHERE 
    date(last_review_date, '+' || stability || ' days') < date('now')
ORDER BY 
    stability ASC;
--- --------------------------------------------------------------------------
