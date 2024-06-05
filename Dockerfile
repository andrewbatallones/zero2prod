FROM rust:1.78.0

WORKDIR /app

RUN apt update && apt install lld clang -y

COPY . .

ENV APPP_ENVIRONMENT production

RUN cargo build --release

ENTRYPOINT ["./target/release/zero2prod"]
