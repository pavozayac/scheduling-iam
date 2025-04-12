create table users (
    id uuid primary key,
    email varchar(255) not null unique,
    registered boolean not null default false,
    date_created timestamp not null default current_timestamp,
    date_updated timestamp not null default current_timestamp,
    deleted timestamp default null
);

create table recovery_codes (
    user_id uuid not null references users (id),
    code varchar(255) not null,
    date_created timestamp not null default current_timestamp,
    date_updated timestamp not null default current_timestamp,
    deleted timestamp default null
);

create table otps (
    id uuid primary key,
    user_id uuid not null references users (id),
    code varchar(255) not null,
    expires_at timestamp not null,
    date_created timestamp not null default current_timestamp,
    date_updated timestamp not null default current_timestamp,
    deleted timestamp default null
);
