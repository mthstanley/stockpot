from rust:1.74-alpine as builder
RUN apk add pkgconfig libressl-dev musl-dev
## Start: Cache Dependencies ##
RUN cargo new --bin app
WORKDIR /app
COPY Cargo.toml /app/
COPY Cargo.lock /app/
RUN cargo build --release
COPY src /app/src
RUN touch src/main.rs
## End: Cache Dependencies ##

## Build Binary ##
RUN cargo build --release


from rust:1.74-alpine
RUN apk add libc6-compat
COPY --from=builder /app/target/release/stockpot /app/stockpot
CMD ["/app/stockpot", "server"]
