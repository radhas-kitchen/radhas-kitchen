-- change enum values of userkind

alter type userkind 
rename value 'Restaurant' to 'Provider';

alter type userkind
rename value 'Farm' to 'Consumer';