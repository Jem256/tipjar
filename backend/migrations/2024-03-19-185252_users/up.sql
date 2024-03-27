CREATE TABLE users (
                       id serial not null primary key,
                       email VARCHAR(255) NOT NULL,
                       name VARCHAR(255) NOT NULL,
                       password VARCHAR(255) NOT NULL,
                       slug VARCHAR(255) NOT NULL,
                       balance VARCHAR(255) NOT NULL
);

INSERT INTO users (email, name,password, slug, balance)
VALUES
    ('user1@example.com','user1', 'password123', 'user1_slug', 100),
    ('user2@example.com','user2', 'pass456', 'user2_slug', 20),
    ('user3@example.com', 'user3','securepass', 'user3_slug', 50),
    ('user4@example.com', 'user4','12345pass', 'user4_slug', 200);
