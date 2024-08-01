#!/bin/sh

read -r -p "name of migration: " name
sea-orm-cli  migrate generate --migration-dir crates/migration $name
