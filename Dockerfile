FROM rust:1.47

WORKDIR /app
COPY . /app

RUN cargo install --path .