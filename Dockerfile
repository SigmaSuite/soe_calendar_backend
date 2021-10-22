FROM rustlang/rust:nightly-buster-slim@sha256:b89eb6862cbd85ed084717cc5008641900171f7d4e94134d8d7945de5f7dc848

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080
RUN apt update -y && apt install -y libpq-dev

WORKDIR /app
COPY ./src ./src
COPY ./migrations ./migrations
COPY Cargo.toml .
COPY .env .

RUN cargo build

# debug
# CMD ["cargo", "watch", "-x", "run"]
CMD ["cargo", "run"]