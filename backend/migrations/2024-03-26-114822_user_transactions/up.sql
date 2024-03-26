CREATE TABLE user_transactions (
                       id serial not null primary key,
                       user_id INTEGER NOT NULL,
                       amount_in_satoshi INTEGER NOT NULL,
                       payment_request VARCHAR(255) NOT NULL,
                       payment_addr VARCHAR(2000) NOT NULL,
                       status INTEGER NOT NULL
);-- Your SQL goes here
