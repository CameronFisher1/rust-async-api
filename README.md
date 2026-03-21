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
- Get all users endpoint (`GET /users`)
- Get user by ID endpoint (`GET /users/:id`)
- Update user endpoint (`PUT /users/:id`)
- Delete user endpoint (`DELETE /users/:id`)
- In-memory state management using `Arc<Mutex<HashMap<...>>>`
- UUID-based user IDs
- Structured JSON error responses
- Async request handling with Axum + Tokio
- Input validation for user creation and updates

  
## Tech Stack

- Rust
- Axum (web framework)
- Tokio (async runtime)
- Serde (JSON serialization)
- UUID (user ID generation)

## Run Locally

Clone the repository and start the server:

```bash
git clone https://github.com/CameronFisher1/rust-async-api.git
cd rust-async-api
cargo run
```

The server runs locally at `http://127.0.0.1:8080`

## API Endpoints

### Health
- `GET /health`  
  Returns a simple health check response.

### Users
- `POST /users`  
  Creates a new user.

- `GET /users`  
  Returns all users.

- `GET /users/:id`  
  Returns a single user by UUID.

- `PUT /users/:id`  
  Updates an existing user.

- `DELETE /users/:id`  
  Deletes a user.

## Example Request

### Create User
```json
{
  "name": "Cameron",
  "description": "Backend engineer learning Rust"
}
```

## Example Response
```json
{
  "id": "generated-uuid",
  "name": "Cameron",
  "description": "Backend engineer learning Rust"
}
```

## Error Responses
```json
{
  "error": "Invalid input"
}
```

## Design Notes

This project uses:
- `Axum` for HTTP routing and handler extraction
- `Tokio` as the async runtime
- `Arc<Mutex<...>>` for shared in-memory application state
- `HashMap<String, User>` for efficient lookup by user ID
- `UUID` values for unique user identifiers
- Structured JSON error responses for invalid input and missing resources

This project is intentionally in-memory to keep the focus on learning Rust backend fundamentals such as async handlers, shared state, request validation, and API design.

## Notes

This project focuses on understanding Rust’s async model and differences from traditional JVM-based backend systems, particularly around memory safety and concurrency.
