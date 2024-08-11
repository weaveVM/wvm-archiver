FROM rust:1.73

COPY ./ ./

RUN cargo build --release

CMD ["./target/release/wvm-archiver"]