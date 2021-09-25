-- Admin Model

CREATE TABLE IF NOT EXISTS admins (
    id serial PRIMARY KEY,
    username varchar(64) not null,
    display_name text not null,
    contact text not null,
    passwd text not null
);
