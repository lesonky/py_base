-- Add migration script here
ALTER TABLE role ADD COLUMN name VARCHAR(255) NOT NULL UNIQUE
