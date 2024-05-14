-- Add migration script here
alter table users
add column name text not null default '';