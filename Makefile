rust_sources := src/*.rs
#rust_sources := src/*.rs anotherdir/*.rs

install: target/release/websink target/x86_64-pc-windows-gnu/release/websink.exe
	cp $? install/

target/release/websink: $(rust_sources) Cargo.toml
	cargo build --release

target/x86_64-pc-windows-gnu/release/websink.exe: $(rust_sources) Cargo.toml
	cargo build --release --target x86_64-pc-windows-gnu

clean:
	rm target/release/websink target/x86_64-pc-windows-gnu/release/websink.exe
	cargo clean