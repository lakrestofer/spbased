#!/usr/bin/env zsh
  
set -e # exit if any step has a nonzero exit code
set -o pipefail # including in the middle of a pipe

command="${1:=help}"
id="$2"

ITEM_MODEL="flashcard" # the name of this review item format

if [[ $command == "help" ]]; then
  echo "usage: flashcard command [argument]"
  echo ""
  echo "examples:"
  echo "       flashcard add    # add a new flashcard interactively"
  echo "       flashcard edit 3 # edit a flashcard interactively"
  echo "       flashcard review # review a due or new flashcard"
  exit 0
fi

# optionally read in spbasedctl path from environment
if [[ -z $FLAKE_ROOT ]]; then
  # check that spbasedctl exist globally
  if which spbasedctl>/dev/null 2>&1; then
    SPBASEDCTL_BIN="spbasedctl"
  else
    gum log --level error "SPBASEDCTL_BIN was unset and spbasedctl not found on PATH! exiting..."
    exit 1
  fi
else
  SPBASEDCTL_BIN="$FLAKE_ROOT/target/debug/spbasedctl"
fi

fatal() {
  local message="$1"
  gum log --level error $message
  exit 1
}

add() {
  # prompt the user for their question answer pair
  QUESTION=$(gum write --placeholder="question")
  ANSWER=$(gum write --placeholder="answer")

  ## Both question and answer must be set
  if [[ -z "$QUESTION" || -z "$ANSWER" ]]; then
    gum log --level error "must provide non empty input"
    exit 1
  fi

  # build stringified json object
  FLASHCARD=$(jq --compact-output --null-input \
    --arg question "$QUESTION" \
    --arg answer "$ANSWER" \
    '.question = $question | .answer = $answer')

  # save json data as review item
  "$SPBASEDCTL_BIN" items add --model "flashcard" --data "$FLASHCARD"
}
edit() {
  if [[ -z $id ]]; then
    echo "usage: flashcard edit <item_id>"
    exit 1
  fi

  # retrieve data
  FLASHCARD=$("$SPBASEDCTL_BIN" items query \
    --pre-filter="id==$id && model=='flashcard'" \
    --post-filter="[0].{id:id,data:data}")

  # extract data
  FLASHCARD_ID=$(jq ".id" <<< "$FLASHCARD")
  FLASHCARD_DATA=$(jq ".data" <<< "$FLASHCARD")
  QUESTION=$(jq -r ".question" <<< $FLASHCARD_DATA)
  ANSWER=$(jq -r ".answer" <<< $FLASHCARD_DATA)

  # promt the user for edits
  QUESTION=$(gum write \
    --placeholder="question" \
    --value="$QUESTION")
  ANSWER=$(gum write \
    --placeholder="answer" \
    --value="$ANSWER")

  ## Both question and answer must be set
  if [[ -z "$QUESTION" || -z "$ANSWER" ]]; then
    echo "must provide non empty input"
    exit 0
  fi

  # build json object
  FLASHCARD=$(jq --compact-output --null-input \
    --arg question $QUESTION \
    --arg answer "$ANSWER" \
    '.question = $question | .answer = $answer')

  # save json object
  "$SPBASEDCTL_BIN" items edit  --data "$FLASHCARD" "$id"
}
review() {
  N_NEW=$("$SPBASEDCTL_BIN" review query-count new)
  N_DUE=$("$SPBASEDCTL_BIN" review query-count due)
  gum log --level info "You have $N_DUE flashcards that are due and $N_NEW that are new"

  if [[ $N_NEW == "0" && $N_DUE == "0" ]]; then
    gum log --level info "No flashcards are new or due. Exiting..."
    exit 0
  fi

  if [[ $N_DUE != "0" ]]; then
    REVIEW_CHOICE="due"
  else
    REVIEW_CHOICE="new"
  fi

  FLASHCARD=$("$SPBASEDCTL_BIN" review next "$REVIEW_CHOICE" \
    --pre-filter="model=='flashcard'" \
    --post-filter="{data:data,id:id,created_at:created_at}")

  if [[ -z $FLASHCARD || $FLASHCARD == "null" ]]; then
    gum log --level error "could not find any $REVIEW_CHOICE item"
    exit 0
  fi

  FLASHCARD_ID=$(jq ".id" <<< "$FLASHCARD")
  FLASHCARD_CREATED_AT=$(jq ".created_at" <<< "$FLASHCARD" | cut -c2-11 )
  FLASHCARD_DATA=$(jq ".data" <<< "$FLASHCARD")

  QUESTION=$(jq -r ".question" <<< "$FLASHCARD_DATA")
  ANSWER=$(jq -r ".answer" <<< "$FLASHCARD_DATA")

  gum log --level info "Card id: $FLASHCARD_ID"

  # show the question
  if [[ $QUESTION != \#* ]]; then 
    glow <<< "# Question"
  fi
  glow <<< "$QUESTION"

  # await response to continue
  gum input --placeholder="Press enter to view answer" --no-show-help

  # show the answer
  if [[ $ANSWER != \#* ]];  then
    glow <<< "# Answer"
  fi
  glow <<< "$ANSWER"

  # prompt for how well it went
  AGAIN="again (could not answer)"
  HARD="hard (could answer with difficulty)"
  GOOD="good (could answer)"
  EASY="easy (could answer easily)"

  RESULT=$(gum choose \
    --header "How easy was it to answer the prompt?" \
    --selected="$GOOD" "$AGAIN" "$HARD" "$GOOD" "$EASY" \
    | cut -d " " -f1) # cut out the first word (again,hard,good,easy) as expected by `spbased review score`

  if [[ -z "$RESULT" ]]; then
    exit 0
  fi

  "$SPBASEDCTL_BIN" review score "$FLASHCARD_ID" "$RESULT"
}

case "$command" in
  ("add") add;;
  ("edit") edit;;
  ("review") review;;
  (*) fatal "Unknown command '$command'";;
esac
