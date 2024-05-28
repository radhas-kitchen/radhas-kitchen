-- alter fields id of table users and tokens to be non-null

alter table users
alter column id set not null;

alter table tokens
alter column id set not null,
alter column user_id set not null;