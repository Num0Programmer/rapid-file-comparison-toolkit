test:
	@echo "\n\033[1mRunning comparison between two duplicate files:\033[0m"
	rfctk tests/data/test_file.txt tests/data/other_test_file.txt
	@echo "\n\n\033[1mRunning comparison between a long and short file:\033[0m"
	rfctk tests/data/test_file.txt tests/data/small_test_file.txt

clean:
	rm -rf target Cargo.lock
