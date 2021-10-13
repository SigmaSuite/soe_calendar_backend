FROM rust:1.55.0

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080

WORKDIR /app
COPY . .

RUN rustup default nightly
RUN cargo install cargo-watch
RUN cargo build

CMD ["cargo", "watch", "-x", "run"]