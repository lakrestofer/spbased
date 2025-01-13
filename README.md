# Spbased

> **NOTE!** This project is still a work in progress. There are some
> features still left on the table before a 1.0 release.

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

### Pre filter expression language

The commandline query commands (review command included) can be passed
a `--pre-filter` flag which then takes a small domainspecific language
as an argument.

The language allows for a simple combination of comparison and logical
operations. Comparisons between some `review_item` field and value
which may then be combined with logical operations.

- `--pre-filter 'id==3'`
- `--pre-filter "maturity=='Young'"`

Supported values include strings, integers, floats and booleans.

Some special handling is taken on fields which represent time. Here
care is taken such that the value is a valid point in time. Here we
may add semantic descriptions of time in the future (e.g "three days
ago")

### Post filter

Most commands that return a json result can also be passed a
`--post-filter` flag which takes a [jmespath](https://jmespath.org/)
expression. This can be done to further extract any wanted data.

```shell
> spbasedctl items query --pre-filter "maturity='Young'" --post-filter "[*].id"
```

whose output may be

```
[1, 2, 42, 4]
```

## Examples

This repo also contains a few example scripts that showcase how the
tool can be used.

**TODO:**

- [ ] some way to filter on tags needs to be implemented
  - the `--pre-filter` expression syntax could be extended to include
    a 'has tags expression'.
    - `#tag and maturity=='Young'`
  - easy to just add separate flags
    - maybe even separate tag lang
- [ ] add more scripts
  - [ ] add_flashcard
  - [ ] edit_flashcard
  - [ ] delete_flashcard
  - [ ] review_flashcard
- [ ] start using the application!

**Wish list:**

- spbased 'open this program with this metadata'
  - This would allow the content agnostic core to become really
    powerful
  - json content of an item could include a program fielda
  - thought really just what I'm doing currently

- spased reading
  - add a list of pdfs, webpages

## Description
