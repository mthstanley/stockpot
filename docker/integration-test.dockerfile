from rust:1.74-alpine
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

CMD ["cargo", "test", "--test", "*"]
