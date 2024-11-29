from rust:1.74-alpine
RUN apk add pkgconfig libressl-dev musl-dev
## Start: Cache Dependencies ##
RUN cargo new --bin api
WORKDIR /api
COPY Cargo.toml /api/
COPY Cargo.lock /api/
RUN cargo build --release
COPY src /api/src
RUN touch src/main.rs
## End: Cache Dependencies ##

CMD ["cargo", "test", "--lib"]
