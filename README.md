# Spbased

## Current TODO

- [ ] finish user commands to interact with cli

## Description
Spbased is a content agnostic spased repetition tool. It only knows about the notion of a
generic "review item", an object with an id, parameters for the spased repetition algorithm,
a specific "item type", and a "data" field. That's it.

- review item
  - id
  - review parameters (difficulty, stability, date of last review etc)
  - review item type
    - foreign id to
  - generic json data field
  - other metadata (updated at, created at etc)
  - tags

- For a review item to be useful, it needs an associated item type.


- each review item may have one or more tags associated with it as well

- spbased is supposed to be adapted to your usecase using scripts
  - many things can be scheduled in a spased maner.
    - flashcards - for "question -> answer" prompts
    - some specific leetcode question
    - some blog post that can be refined over time.

- a review can be narrowed to only include review items with a specific tag, review item type


## Quick rundown of vocabulary

- "review item" - the flashcard,prompt,task etc. being scheduled in a spased maner.
- "review" - (in the context of spbased) given a review item that is due, launch the program that can interpret it, review it and return a measure of how well it went.
- "review item model" - or just 'model', the specific type of review item, and

## Example use

**Add review item (flashcard)**
```zsh
>> spbasedctl item add \
  "flashcard" \
  '{"front":"what is the capital of sweden","back":"stockholm","tags":["geography"]}'
```

**review item (flashcard)**
