--- ============================ Item ============================
--- Content agnostic container for scheduling data and model data.
CREATE TABLE IF NOT EXISTS item (
    id INTEGER PRIMARY KEY,
    maturity TEXT NOT NULL DEFAULT "New",
    stability REAL NOT NULL DEFAULT 0.0,                      -- sra parameter. The number of days since last review date until probability of recal reaches 90%
    difficulty REAL NOT NULL DEFAULT 0.0,                     -- sra parameter. Number between 1 and 10. Meausure of item difficulty
    last_review_date TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP, -- sra parameter. Date in iso8601
    n_reviews INTEGER NOT NULL DEFAULT 0,                     -- sra parameter. Number of times we've review and given the review a score.
    n_lapses INTEGER NOT NULL DEFAULT 0,                      -- sra parameter. Number of times we've failed to recall the item.
    model TEXT NOT NULL,                                      -- the model, tells us how data is to be interpreted
    data TEXT NOT NULL,                                       -- untyped text field, usually json data
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,       -- metadata
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP        -- metadata
);
--- keep update_at column in sync
CREATE TRIGGER IF NOT EXISTS
update_at_field_trigger__item
AFTER UPDATE ON
item
WHEN OLD.maturity <> NEW.maturity OR
    OLD.stability <> NEW.stability OR
    OLD.difficulty <> NEW.difficulty OR
    OLD.last_review_date <> NEW.last_review_date OR
    OLD.n_reviews <> NEW.n_reviews OR
    OLD.model <> NEW.model OR
    OLD.data <> NEW.data
BEGIN
    UPDATE item SET updated_at = datetime('now') WHERE id == old.id;
END;
--- --------------------------------------------------------------------------


--- ============================ Tag ============================
CREATE TABLE IF NOT EXISTS tag (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,                          -- name of tag
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP, -- metadata
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP  -- metadata
);
-- keep update_at field in sync
CREATE TRIGGER IF NOT EXISTS
update_at_field_trigger__tag
AFTER UPDATE ON
tag
WHEN OLD.name <> NEW.name
BEGIN
    UPDATE item SET updated_at = datetime('now') WHERE id == old.id;
END;
CREATE TABLE IF NOT EXISTS tag_item_map (
    id INTEGER PRIMARY KEY,
    tag_id INTEGER NOT NULL,
    item_id INTEGER NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP, -- metadata
    FOREIGN KEY(tag_id) REFERENCES tag(id) ON DELETE CASCADE,
    FOREIGN KEY(item_id) REFERENCES item(id) ON DELETE CASCADE,
    UNIQUE(tag_id, item_id)
);
--- --------------------------------------------------------------------------


--- ============================ Due items ============================
CREATE VIEW IF NOT EXISTS due_item AS
SELECT
    *
FROM
    item
WHERE
    date(last_review_date, '+' || stability || ' days') < date('now')
ORDER BY
    stability ASC;
--- --------------------------------------------------------------------------


--- ============================ New items ============================
CREATE VIEW IF NOT EXISTS new_item AS
SELECT
    *
FROM
    item
WHERE
    maturity == "New"
ORDER BY
    n_reviews ASC, created_at ASC;
--- --------------------------------------------------------------------------
