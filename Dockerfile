FROM rust:latest as builder

WORKDIR /app

# Copy the project files
COPY . .

RUN rm -f Cargo.lock && cargo build --release

# Use a minimal base image for the final container
FROM debian:buster-slim

WORKDIR /app

COPY --from=builder /app/target/release/lambert_w_function .

CMD ["./lambert_w_function"]