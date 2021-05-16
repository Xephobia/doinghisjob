FROM rust:latest

RUN cargo build --release

CMD ["./target/release/doinghisjob"]
