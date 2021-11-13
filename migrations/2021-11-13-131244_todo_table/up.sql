-- Your SQL goes here
CREATE TABLE todo (
    id SERIAL PRIMARY KEY,
    title VARCHAR,
    body TEXT,
    created_at TIMESTAMP WITH TIME ZONE,
    updated_at TIMESTAMP WITH TIME ZONE
);