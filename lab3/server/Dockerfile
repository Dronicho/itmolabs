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

# copy server binary from build stage
COPY --from=builder /code/target/release/server target
COPY --from=builder /code/data.db data.db

RUN chmod a+rw data.db
# set user to non-root unless root is required for your app
RUN mkdir tmp_out
RUN mkdir tmp
RUN apt-get update
RUN apt-get install -y ffmpeg

RUN chmod a+rw tmp
RUN chmod a+rw tmp_out
USER 1001

# indicate what port the server is running on
EXPOSE 8080

# run server
CMD [ "/app/target" ]