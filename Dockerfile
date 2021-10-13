FROM rustlang/rust:nightly-buster-slim

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080
RUN apt update -y && apt install -y libpq-dev

WORKDIR /app
COPY ./src ./src
COPY Cargo.toml .
COPY .env .

RUN cargo build

# debug
# CMD ["cargo", "watch", "-x", "run"]
CMD ["cargo", "run"]