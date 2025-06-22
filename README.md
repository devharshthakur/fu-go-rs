# FuGo-rs - The Go Uninstaller, in Rust

This is a Rust port of the Go TUI application `fugo`. It finds and helps you remove Go SDK installations from your system.

## Prerequisites

You need to have the Rust toolchain installed. You can get it from [rustup.rs](https://rustup.rs/).

## How to Build and Run

1.  **Clone the repository:**
    ```sh
    git clone <your-repo-url>
    cd fugo-rust
    ```

2.  **Build the project:**
    ```sh
    cargo build --release
    ```

3.  **Run the application:**
    ```sh
    ./target/release/fugo-rust
    ```

    On some systems, you might need `sudo` for write permissions to the Go installation directory:
    ```sh
    sudo ./target/release/fugo-rust
    ```