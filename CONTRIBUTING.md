# Contributing to CircuitPython Deploy

Thank you for your interest in contributing to CircuitPython Deploy! This document provides guidelines and information for contributors.

## 🤝 How to Contribute

### Reporting Bugs

If you find a bug, please create an issue using our [bug report template](.github/ISSUE_TEMPLATE/bug_report.md). Include:

- Clear description of the issue
- Steps to reproduce
- Expected vs actual behavior
- Environment details (OS, cpd version, board type)
- Command output and error messages

### Suggesting Features

For feature requests, use our [feature request template](.github/ISSUE_TEMPLATE/feature_request.md). Include:

- Clear description of the proposed feature
- Use cases and motivation
- Example usage
- Implementation ideas (if any)

### Pull Requests

1. **Fork the repository** and create a feature branch
2. **Make your changes** following our coding standards
3. **Add tests** for new functionality
4. **Update documentation** as needed
5. **Submit a pull request** using our [PR template](.github/pull_request_template.md)

## 🔧 Development Setup

### Prerequisites

- Rust 1.75 or later
- Git
- A CircuitPython board for testing (recommended)

### Getting Started

```bash
# Clone your fork
git clone https://github.com/yourusername/circuitpython-deploy.git
cd circuitpython-deploy

# Create a feature branch
git checkout -b feature/my-new-feature

# Build and test
cargo build
cargo test
cargo test --test integration_test

# Run the tool
cargo run -- --help
```

### Project Structure

```
circuitpython-deploy/
├── src/                    # Source code
│   ├── main.rs            # Entry point
│   ├── cli.rs             # CLI argument parsing
│   ├── board.rs           # Board detection
│   ├── file_ops.rs        # File operations
│   ├── ignore.rs          # .cpdignore handling
│   └── error.rs           # Error types
├── tests/                 # Integration tests
├── examples/              # Example projects
├── .github/               # GitHub workflows and templates
└── scripts/               # Build and utility scripts
```

## 📝 Coding Standards

### Code Style

- Follow standard Rust formatting: `cargo fmt`
- Use clippy for linting: `cargo clippy`
- Write clear, descriptive commit messages
- Add comments for complex logic
- Use meaningful variable and function names

### Testing

- Add unit tests for new functions
- Add integration tests for new features
- Ensure all tests pass: `cargo test`
- Test on multiple platforms when possible
- Include edge cases and error conditions

### Documentation

- Update README.md for user-facing changes
- Add/update code comments for complex logic
- Update CHANGELOG.md for all changes
- Include examples for new features

## 🚀 Development Workflow

### Before Making Changes

1. Check existing issues and PRs to avoid duplication
2. Create an issue to discuss large changes
3. Ensure you understand the project goals and architecture

### Making Changes

1. **Create a feature branch**: `git checkout -b feature/description`
2. **Write code following our standards**
3. **Add comprehensive tests**
4. **Update documentation**
5. **Test thoroughly**

### Testing Your Changes

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Test with real CircuitPython board
cargo run -- --list-boards
cargo run -- --dry-run

# Check code quality
cargo fmt --check
cargo clippy -- -D warnings

# Validate workflows (if changed)
./scripts/validate-workflows.sh
```

### Submitting Changes

1. **Commit your changes**: Use clear, descriptive commit messages
2. **Push to your fork**: `git push origin feature/description`
3. **Create a pull request**: Use our PR template
4. **Respond to feedback**: Be responsive to review comments

## 🧪 Testing Guidelines

### Unit Tests

- Test individual functions and methods
- Use `#[cfg(test)]` modules
- Mock external dependencies when needed
- Cover both success and error cases

### Integration Tests

- Test end-to-end functionality
- Use temporary directories for file operations
- Test with mock CircuitPython boards
- Verify CLI behavior and output

### Manual Testing

Before submitting, manually test:

- Basic deployment workflow
- Board detection
- Error handling
- Different project structures
- .cpdignore patterns

## 📋 Pull Request Checklist

Before submitting a PR, ensure:

- [ ] Code follows project style guidelines
- [ ] All tests pass locally
- [ ] New tests added for new functionality
- [ ] Documentation updated (README, CHANGELOG, code comments)
- [ ] PR description clearly explains the changes
- [ ] No breaking changes (or clearly documented)
- [ ] Commit messages are clear and descriptive

## 🏷️ Release Process

Releases are automated via GitHub Actions:

1. **Update version** in `Cargo.toml`
2. **Update CHANGELOG.md** with release notes
3. **Create and push a tag**: `git tag v0.1.0 && git push origin v0.1.0`
4. **GitHub Actions** will build binaries and create the release

Only maintainers can create releases.

## 🎯 Areas for Contribution

We welcome contributions in these areas:

### High Priority
- Bug fixes and stability improvements
- Performance optimizations
- Cross-platform compatibility
- Documentation improvements

### Medium Priority
- New features (board management, deployment options)
- Additional CircuitPython board support
- Better error messages and user experience
- Test coverage improvements

### Nice to Have
- Shell completion scripts
- Configuration file support
- Watch mode for automatic deployment
- Network deployment features

## 🤔 Questions?

- **General questions**: Open a discussion on GitHub
- **Bug reports**: Use the bug report template
- **Feature ideas**: Use the feature request template
- **Security issues**: Email security@example.com (if applicable)

## 📜 Code of Conduct

We are committed to providing a welcoming and inclusive environment. Please:

- Be respectful and constructive in all interactions
- Welcome newcomers and help them learn
- Focus on what is best for the community
- Show empathy towards other community members

## 🙏 Recognition

Contributors will be recognized in:

- CHANGELOG.md release notes
- GitHub releases
- README.md (for significant contributions)

Thank you for helping make CircuitPython Deploy better for everyone! 🚀
