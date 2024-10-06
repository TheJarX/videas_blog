CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    content TEXT NOT NULL,
    slug VARCHAR NOT NULL,
    tags TEXT[] NOT NULL, -- because of diesel recommendation using TEXT[] instead of VARCHAR[]
    description TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)