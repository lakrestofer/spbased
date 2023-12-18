#!/bin/sh

./bin/sea-orm-cli generate entity\
  --with-serde=both\
  --output-dir="./crates/entity/src/"\
  --date-time-crate="chrono"\
  --expanded-format\
  --lib

