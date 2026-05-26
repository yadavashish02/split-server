# Stage 1: Compute a recipe file for dependencies
FROM rust:1.80-slim-bookworm AS chef
# We only pay the installation cost once, it will be cached from the second build onwards
RUN cargo install cargo-chef
WORKDIR /usr/src/app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage 2: Build dependencies and the application
FROM chef AS builder
COPY --from=planner /usr/src/app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release

# Stage 3: Create the tiny final runtime image
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
# Create the directory that will be used as a mount point for the SQLite volume
RUN mkdir -p /app/data

# Copy binary from builder
COPY --from=builder /usr/src/app/target/release/split-server /app/split-server

# Set the database URL to point to the volume mount point
ENV DATABASE_URL=sqlite:///app/data/split.db
EXPOSE 8000

CMD ["/app/split-server"]
