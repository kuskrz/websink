rm install/linux-gnu/* install/linux-musl/* install/windows/*
cargo build --release
RUSTFLAGS="-C target-feature=+crt-static" cargo build --target x86_64-unknown-linux-gnu --release
cargo build --release --target x86_64-unknown-linux-musl
#cargo build --release --target x86_64-pc-windows-gnu
mv target/release/websink install/linux-gnu/
mv target/x86_64-unknown-linux-gnu/release/websink install/linux-gnu/websink_gnu_static
mv target/x86_64-unknown-linux-musl/release/websink install/linux-musl/websink_musl_static
