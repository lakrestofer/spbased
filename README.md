# Spbased

Welcome to the monorepo for the spbased ecosystem of spaced applications. The
primary thesis of the project is that many tasks benefit from being scheduled in
a spaced manner. Currently the most common model for spaced applications is the
flashcard based one, were the items being scheduled are promts that ask the user
to remember some fact, vocabulary etc. In this project I want to see if more tasks
benefit from being scheduled in this manner.

## Spaced repetition algorithm

We utilize the FSRS algorithm initially developed by Jarrett Ye et. al. [https://github.com/open-spaced-repetition](https://github.com/open-spaced-repetition)

## Daemon

The central pillar of the project. Here we store and schedule all the review
items. I want it to exist as a daemon such that it is always available for
querying (from a systray application). The  daemon should be agnostic to what
specific data is stored for each item, only being aware of parameters such as
difficulty and stability (the parameters used to in  the FSRS algorithm).

## Other 

Here I list some "frontends" which would utilize the daemon.

- Spaced reading
  - input a source into an application
  - the source is broken up into "chunks"
  - each chunk is "reviewed" by reading it.
  - then for every time the chunk is read, one could digest it by creating some
    ordinary flashcards on the content
  - the benefit here would be that you are "guaranteed" to interact with some content.
  - the hard part about writing such an application would be making it source type agnostic.
  - should there exist a unique application for every content type (pdf, epub, html)
  - hopefully simply targeting the browser should be enough (as it probably knows how to open most source files).
- spaced flashcards
  - typical spaced repetition application
  - each item is a card with promt and answer.
- spaced typing
  - any skill is probably improved by directed practice so it is not directly clear if touch typing would benefit from being spaced.
  - typing speed is usually bottlenecked by typing errors rather than by raw finger to button speeds.
  - it is also more effective to write word by word rather than letter by letter.
  - creating correct muscle memory for the 1000 most common words of the target language is probably a effective way to greatly improve typing speeds.
- systray icon
  - simple systray icon that tells the user how many items (between all the different frontends) are due.
- dashboard application
  - "start here" application that lists all other registered spbased frontends.
  - lists the different items by type.

## Data Model

The daemon should be unaware of any data that is specific to each frontend.

review item:
- id
- review data
  - difficulty
  - stability
  - last review date (as fixed point in time, not timezone aware)
- uri
  - every frontend can register an unique uri
  - `spbased_frontend://review/{id}`
  - xdg-open can then be used to launch the relevant frontend with the uri, which then starts a review for that specific item.
- data
  - stringified json data

scheduled_review_event (the future scheduled review events):
  - id
  - item_id
  - scheduled review date

review_event_log (past review events)
  - id
  - item_id
  - scheduled review date
  - review date, when the item was actually reviewed
  - grade

## Message model

how should we interact with the server?

we will use gRPC.

what do we need to do?
- queries
  - number of due items
  - list of due items
  - items by type
  - item by id
- mutations
  - reviewResult. send grade and item id only. Only update item if first review of the day
  - new item
  - update data of item

## Libraries

- tonic
- sea-orm
