# Contributing to **`fu-go-rs`**

First off, thank you for considering contributing to **`fu-go-rs`**! 🎉

This document provides guidelines and information for contributors. Following these guidelines helps communicate that you respect the time of the developers managing and developing this open source project.

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How Can I Contribute?](#how-can-i-contribute)
- [Development Setup](#development-setup)
- [Pull Request Process](#pull-request-process)
- [Coding Standards](#coding-standards)
- [Issue Guidelines](#issue-guidelines)
- [Security Issues](#security-issues)

## 📜 Code of Conduct

This project and everyone participating in it is governed by our [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code. Please report unacceptable behavior to the project maintainers.

## 🤝 How Can I Contribute?

### 🐛 Reporting Bugs

Before creating bug reports, please check existing issues as you might find that the problem has already been reported. When creating a bug report, please include as many details as possible:

**Bug Report Template:**
```markdown
**Describe the bug**
A clear and concise description of what the bug is.

**To Reproduce**
Steps to reproduce the behavior:
1. Go to '...'
2. Click on '....'
3. Scroll down to '....'
4. See error

**Expected behavior**
A clear and concise description of what you expected to happen.

**Screenshots**
If applicable, add screenshots to help explain your problem.

**Environment:**
 - OS: [e.g. macOS 13.0, Ubuntu 22.04]
 - Architecture: [e.g. arm64, x86_64]
 - Rust version: [e.g. 1.70.0]
 - Go installation type: [e.g. Homebrew, manual]

**Additional context**
Add any other context about the problem here.
```

### 💡 Suggesting Enhancements

Enhancement suggestions are welcome! Please:

1. Check if the enhancement has already been suggested
2. Provide a clear and detailed explanation of the feature
3. Explain why this enhancement would benefit users
4. Consider the scope and complexity

**Enhancement Template:**
```markdown
**Is your feature request related to a problem? Please describe.**
A clear and concise description of what the problem is.

**Describe the solution you'd like**
A clear and concise description of what you want to happen.

**Describe alternatives you've considered**
A clear and concise description of any alternative solutions.

**Additional context**
Add any other context or screenshots about the feature request.
```

### 🔧 Code Contributions

We love code contributions! Here are areas where help is especially appreciated:

1. **Cross-platform support** - Testing and improving Windows/Linux support
2. **Detection improvements** - Better Go installation detection logic
3. **UI/UX enhancements** - Improving the terminal interface
4. **Performance optimizations** - Making the tool faster
5. **Testing** - Adding unit tests and integration tests
6. **Documentation** - Improving code documentation

## 🚀 Development Setup

Please see [SETUP.md](SETUP.md) for detailed development setup instructions.

## 📝 Pull Request Process

### Before Creating a PR

1. **Fork the repository** and create your branch from `main`
2. **Install dependencies** and ensure the project builds
3. **Make your changes** following our coding standards
4. **Test your changes** thoroughly
5. **Update documentation** if needed
6. **Run the code quality checks**

### PR Submission

1. **Create a descriptive PR title**
   - Use conventional commit format: `feat:`, `fix:`, `docs:`, etc.
   - Be specific: "Add Windows Go detection" vs "Fix bug"

2. **Write a comprehensive PR description**
   ```markdown
   ## What does this PR do?
   Brief description of changes

   ## Why?
   Explain the motivation for these changes

   ## Testing
   How did you test these changes?

   ## Screenshots (if applicable)
   Add screenshots for UI changes

   ## Checklist
   - [ ] Code follows project style guidelines
   - [ ] Self-review completed
   - [ ] Tests added/updated
   - [ ] Documentation updated
   - [ ] No breaking changes (or clearly documented)
   ```

3. **Link related issues** using keywords like "Fixes #123"

### Code Quality Checks

Before submitting, ensure your code passes:

```bash
# Format code
cargo fmt

# Run linting
cargo clippy -- -D warnings

# Run tests
cargo test

# Check compilation
cargo check
```

## 🎯 Coding Standards

### Rust Style Guidelines

1. **Follow Rust conventions**
   - Use `snake_case` for functions and variables
   - Use `PascalCase` for types and enums
   - Use `SCREAMING_SNAKE_CASE` for constants

2. **Code formatting**
   - Use `cargo fmt` to format code
   - Line length: 100 characters (rustfmt default)
   - Use 4 spaces for indentation

3. **Documentation**
   ```rust
   /// Brief description of what this function does.
   /// 
   /// # Arguments
   /// 
   /// * `param` - Description of parameter
   /// 
   /// # Returns
   /// 
   /// Description of return value
   /// 
   /// # Examples
   /// 
   /// ```
   /// let result = my_function(param);
   /// ```
   pub fn my_function(param: Type) -> ReturnType {
       // Implementation
   }
   ```

4. **Error handling**
   - Use `Result<T, E>` for fallible operations
   - Use `thiserror` for custom error types
   - Provide meaningful error messages

5. **Async code**
   - Use `async/await` syntax
   - Handle cancellation gracefully
   - Avoid blocking operations in async contexts

### Project-Specific Guidelines

1. **TUI Guidelines**
   - Keep UI responsive (non-blocking operations)
   - Use consistent colors and styling
   - Handle terminal resize gracefully
   - Provide clear user feedback

2. **File System Operations**
   - Always check permissions before operations
   - Use atomic operations where possible
   - Provide detailed error messages
   - Handle edge cases (permissions, missing files, etc.)

3. **Cross-platform Considerations**
   - Test on multiple platforms
   - Use platform-specific conditional compilation
   - Handle path separators correctly
   - Consider different Go installation methods

## 🐛 Issue Guidelines

### Creating Issues

1. **Search existing issues** before creating new ones
2. **Use issue templates** when available
3. **Be specific** in titles and descriptions
4. **Include system information** for bugs
5. **Add appropriate labels**

### Issue Labels

- `bug` - Something isn't working
- `enhancement` - New feature or request
- `documentation` - Improvements or additions to documentation
- `good first issue` - Good for newcomers
- `help wanted` - Extra attention is needed
- `question` - Further information is requested
- `wontfix` - This will not be worked on

## 🔒 Security Issues

If you discover a security vulnerability, please:

1. **Do NOT create a public issue**
2. **Email the maintainers** directly
3. **Include detailed information** about the vulnerability
4. **Allow time** for the issue to be addressed before public disclosure



## 📚 Resources

### Learning Resources
- [Rust Book](https://doc.rust-lang.org/book/)
- [ratatui Documentation](https://docs.rs/ratatui/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)

### Project Resources
- [Development Setup](SETUP.md)
- [Code of Conduct](CODE_OF_CONDUCT.md)
- [Project Issues](https://github.com/devharshthakur/fu-go-rs/issues)
****

---

*This document is adapted from various open source contribution guidelines and is licensed under [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/).* 