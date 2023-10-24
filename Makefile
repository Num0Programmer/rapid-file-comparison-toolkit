all:
	cargo build --release
	cp target/release/rfctk /usr/local/bin

clean:
	rm -rf target Cargo.lock
