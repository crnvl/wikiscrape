FROM rust:latest

WORKDIR /usr/src
COPY . . 

RUN cargo build -p scrape --release

CMD ["./target/release/scrape"]