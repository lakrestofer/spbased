# Description of how spbased is built

This document contains some loselly structured notes on how spbased is built. In its current state, it serves
as a scratchpad for ideas on how to build it.

## configuration

where is the db?

- read path from args
- check for .spbased directory in current directory
- read from environment variable SPBASED_DB_DIR
- read from default directory
