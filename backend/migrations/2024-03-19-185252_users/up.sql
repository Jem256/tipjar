CREATE TABLE users (
                       id serial not null primary key,
                       email VARCHAR(255) NOT NULL,
                       password VARCHAR(255) NOT NULL,
                       slug VARCHAR(255) NOT NULL,
                       balance DECIMAL(255)
);

INSERT INTO users (email, password, slug, balance)
VALUES
    ('user1@example.com', 'password123', 'user1_slug', 100.00),
    ('user2@example.com', 'pass456', 'user2_slug', NULL),
    ('user3@example.com', 'securepass', 'user3_slug', 50.75),
    ('user4@example.com', '12345pass', 'user4_slug', 200.50);
