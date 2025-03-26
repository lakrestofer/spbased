PRAGMA journal_mode = wal; -- different implementation of the atomicity properties
PRAGMA synchronous = normal; -- synchronise less often to the filesystem
PRAGMA foreign_keys = on; -- check foreign key reference, slightly worst performance

