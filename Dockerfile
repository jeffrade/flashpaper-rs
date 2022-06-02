FROM rust:1.61.0 AS builder
WORKDIR /usr/src/github.com/jeffrade/flashpaper-rs/
COPY src/ src/
COPY static/ static/
COPY Cargo.toml .
RUN cargo build --release

FROM debian:buster-slim
ARG APP=/usr/src/app
RUN apt-get update \
  && apt-get -y install ca-certificates \
  libssl-dev \
  sqlite3 \
  libsqlite3-dev
RUN mkdir -p ${APP}
COPY --from=builder /usr/src/github.com/jeffrade/flashpaper-rs/target/release/flashpaper-rs ${APP}/flashpaper-rs
WORKDIR ${APP}
EXPOSE 8321
CMD ["./flashpaper-rs"]
