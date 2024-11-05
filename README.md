# Spbased

Spbased is a content agnostic spased repetition tool. It only knows about the notion of a
generic "review item", an object with an id, parameters for the spased repetition algorithm,
a specific "item type", and a "data" field. That's it. 

- review item
  - id
  - review parameters (difficulty, stability, date of last review etc)
  - review item type
    - foreign id to 
  - generic data field (string. can store json, some id, whatever)
    - The data field is just a string. It is up to the user to interpret the data.
  - other metadata (updated at, created at etc)
  - tags

- For a review item to be useful, it needs an associated item type.

- register one or several items types
  - item type
    - name
    - program that is used for "reviewing" a specific review item.
      - expects the contents of the data field
      - returns 

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

