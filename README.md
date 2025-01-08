# Spbased

## Description

Spbased is a content agnostic spased repetition tool. It only knows
about the notion of a generic "review item", an object with an id,
parameters for the spased repetition algorithm, and some json data.
The full schema can be seen below.

```sql
CREATE TABLE IF NOT EXISTS item (
    id INTEGER PRIMARY KEY,
    maturity TEXT NOT NULL DEFAULT "New",
    stability REAL NOT NULL DEFAULT 0.0,                      -- sra parameter. The number of days since last review date until probability of recal reaches 90%
    difficulty REAL NOT NULL DEFAULT 0.0,                     -- sra parameter. Number between 1 and 10. Meausure of item difficulty
    last_review_date TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP, -- sra parameter. Date in iso8601
    n_reviews INTEGER NOT NULL DEFAULT 0,                     -- sra parameter. Number of times we've review and given the review a score.
    n_lapses INTEGER NOT NULL DEFAULT 0,                      -- sra parameter. Number of times we've failed to recall the item.
    model TEXT NOT NULL,                                      -- the model, tells us how data is to be interpreted
    data TEXT NOT NULL,                                       -- json data
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,       -- metadata
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP        -- metadata
);
```

It is built as a command line tool, to be wrapped in custom scripts
for more advanced or streamlined usecases.

## Quick rundown of vocabulary

- "review item" - the flashcard,prompt,task etc. being scheduled in a
  spased maner.
- "review" - (in the context of spbased) given a review item that is
  due, launch the program that can interpret it, review it and return
  a measure of how well it went.
- "review item model" - or just 'model', the specific type of review
  item, and

## Command line usage

### Filter expression language

The commandline query commands (review command included) can be passed
a `--pre-filter` flag

## Examples

This repo also contains a few example scripts that showcase how the
tool can be used.
