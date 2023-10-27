install:
	@echo "\033[1mInstalling rfc binary...\033[0m"
	cargo install --path .

test:
	@echo "\033[1mRunning unit tests...\033[0m"
	cargo test

clean:
	rm -rf target Cargo.lock
