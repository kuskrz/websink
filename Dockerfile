FROM rust:1.82.0-slim-bullseye as build
WORKDIR /websink
COPY . .
RUN RUSTFLAGS="-C target-feature=+crt-static" cargo build --target x86_64-unknown-linux-gnu --release
FROM scratch
WORKDIR /app
COPY --from=build /websink/target/x86_64-unknown-linux-gnu/release/websink .
USER 1000:1000
CMD ["./websink"]