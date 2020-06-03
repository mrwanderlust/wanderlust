FROM rust:1.43.1 as rust

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

FROM node:current-stretch as npm

WORKDIR /opt/wanderlust/app
COPY app .
RUN npm install
RUN npm run webpack

FROM rust as entrypoint
COPY --from=npm /opt/wanderlust/app /opt/wanderlust/app
CMD ["cargo", "run", "--release"]
