# rust-async-api

This project is a small backend service built in Rust using Axum and Tokio.

The goal of this project was to explore:
- async request handling in Rust
- shared state management using Arc and Mutex
- building simple REST APIs with Axum

This is part of my transition into systems-level backend development and learning Rust.This project is a small backend service built in Rust using Axum and Tokio.

The goal of this project was to explore:
- async request handling in Rust
- shared state management using Arc and Mutex
- building simple REST APIs with Axum

This is part of my transition into systems-level backend development and learning Rust.

## Features

- Health check endpoint (`GET /health`)
- Create user endpoint (`POST /users`)
- In-memory data storage
- Async request handling

## Tech Stack

- Rust
- Axum (web framework)
- Tokio (async runtime)
- Serde (JSON serialization)

## Run Locally

```bash
cargo run
```


## Notes

This project focuses on understanding Rust’s async model and differences from traditional JVM-based backend systems, particularly around memory safety and concurrency.