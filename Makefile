install:
	@echo "\033[1mInstalling rfc binary...\033[0m"
	cargo install --path .

test:
	@echo "\n\033[1mRunning comparison between two duplicate files:\033[0m"
	rfc tests/data/test_file.txt tests/data/other_test_file.txt
	@echo "\n\n\033[1mRunning comparison between a long and short file:\033[0m"
	rfc tests/data/test_file.txt tests/data/small_test_file.txt
	@echo "\n\n\033[1mRunning comparison between two duplicate directories:\033[0m"
	rfc tests/data/test_directory tests/data/other_test_directory

clean:
	rm -rf target Cargo.lock
