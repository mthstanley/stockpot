from rust:1.83-alpine
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

CMD ["cargo", "test", "--test", "*", "--", "--nocapture"]
