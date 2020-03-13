CREATE TABLE channel (
    id serial,
    name varchar(100) not null,
    last_message varchar(100),
    last_modified timestamp,
    primary key(id));

CREATE TABLE user (
    id serial,
    name varchar(100) not null,
    primary key(id));

CREATE TABLE message (
    id serial,
    sender int not null,
    channel_id int not null,
    type varchar(32) not null,
    body varchar(250) not null,
    sent timestamp not null,
    primary key(id));
