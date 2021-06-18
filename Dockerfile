FROM rust:1.48 AS builder

WORKDIR /app

COPY Cargo.toml Cargo.toml
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY ./src ./src
COPY ./templates ./templates
COPY ./static ./static

RUN rm -f target/release/deps/diff_visualize*
RUN cargo build --release

FROM debian:10.4
COPY --from=builder /diff_visualize/target/release/diff_visualize /usr/local/bin/diff_visualize
CMD ["diff_visualize"]
