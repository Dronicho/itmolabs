FROM rust:1-slim-bookworm AS builder
WORKDIR /code
RUN apt-get update \
    && apt-get install -y pkg-config libssl-dev
RUN apt-get install -y ffmpeg
RUN USER=root cargo init
COPY Cargo.toml Cargo.toml
RUN cargo fetch
ENV SQLX_OFFLINE=true
COPY src src
COPY .sqlx .sqlx
COPY data.db data.db
COPY migrations migrations

RUN cargo build --release

FROM bitnami/minideb:bookworm
WORKDIR /app

COPY --from=builder /code/target/release/server target
COPY --from=builder /code/data.db data.db

RUN chmod a+rw data.db
RUN mkdir tmp_out
RUN mkdir tmp
RUN apt-get update

RUN chmod a+rw tmp
RUN chmod a+rw tmp_out
USER 1001

EXPOSE 8080

CMD [ "/app/target" ]