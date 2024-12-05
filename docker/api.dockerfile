from rust:1.74-alpine as builder
RUN apk add pkgconfig libressl-dev musl-dev
## Start: Cache Dependencies ##
WORKDIR /app
RUN cargo new --bin api
COPY Cargo.toml /app/
COPY Cargo.lock /app/
COPY api/Cargo.toml /app/api/
RUN cargo build --release
COPY api/src /app/api/src
RUN touch api/src/main.rs
## End: Cache Dependencies ##

## Build Binary ##
RUN cargo build --release


from rust:1.74-alpine
RUN apk add libc6-compat
COPY --from=builder /app/target/release/stockpot /app/stockpot
CMD ["/app/stockpot", "server"]
