#!/bin/sh

if [[ -z "$1" ]]
  then
    echo "usage: ./scripts/create_migration.sh 'migration name'"; 
    exit 1
fi

./bin/sea-orm-cli migrate generate $1
