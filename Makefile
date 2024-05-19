rust_sources := src/*.rs
#rust_sources := src/*.rs anotherdir/*.rs
cargo := Cargo.toml Cargo.lock
install: copy.timestamp
	@echo "Done"

.SILENT:

copy.timestamp: target/release/websink target/x86_64-pc-windows-gnu/release/websink.exe
	cp $? install/
	touch copy.timestamp
	echo "Copied $?"

target/release/websink: $(rust_sources) $(cargo)
	cargo build --release
	echo "Build for Linux"

target/x86_64-pc-windows-gnu/release/websink.exe: $(rust_sources) $(cargo)
	cargo build --release --target x86_64-pc-windows-gnu
	echo "Build for Windows"

clean:
	rm target/release/websink target/x86_64-pc-windows-gnu/release/websink.exe
	cargo clean
	echo "Cleaned"