-- starting from scratch
create type userkind as enum ('Driver', 'Restaurant', 'Farm');

create table users (
	id uuid primary key default gen_random_uuid(),
	email text not null unique,
	password text not null,
	salt text not null default gen_salt('bf'),
	created timestamp not null default now(),
	kind userkind not null
);

create table tokens (
	id uuid primary key default gen_random_uuid(),
	user_id uuid references users(id) on delete cascade,
	token text not null default gen_random_uuid(),
	created timestamp not null default now(),
	expires timestamp not null default now() + interval '7 days'
);