-- drop fkey constraint to allow changing types
alter table tokens
drop constraint tokens_user_id_fkey;

-- change the type in users
alter table users
alter column id type text using id::text,
alter column id set default gen_random_uuid()::text;

-- change the type in tokens
alter table tokens
alter column user_id type text using user_id::text;

-- bring back fkey constraint
alter table tokens
add constraint tokens_user_id_fkey
foreign key (user_id) references users(id) on delete cascade;

-- change misc fields in other tables
alter table tokens
alter column id type text using id::text,
alter column id set default gen_random_uuid()::text,
alter column token type text using token::text,
alter column token set default gen_random_uuid()::text;