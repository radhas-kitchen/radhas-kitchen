create type jobstatus as enum (
	'Pending', 
	'PickedUp', 
	'DroppedOff', 
	'Confirmed', 
	'Cancelled'
);

create table jobs (
	id text primary key not null default gen_random_uuid()::text,

	provider text not null references users(id) on delete cascade,	
	consumer text not null references users(id) on delete cascade,
	driver text references users(id) on delete set null default null,
	
	pickup_location text not null,
	dropoff_location text not null,

	pickup_time timestamp default null,
	dropoff_time timestamp default null,
	confirm_time timestamp default null,

	status jobstatus not null default 'Pending'
);