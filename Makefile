.PHONY: build run format clean

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

clean:
	@echo "Cleaning the project..."
	cargo clean 