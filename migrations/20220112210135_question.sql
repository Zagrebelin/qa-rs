-- Add migration script here
create table if not exists questions (
    id serial primary key ,
    header text not null ,
    body text not null,
    created timestamp with time zone not null default now(),
    updated timestamp with time zone not null default now()
);

create trigger question_updated before insert or update on questions
    for each row execute procedure update_timestamp();
