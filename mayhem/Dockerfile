FROM --platform=linux/amd64 rustlang/rust:nightly as builder

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update 
RUN apt-get install -y cmake clang
RUN cargo install cargo-fuzz

ADD . /daachorse/

WORKDIR /daachorse/fuzz/

RUN cargo +nightly fuzz build

FROM --platform=linux/amd64 rustlang/rust:nightly

COPY --from=builder /daachorse/fuzz/target/x86_64-unknown-linux-gnu/release/initialize_ac /

COPY --from=builder /daachorse/fuzz/target/x86_64-unknown-linux-gnu/release/find_overlapping /