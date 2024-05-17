-- Add migration script here
alter table jobs
drop constraint if exists jobs_provider_fkey,
drop constraint if exists jobs_consumer_fkey,
drop constraint if exists jobs_driver_fkey;

alter table jobs
add constraint jobs_provider_fkey foreign key (provider) references providers(id) on delete cascade,
add constraint jobs_consumer_fkey foreign key (consumer) references consumers(id) on delete cascade,
add constraint jobs_driver_fkey foreign key (driver) references drivers(id) on delete set null;