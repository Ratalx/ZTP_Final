-- Your SQL goes here
create table orders (
    id serial primary key,
    web_user_login varchar(255) not null,
    fruit_id int not null,
    order_date timestamp not null default CURRENT_TIMESTAMP
);

create table fruits (
    id serial primary key,
    name varchar(255) not null,
    in_stock boolean not null default false
);

create table users (
    login varchar(255) primary key,
    password varchar(255) not null,
    admin_user boolean not null default false
);

create table favourites (
    login varchar(255),
    fruit_id int not null,
    CONSTRAINT favourite_pkey PRIMARY KEY (login, fruit_id)
);