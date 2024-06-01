# Rust Web Example
**Nachiket More (nmore@pdx.edu)** 

This is the repo for homeworks for TOP: Rust Web Development Spring 2024


## Homework - Persistent Data

This is a CRUD API built using Rust and the Axum web framework for questions with Postgress database connection for Persistent Data.

## Features
- Retrieve a list of all questions in the database
- Get a specific question by its ID
- Create a new question
- Delete an existing question
- Update an existing question

## Project Structure
- main.rs: The entry point of the application, sets up the server and starts the Axum application.
- routes.rs: Defines the routes for the API and maps them to the corresponding handler functions.
- handler.rs: Implements the logic for handling the API requests
- database.rs: Manages the database of questions, including seeding the database
- .env: Contains database url for conenction to the postgress instance

## Features
### Prerequisites
- PostgreSQL installed on your system. You can download it from here https://www.postgresql.org/download/

### Database setup
Database Creation:
```
CREATE DATABASE questions-db;
```
Creating Table:

execute the SQL commands below to create the table
```
CREATE TABLE questions (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    tags TEXT[] NOT NULL
);
```
Seeding the data:

we can insert some initial data into the table by running following command, 
```
INSERT INTO questions (title, content, tags) VALUES ('How?', 'Please help!', '{"general"}');
```


## Usage
Run the server
```
cargo run
```

Access the API endpoints:
- Health check: GET /
- Get all questions: GET /questions
- Get a question by ID: GET /question/:id
- Create a question: POST /question with a JSON payload
- Delete a question: DELETE /question/:id
- Update a question: PATCH /question/:id with a JSON payload

## References

 - https://docs.rs/axum/latest/axum/
 - https://github.com/wpcodevo/simple-api-rust-axum/blob/master/src/handler.rs
 - https://codevoweb.com/create-a-simple-api-in-rust-using-the-axum-framework/
 - https://rust-lang-nursery.github.io/rust-cookbook/database/postgres.html