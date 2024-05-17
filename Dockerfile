FROM messense/rust-musl-cross:x86_64-musl AS chef
RUN cargo install cargo-chef
WORKDIR /build

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /build/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:latest AS runner
WORKDIR /app
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/economic app
ENTRYPOINT "./app"

