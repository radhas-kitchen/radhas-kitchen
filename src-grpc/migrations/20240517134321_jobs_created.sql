-- Add migration script here
alter type jobstatus
rename value 'Pending' to 'Created';

alter table jobs
add column created timestamp not null default now();