FROM rust:1.90.0-alpine3.22 AS builder
RUN apk add --no-cache musl-dev
WORKDIR /usr/src/pg-tempest
COPY . .
RUN cargo build -r

FROM alpine:3.22.2
WORKDIR /pg-tempest
COPY ./pg-tempest-configs.docker.toml ./pg-tempest-configs.toml
COPY --from=builder /usr/src/pg-tempest/target/release/pg-tempest .
EXPOSE 8000
CMD ["./pg-tempest"]