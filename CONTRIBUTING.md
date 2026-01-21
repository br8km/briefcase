# Contributing to Briefcase

Thank you for your interest in contributing to Briefcase! We welcome contributions from the community. This document provides guidelines and information for contributors.

## 🚀 Getting Started

### Development Setup

1. **Prerequisites**
   - Rust 1.75 or later
   - Git

2. **Clone and Setup**
   ```bash
   git clone https://github.com/br8km/briefcase.git
   cd briefcase
   cargo build
   cargo test
   ```

### Development Workflow

1. **Choose an Issue**: Look for issues labeled `good first issue` or `help wanted`
2. **Fork & Branch**: Create a feature branch from `main`
   ```bash
   git checkout -b feature/your-feature-name
   ```
3. **Code**: Make your changes following our guidelines
4. **Test**: Ensure all tests pass and add new tests if needed
5. **Commit**: Use clear, descriptive commit messages
6. **Push & PR**: Push your branch and create a pull request

## 📝 Code Guidelines

### Rust Standards

- Follow the official [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` for code formatting
- Run `cargo clippy` and fix all warnings
- Write comprehensive documentation comments (`///`) for public APIs

### Code Style

```rust
// Good: Clear naming, proper error handling
pub fn backup_data(source: &Path, destination: &Path) -> Result<(), BackupError> {
    // Implementation
    Ok(())
}

// Avoid: Unclear naming, poor error handling
pub fn do_backup(s: &Path, d: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Implementation
    Ok(())
}
```

### Testing

- Write unit tests for all public functions
- Include integration tests for complex workflows
- Use descriptive test names: `test_backup_workflow_firefox`
- Test both success and error cases

### Commit Messages

```
type(scope): description

[optional body]

[optional footer]
```

Types:
- `feat`: New features
- `fix`: Bug fixes
- `docs`: Documentation
- `style`: Code style changes
- `refactor`: Code refactoring
- `test`: Testing
- `chore`: Maintenance

Examples:
```
feat(backup): add support for custom folder sources
fix(crypto): resolve PBKDF2 key derivation issue
docs(readme): update installation instructions
```

## 🧪 Testing

### Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_backup_workflow_firefox

# With output
cargo test -- --nocapture

# Coverage (requires cargo-llvm-cov)
cargo llvm-cov --html
```

### Test Structure

```
tests/
├── unit/           # Unit tests for individual components
├── integration/    # Integration tests for workflows
├── contract/       # Contract tests for CLI interfaces
└── common/         # Shared test utilities
```

## 📚 Documentation

### Code Documentation

```rust
/// Performs a secure backup of the specified source directory.
///
/// This function handles the complete backup workflow including:
/// - Data validation
/// - Compression
/// - Encryption
/// - Integrity verification
///
/// # Arguments
/// * `source` - Path to the source directory
/// * `destination` - Path to store the backup
/// * `password` - Encryption password
///
/// # Returns
/// Returns `Ok(())` on success, or an error if backup fails
///
/// # Examples
/// ```
/// use briefcase::backup_data;
/// use std::path::Path;
///
/// let result = backup_data(
///     Path::new("/home/user/data"),
///     Path::new("/tmp/backup"),
///     "secure-password"
/// );
/// assert!(result.is_ok());
/// ```
pub fn backup_data(source: &Path, destination: &Path, password: &str) -> Result<(), BackupError> {
    // Implementation
}
```

### User Documentation

- Keep README.md updated with new features
- Add examples for new functionality
- Update configuration examples

## 🔒 Security

- Never commit sensitive data or credentials
- Use secure random generation for cryptographic operations
- Follow [Rust Security Guidelines](https://anssi-fr.github.io/rust-guide/)
- Report security issues privately to maintainers

## 🚨 Issue Reporting

### Bug Reports

Please include:
- Brief description of the issue
- Steps to reproduce
- Expected vs actual behavior
- Environment details (OS, Rust version)
- Relevant log output

### Feature Requests

Please include:
- Clear description of the proposed feature
- Use case and benefits
- Implementation suggestions if applicable

## 📋 Pull Request Process

1. **Ensure CI Passes**: All checks must pass before merging
2. **Review**: At least one maintainer review required
3. **Merge**: Squash merge with descriptive commit message

### PR Checklist

- [ ] Tests pass (`cargo test`)
- [ ] Code formatted (`cargo fmt`)
- [ ] Clippy warnings resolved (`cargo clippy`)
- [ ] Documentation updated
- [ ] Commit messages follow guidelines
- [ ] Breaking changes documented

## 🎯 Areas for Contribution

### High Priority
- **Performance**: Optimize backup speed and memory usage
- **Platforms**: Windows and macOS support
- **UI/UX**: Better CLI output and progress indicators

### Medium Priority
- **New Sources**: Database backups, application data
- **Monitoring**: Metrics and alerting
- **Plugins**: Extensible backup source system

### Low Priority
- **GUI**: Desktop application wrapper
- **Mobile**: Companion mobile app
- **Web**: Web-based management interface

## 📞 Getting Help

- **Discussions**: Use GitHub Discussions for questions
- **Issues**: Report bugs and request features
- **Discord**: Join our community chat

## 📄 License

By contributing to Briefcase, you agree that your contributions will be licensed under the same license as the project (MIT).

---

Thank you for contributing to Briefcase! Your efforts help make secure backup accessible to everyone. 🚀