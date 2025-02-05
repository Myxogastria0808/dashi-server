ARG APP_NAME=presentation

FROM rust:1.81.0 AS builder
ARG APP_NAME

WORKDIR /app
COPY . /app

RUN touch /app/.env \
    && chmod +x /app/.env \
    && cargo build --release --bin $APP_NAME

ENTRYPOINT ["/app/target/release/presentation"]
