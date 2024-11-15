# syntax=docker/dockerfile:1.3-labs

# The above line is so we can use can use heredocs in Dockerfiles. No more && and \!
# https://www.docker.com/blog/introduction-to-heredocs-in-dockerfiles/

FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

# Build recipe
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin crustchan-api


FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /app/target/release/crustchan-api /usr/local/bin
ENTRYPOINT ["/usr/local/bin/crustchan-api"]

EXPOSE 3000