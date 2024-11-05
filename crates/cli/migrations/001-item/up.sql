--- Model. Descriptor of Review Item data format
--- and the program used to consume it.
CREATE TABLE IF NOT EXISTS review_item_model (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,       -- name of model
    program TEXT NOT NULL,    -- name of program on $PATH that will be called
    updated_at TEXT NOT NULL, -- metadata
    created_at TEXT NOT NULL -- metadata
);

--- Review Item. Content agnostic container for scheduling data
--- and model data.
CREATE TABLE IF NOT EXISTS review_item (
    id INTEGER PRIMARY KEY,
    stability REAL NOT NULL,        -- sra parameter
    difficulty REAL NOT NULL,       -- sra parameter
    last_review_date TEXT NOT NULL, -- sra parameter
    model INTEGER NOT NULL,         -- the model, tells us how data is to be interpreted
    data TEXT NOT NULL,             -- untyped text field, usually json data
    updated_at TEXT NOT NULL,       -- metadata
    created_at TEXT NOT NULL,       -- metadata
    FOREIGN KEY(model) REFERENCES review_item_model(id)
);

--- tag
CREATE TABLE IF NOT EXISTS tag (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    updated_at TEXT NOT NULL, -- metadata
    created_at TEXT NOT NULL -- metadata
);

--- mapping between tag and review_item
CREATE TABLE IF NOT EXISTS tag_item_map (
    id INTEGER PRIMARY KEY,
    tag_id INTEGER NOT NULL,
    item_id INTEGER NOT NULL,
    FOREIGN KEY(tag_id) REFERENCES tag(id),
    FOREIGN KEY(item_id) REFERENCES review_item(id)
);
