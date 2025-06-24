.PHONY: build run format clean help c

run:
	@echo "Running the application..."
	cargo run

build:
	@echo "Building the project..."
	cargo build --release
	@echo "Build complete. You can run the application using 'make run'."


format:
	@echo "Formatting the code..."
	cargo fmt

check:
	@echo "Checking the code..."
	cargo check

clean:
	@echo "Cleaning the project..."
	cargo clean 

help:
	@echo "\033[1;33mAvailable commands:\033[0m"
	@echo "  \033[1;36mrun\033[0m       Run the application"
	@echo "  \033[1;36mbuild\033[0m     Build the project in release mode"
	@echo "  \033[1;36mformat\033[0m    Format the code using cargo fmt"
	@echo "  \033[1;36mc\033[0m         Check the code using cargo check"
	@echo "  \033[1;36mclean\033[0m     Clean the project build targets" 