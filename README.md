## Creating a REST API in Rust using Rocket and Diesel with Postgres

### Prerequisites

- Install [Rustup](*rustup* is an installer for
  the systems programming language [Rust](https://www.rust-lang.org/)), After this, you can use the `rustup` command to also install  `nightly` channels for Rust and Cargo.

  ```
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- Install Postgres

### API Endpoints

- [Show Accessible Products]() : `GET /products/`
- [Create New Product]() : `POST /products/`
- [Update Products]() : `PUT /products/<id>`
- [Delete Product]() : `DELETE /products/<id>`

### Project Setup

**Dependencies**

Let’s take a moment to review the libraries that we’ll be using.

- **[Rocket](https://rocket.rs/)**: Rocket is a web framework for Rust that makes it simple to write *fast*, secure web applications without sacrificing flexibility, usability, or type safety.
- **[Diesel](https://diesel.rs/)**: Diesel is a Safe, Extensible ORM and Query Builder for Rust
- **[r2d2](https://github.com/sfackler/r2d2)**: A generic database connection pool for Rust.
- **[Serde](https://serde.rs/)**: Serde is a framework for **ser**ializing and **de**serializing Rust data structures efficiently and generically.

**Project Structure**

```
|-- Cargo.lock
|-- Cargo.toml
|-- README.md
|-- diesel.toml
|-- migrations
|   |-- 00000000000000_diesel_initial_setup
|   |   |-- down.sql
|   |   `-- up.sql
|   `-- 2020-11-07-181730_create_products
|       |-- down.sql
|       `-- up.sql
`-- src
    |-- connection.rs
    |-- main.rs
    |-- product
    |   |-- handler.rs
    |   |-- mod.rs
    |   |-- repository.rs
    |   `-- router.rs
    `-- schema.rs
```

**Database**

- Set Database Url into .env:  `DATABASE_URL=postgres://<username>:<password>@localhost/rust-rest-api`
- `diesel setup`
- `diesel migration generate create_products`
- `diesel migration run`

**Server**

- Set below env vars into .env

  ```
  ROCKET_ADDRESS=localhost
  ROCKET_PORT=8000
  ```

- Let’s fire up the server with `cargo run`
