# Setup Guide

This guide will help you set up the development environment for **fu-go-rs**.

## 📋 Prerequisites

### Required Tools

1. **Rust Toolchain**
   ```bash
   # Install Rust via rustup
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Reload your shell or run:
   source ~/.cargo/env
   
   # Verify installation
   rustc --version
   cargo --version
   ```

2. **Git**
   ```bash
   # macOS (via Homebrew)
   brew install git
   
   # Ubuntu/Debian
   sudo apt update && sudo apt install git
   
   # Verify installation
   git --version
   ```

### Optional but Recommended

1. **Rust Analyzer** (VS Code Extension)
   - Provides excellent Rust language support
   - Install from VS Code marketplace

2. **rust-fmt** and **clippy** (included with Rust)
   ```bash
   # Ensure they're installed
   rustup component add rustfmt clippy
   ```

## 🚀 Project Setup

### 1. Clone the Repository

```bash
git clone https://github.com/devharshthakur/fu-go-rs.git
cd fu-go-rs
```

### 2. Verify Dependencies

The project uses these main dependencies (defined in `Cargo.toml`):
- `ratatui` - TUI framework
- `crossterm` - Cross-platform terminal handling
- `tokio` - Async runtime
- `thiserror` - Error handling
- `home` - Home directory detection

### 3. Build the Project

```bash
# Debug build (faster compilation, slower runtime)
cargo build

# Release build (slower compilation, faster runtime)
cargo build --release

# Check for compilation errors without building
cargo check
```

### 4. Run the Application

```bash
# Run in debug mode
cargo run

# Run the release build
./target/release/fu-go-rs
```


### Manual Testing

For manual testing, you can:

1. **Test on a system with Go installed**
   ```bash
   # Install Go first (if not present)
   # Then run fu-go-rs to test detection
   cargo run
   ```

2. **Test on a system without Go**
   ```bash
   # Should show "No Go installation found"
   cargo run
   ```

3. **Test different installation types**
   - Manual Go installation (`/usr/local/go`)
   - Homebrew installation (`/opt/homebrew/Cellar/go`)
   - GVM installation (`~/.gvm/gos/`)

## 🔧 Development Workflow

### Code Style

```bash
# Format code
cargo fmt

# Check formatting without changing files
cargo fmt --check

# Run clippy for linting
cargo clippy

# Run clippy with stricter rules
cargo clippy -- -D warnings
```

### Debugging

1. **Add debug prints**
   ```rust
   println!("Debug: {:?}", variable);
   dbg!(variable);
   ```

2. **Use logging** (if implemented)
   ```rust
   log::debug!("Debug message");
   log::info!("Info message");
   ```

3. **VS Code debugging**
   - Install "CodeLLDB" extension
   - Set breakpoints in code
   - Use F5 to start debugging

### Project Structure

```
fu-go-rs/
├── Cargo.toml          # Project configuration and dependencies
├── Cargo.lock          # Dependency lock file
├── src/
│   ├── main.rs         # Entry point - handles TUI event loop
│   ├── app.rs          # Application state and message handling
│   ├── tui/
│   │   ├── mod.rs      # TUI initialization/cleanup
│   │   └── ui.rs       # UI rendering and layout
│   └── util/
│       ├── mod.rs      # Utility modules declaration
│       ├── go_finder.rs # Go installation detection logic
│       └── go_deleter.rs # Go installation removal logic
├── README.md           # Project documentation
├── SETUP.md           # This file
├── CONTRIBUTING.md    # Contribution guidelines
└── CODE_OF_CONDUCT.md # Code of conduct
```

## 🐛 Common Issues & Troubleshooting

> ⚠️ **Beta Notice**: This project is currently in beta stage. While we strive to provide a stable experience, issues may arise and solutions may not work in all environments. Use at your own discretion.

### 🔧 Rust Toolchain Issues

**Problem**: `command not found: cargo`
**Solution**: Ensure Rust is properly installed and your shell is reloaded:

## 📊 Performance Profiling

### Basic Profiling

```bash
# Build with debug symbols
cargo build --release

# Use built-in time measurement
time ./target/release/fu-go-rs

# Memory usage monitoring
valgrind --tool=memcheck ./target/release/fu-go-rs
```

### Advanced Profiling

```bash
# Install cargo-profiler
cargo install cargo-profiler

# Profile the application
cargo profiler callgrind
```

## 🔄 Git Workflow

### Branching Strategy

```bash
# Create feature branch
git checkout -b feature/your-feature-name

# Make changes and commit
git add .
git commit -m "feat: add new feature"

# Push to remote
git push origin feature/your-feature-name

# Create pull request via GitHub
```

### Commit Message Format

Use conventional commits:
- `feat:` for new features
- `fix:` for bug fixes
- `docs:` for documentation changes
- `refactor:` for code refactoring
- `test:` for adding tests
- `chore:` for maintenance tasks

## 📚 Learning Resources

### Rust Resources
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings) - Interactive exercises

### TUI Development
- [ratatui documentation](https://docs.rs/ratatui/)
- [ratatui examples](https://github.com/ratatui-org/ratatui/tree/main/examples)
- [crossterm documentation](https://docs.rs/crossterm/)

### Async Rust
- [Tokio tutorial](https://tokio.rs/tokio/tutorial)
- [Async Rust book](https://rust-lang.github.io/async-book/)


Happy coding! 🦀 

## 🛠️ Using the Makefile

The project includes a `Makefile` that provides convenient shortcuts for common development tasks:

### Available Commands

```bash
# Run the application
make run

# Build the project in release mode
make build

# Format the code using cargo fmt
make format

# Clean the project build targets
make clean

# Show all available commands
make help
```


### Example Workflow

```bash
# Quick development cycle
make format    # Format your code
make build     # Build the project
make run       # Test your changes
make clean     # Clean up when done
```

### 🔧 Makefile Issues

**Problem**: `command not found: make`
**Solution**: Install Make on your system:

```bash
# macOS (usually pre-installed)
# If not, install via Homebrew:
brew install make

# Ubuntu/Debian
sudo apt update && sudo apt install make

# Windows (with WSL or Git Bash)
# Make is usually available in these environments
```

**Problem**: Makefile commands not working
**Solution**: Ensure you're in the project root directory and the Makefile exists:

```bash
ls Makefile  # Should show the Makefile
make help    # Should show available commands

## 🚀 Ready to Contribute?

Once you have the development environment set up:

1. Read [CONTRIBUTING.md](CONTRIBUTING.md) for contribution guidelines
2. Check the [Issues](https://github.com/devharshthakur/fu-go-rs/issues) page for tasks
``` 