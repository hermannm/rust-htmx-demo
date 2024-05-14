CREATE TABLE
    todos (
        id SERIAL PRIMARY KEY,
        author VARCHAR(255) NOT NULL,
        content TEXT NOT NULL
    );