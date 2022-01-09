CREATE TABLE users(
    id         SERIAL PRIMARY KEY,
    password   VARCHAR(50) NOT NULL
);

CREATE TABLE messages(
    id         SERIAL PRIMARY KEY,
    author     INT4 REFERENCES users (id),
    text       VARCHAR(100) NOT NULL,
    created    INT8 NOT NULL,
    modified   INT8
);
