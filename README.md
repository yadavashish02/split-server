# Split Server

A Rust-based GraphQL Splitwise clone server. Built using clean Domain-Driven Design (DDD) principles.

## 🚀 Features
- **Domain-Driven Design**: Clean separation of Domain interfaces and Repository implementations.
- **GraphQL API**: Powered by `async-graphql` and `axum`.
- **Database integration**: SQLite via `sqlx` with automatic migration runs on server startup.
- **Docker Support**: Ready for containerized deployment with `docker-compose`.

## 🛠️ Technology Stack
- **Core Language**: Rust (Edition 2024)
- **Web Framework**: [Axum](https://github.com/tokio-rs/axum)
- **GraphQL Library**: [async-graphql](https://github.com/async-graphql/async-graphql)
- **Database Engine & ORM/Query Builder**: [SQLx](https://github.com/launchbadge/sqlx) (SQLite driver)
- **Runtime**: [Tokio](https://github.com/tokio-rs/tokio)
## ⚙️ Getting Started

### Prerequisites
- **Rust**: Install Rust via [rustup](https://rustup.rs/) (edition 2024 compatible)
- **SQLx CLI** (Optional, for managing migrations):
  ```bash
  cargo install sqlx-cli --no-default-features --features sqlite
  ```

### Local Setup

1. **Clone the repository**:
   ```bash
   git clone <your-repo-url>
   cd split-server
   ```

2. **Set up Environment Variables**:
   Copy the example environment file and configure it if needed:
   ```bash
   cp .env.example .env
   ```

3. **Database migrations**:
   The server runs migrations automatically on startup. If you want to run them manually:
   ```bash
   sqlx database setup
   ```

4. **Run the server**:
   ```bash
   cargo run
   ```

### 🐳 Running with Docker

You can spin up the application in a Docker container using:

```bash
docker-compose up --build
```
