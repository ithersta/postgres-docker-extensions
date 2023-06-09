FROM rust AS build-rust
WORKDIR /usr/src/dump
COPY dump .
RUN cargo install --path .

FROM postgres
COPY --from=build-rust /usr/local/cargo/bin/dump /usr/local/bin/dump
EXPOSE 8080
RUN sed -i '2idump &' /usr/local/bin/docker-entrypoint.sh