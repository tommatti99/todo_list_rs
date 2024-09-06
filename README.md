# Rust Todo List

## About

It's a todo list written entirely in rust.

Its used Rust Yew for frontend and Rust Rocket for Backend.

### Backend
  - 1º. Start the Postgresql at your machine and set the database link at .env file, seems something like "DATABASE_URL=postgres://username:password@localhost/diesel_demo".

  - 2º. install Rust Diesel cli and starts diesel schema:
  ```
  cargo install diesel_cli
  cargo install diesel_cli --no-default-features --features postgres
  diesel setup
  diesel migration run
  ```

### Frontend
