rust_sources := src/*.rs
#rust_sources := src/*.rs anotherdir/*.rs
cargo := Cargo.toml Cargo.lock

install: copy.timestamp
	@echo "Done"

.SILENT:

copy.timestamp: target/release/websink target/x86_64-unknown-linux-musl/release/websink target/x86_64-pc-windows-gnu/release/websink.exe
	touch copy.timestamp

target/release/websink: $(rust_sources) $(cargo)
	cargo build --release
	cp target/release/websink install/linux-gnu
	echo "Build for GNU Linux"

target/x86_64-unknown-linux-musl/release/websink: $(rust_sources) $(cargo)
	cargo build --release --target x86_64-unknown-linux-musl
	cp target/x86_64-unknown-linux-musl/release/websink install/linux-musl
	echo "Build for MUSL Linux"

target/x86_64-pc-windows-gnu/release/websink.exe: $(rust_sources) $(cargo)
	cargo build --release --target x86_64-pc-windows-gnu
	cp target/x86_64-pc-windows-gnu/release/websink.exe install/windows
	echo "Build for Windows"

clean:
	cargo clean
	if [ -f install/linux-gnu/websink ]; then rm install/linux-gnu/websink; fi
	if [ -f install/linux-musl/websink ]; then rm install/linux-musl/websink; fi
	if [ -f install/windows/websink.exe ]; then rm install/windows/websink.exe; fi
	echo "Cleaned"