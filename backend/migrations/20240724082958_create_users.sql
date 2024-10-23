create table if not exists users (
    id serial primary key,
    username varchar(255) not null unique,
    password_hash varchar(255) not null,
    created_at timestamp not null default now(),
    updated_at timestamp not null default now()
);

alter table todos add column user_id integer;

-- create the admin user and set a string as password, this will never match as the columns will contain hashes
insert into users (username, password_hash) values ('preexisting', 'will not be able to log in');
update todos set user_id = (select id from users where username = 'admin');

alter table todos alter column user_id set not null;
alter table todos add foreign key (user_id) references users(id) on delete cascade;