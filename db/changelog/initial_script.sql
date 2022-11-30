CREATE DATABASE db_anothergtw encoding 'UTF-8';
CREATE USER anothergtw_user WITH PASSWORD 'yNJG7y52S7tkn3W&';
GRANT ALL PRIVILEGES ON DATABASE db_anothergtw TO anothergtw_user;

-- access the db_anothergtw with postgres and run commands bellow.
CREATE EXTENSION unaccent;
CREATE EXTENSION pg_trgm;