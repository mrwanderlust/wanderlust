FROM rust:1.43.1

WORKDIR /opt/wanderlust
EXPOSE 3030

COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN cargo build --release

RUN rm -f target/release/deps/wanderlust*

COPY src src

RUN cargo build --release

CMD ["cargo", "run", "--release"]
