# Contributing to HelloWorld

Thank you for your interest in contributing to the HelloWorld project!

## Development Setup

1. Fork the repository
2. Clone your fork: `git clone https://github.com/yourusername/helloworld.git`
3. Install Rust: https://rustup.rs/
4. Run tests: `cargo test`

## Code Standards

- Follow Rust naming conventions
- Write tests for new functionality
- Run `cargo fmt` before committing
- Run `cargo clippy` and fix any warnings
- Update documentation for public APIs

## Testing

- Unit tests should be in the same file as the code being tested
- Integration tests go in the `tests/` directory
- Run all tests with `cargo test`
- Run benchmarks with `cargo bench`

## Submitting Changes

1. Create a new branch for your feature: `git checkout -b feature-name`
2. Make your changes
3. Add tests for your changes
4. Run the full test suite: `cargo test`
5. Run formatting: `cargo fmt`
6. Run linting: `cargo clippy`
7. Commit your changes with a clear message
8. Push to your fork and submit a pull request

## Pull Request Guidelines

- Keep changes focused and atomic
- Write clear commit messages
- Include tests for new functionality
- Update documentation as needed
- Ensure CI passes

## Code of Conduct

Be respectful and inclusive in all interactions.
