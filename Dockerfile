FROM rust:1.70 as builder

WORKDIR /app

# Copy the project files
COPY . .

RUN cargo build --release

# Use a minimal base image for the final container
FROM debian:buster-slim

WORKDIR /app

COPY --from=builder /app/target/release/lambert_w_function .

CMD ["./lambert_w_function"]