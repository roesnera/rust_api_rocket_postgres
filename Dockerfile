FROM rust:1.73

WORKDIR /app

COPY . .

RUN rustup default

RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install cargo-watch
RUN cargo install sccache --locked

ENV RUSTC_WRAPPER=/usr/local/cargo/bin/sccache

CMD ["cargo", "watch", "--why", "--", "echo"]