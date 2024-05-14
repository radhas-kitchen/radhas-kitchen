-- set primary key of tokens to token
alter table tokens
drop constraint tokens_pkey,
drop column id,
add primary key (token);