create table drivers (
	id text primary key not null references users(id) on delete cascade
);

create table providers (
	id text primary key not null references users(id) on delete cascade,
	location text not null
);

create table consumers (
	id text primary key not null references users(id) on delete cascade,
	location text not null
);