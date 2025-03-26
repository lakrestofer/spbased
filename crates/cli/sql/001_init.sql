--- ============================ item ============================
--- content agnostic container for scheduling data and model data.
create table item (
    id integer primary key,
    maturity text not null default "new",
    stability real not null default 0.0,                      -- sra parameter. the number of days since last review date until probability of recal reaches 90%
    difficulty real not null default 0.0,                     -- sra parameter. number between 1 and 10. meausure of item difficulty
    last_review_date text not null default current_timestamp, -- sra parameter. date in iso8601
    n_reviews integer not null default 0,                     -- sra parameter. number of times we've review and given the review a score.
    n_lapses integer not null default 0,                      -- sra parameter. number of times we've failed to recall the item.
    model text not null,                                      -- the model, tells us how data is to be interpreted
    data text not null,                                       -- untyped text field, usually json data
    updated_at text not null default current_timestamp,       -- metadata
    created_at text not null default current_timestamp        -- metadata
);
--- keep update_at column in sync
create trigger
update_at_field_trigger__item
after update on
item
when old.maturity <> new.maturity or
    old.stability <> new.stability or
    old.difficulty <> new.difficulty or
    old.last_review_date <> new.last_review_date or
    old.n_reviews <> new.n_reviews or
    old.model <> new.model or
    old.data <> new.data
begin
    update item set updated_at = datetime('now') where id == old.id;
end;
--- --------------------------------------------------------------------------


--- ============================ tag ============================
create table tag (
    id integer primary key,
    name text not null unique,                          -- name of tag
    updated_at text not null default current_timestamp, -- metadata
    created_at text not null default current_timestamp  -- metadata
);
-- keep update_at field in sync
create trigger
update_at_field_trigger__tag
after update on
tag
when old.name <> new.name
begin
    update item set updated_at = datetime('now') where id == old.id;
end;
create table tag_item_map (
    id integer primary key,
    tag_id integer not null,
    item_id integer not null,
    created_at text not null default current_timestamp, -- metadata
    foreign key(tag_id) references tag(id) on delete cascade,
    foreign key(item_id) references item(id) on delete cascade,
    unique(tag_id, item_id)
);
--- --------------------------------------------------------------------------


--- ============================ due items ============================
create view due_item as
select
    *
from
    item
where
    maturity != "new" and
    date(last_review_date, '+' || stability || ' days') < date('now')
order by
    stability asc;
--- --------------------------------------------------------------------------


--- ============================ new items ============================
create view new_item as
select
    *
from
    item
where
    maturity == "new"
order by
    random();
--- --------------------------------------------------------------------------
