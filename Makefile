build:
	cargo build --release
	cargo build --release --target x86_64-pc-windows-gnu

install: build
	cp target/release/websink install/
	cp target/x86_64-pc-windows-gnu/release/websink.exe install/