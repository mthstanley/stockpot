from rust:1.74-alpine as builder
WORKDIR /build
RUN apk add pkgconfig musl-dev libressl-dev
RUN cargo install sqlx-cli
RUN cp $CARGO_HOME/bin/sqlx .


from rust:1.74-alpine
RUN apk add libc6-compat
WORKDIR /app
COPY --from=builder /build/sqlx sqlx
