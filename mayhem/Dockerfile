FROM --platform=linux/amd64 rustlang/rust:nightly as builder

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update 
RUN apt-get install -y cmake clang
RUN cargo install cargo-fuzz

ADD . /barcoders/

WORKDIR /barcoders/fuzz/

RUN cargo +nightly fuzz build

FROM --platform=linux/amd64 rustlang/rust:nightly

COPY --from=builder /barcoders/fuzz/target/x86_64-unknown-linux-gnu/release/generate /

COPY --from=builder /barcoders/fuzz/target/x86_64-unknown-linux-gnu/release/ean13 /

COPY --from=builder /barcoders/fuzz/target/x86_64-unknown-linux-gnu/release/ean8 /

COPY --from=builder /barcoders/fuzz/target/x86_64-unknown-linux-gnu/release/code11 /

COPY --from=builder /barcoders/fuzz/target/x86_64-unknown-linux-gnu/release/code39 /

COPY --from=builder /barcoders/fuzz/target/x86_64-unknown-linux-gnu/release/code93 /