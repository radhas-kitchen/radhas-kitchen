-- Add migration script here

alter table jobs
drop pickup_location,
drop dropoff_location,
drop confirm_time,
drop status;

drop type jobstatus;
create type jobstatus as enum (
	'Pending',
	'Claimed',
	'PickedUp', 
	'DroppedOff', 
	'Cancelled'
);

alter table jobs
rename column pickup_time to pickedup;

alter table jobs
rename column dropoff_time to droppedoff;

alter table jobs
add column cancelled timestamp default null,
add column claimed timestamp default null,
add column status jobstatus not null default 'Pending';