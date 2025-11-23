FROM rust:1.91.1-alpine3.22 AS builder
RUN apk add --no-cache musl-dev
WORKDIR /usr/src/pg-tempest
COPY . .
RUN cargo build -r

FROM alpine:3.22.2
WORKDIR /pg-tempest
COPY pg-tempest.defaults.toml ./pg-tempest.defaults.toml
COPY --from=builder /usr/src/pg-tempest/target/release/pg-tempest .
EXPOSE 8000
ENV PG_TEMPEST_SERVER_IPV4=0.0.0.0
CMD ["./pg-tempest"]