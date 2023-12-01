#!/bin/sh

./bin/sea-orm-cli generate entity\
  --with-serde=both\
  --output-dir="./entity/src/"\
  --lib

