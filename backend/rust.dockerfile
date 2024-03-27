FROM rust:1.77.0 as builder

WORKDIR /app

ARG DATABASE_URL

ENV DATABASE_URL=$DATABASE_URL

COPY . .

# Install protobuf-compiler
RUN apt-get update \
    && apt-get install -y protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

# Compile the Rust project
RUN cargo build --release


FROM debian:buster-slim

WORKDIR /usr/local/bin

# Copy the built executable from the builder stage
COPY --from=builder /app/target/release/tipjarbackend .

CMD ["./tipjarbackend"]
