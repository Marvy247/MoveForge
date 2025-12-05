# Contributing to ParaForge

Thank you for your interest in contributing to ParaForge! This document provides guidelines and instructions for contributing.

## Code of Conduct

- Be respectful and inclusive
- Provide constructive feedback
- Focus on what is best for the community

## Getting Started

### Prerequisites

- Rust 1.75 or later
- Node.js 20 or later
- Git
- Cargo and npm/yarn

### Setting Up Development Environment

```bash
# Clone the repository
git clone https://github.com/paraforge/paraforge
cd paraforge

# Build Rust components
cargo build

# Run tests
cargo test

# Build SDK
cd sdk
npm install
npm run build

# Build frontend
cd ../frontend
npm install
npm run dev
```

## Development Workflow

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
```

### 2. Make Changes

- Write clean, documented code
- Follow existing code style
- Add tests for new features
- Update documentation as needed

### 3. Run Tests

```bash
# Rust tests
cargo test --all

# Rust linting
cargo clippy -- -D warnings

# Rust formatting
cargo fmt --all

# SDK tests
cd sdk && npm test

# Frontend build
cd frontend && npm run build
```

### 4. Commit Changes

Follow conventional commit messages:

```bash
git commit -m "feat: add new simulation feature"
git commit -m "fix: resolve race condition in parallel executor"
git commit -m "docs: update API documentation"
git commit -m "test: add integration tests for node forking"
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `test`: Adding or updating tests
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `chore`: Maintenance tasks

### 5. Push and Create PR

```bash
git push origin feature/your-feature-name
```

Then create a Pull Request on GitHub with:
- Clear description of changes
- Link to related issues
- Screenshots/videos if applicable
- Test results

## Code Style Guidelines

### Rust

- Follow standard Rust style (`cargo fmt`)
- Use meaningful variable names
- Add documentation comments for public APIs
- Keep functions focused and testable
- Prefer `Result` over `panic!`

Example:
```rust
/// Simulates a batch of transactions in parallel
///
/// # Arguments
/// * `transactions` - Vector of transactions to simulate
/// * `parallel` - Whether to execute in parallel
///
/// # Returns
/// Vector of simulation results
pub async fn simulate_batch(
    transactions: Vec<Transaction>,
    parallel: bool,
) -> Result<Vec<SimulationResult>> {
    // Implementation
}
```

### TypeScript

- Use TypeScript strict mode
- Prefer `const` over `let`
- Use async/await over promises
- Add JSDoc comments for exported functions
- Follow React best practices

Example:
```typescript
/**
 * Simulates a transaction on the ParaForge node
 * @param tx - Transaction to simulate
 * @returns Simulation result with gas estimates
 */
export async function simulateTransaction(
  tx: Transaction
): Promise<SimulationResult> {
  // Implementation
}
```

## Testing Guidelines

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_simulation() {
        // Test implementation
    }

    #[tokio::test]
    async fn test_async_function() {
        // Async test
    }
}
```

### Integration Tests

Create tests in `tests/` directory:

```rust
// tests/integration_test.rs
use paraforge::Simulator;

#[tokio::test]
async fn test_full_simulation_flow() {
    // Integration test
}
```

## Documentation

### Code Documentation

- Add doc comments to all public APIs
- Include examples in documentation
- Document error conditions
- Explain complex algorithms

### README Updates

- Update README.md for new features
- Add examples for new APIs
- Update installation instructions if needed

## Pull Request Process

1. **Ensure CI passes**: All tests must pass
2. **Update documentation**: Add/update relevant docs
3. **Add changelog entry**: Describe your changes
4. **Request review**: Tag appropriate reviewers
5. **Address feedback**: Respond to all review comments
6. **Squash commits**: Clean up commit history if needed

## Release Process

Releases are handled by maintainers:

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Create git tag: `git tag -a v0.2.0 -m "Release v0.2.0"`
4. Push tag: `git push origin v0.2.0`
5. CI will automatically build and publish

## Community

- **GitHub Discussions**: Ask questions and share ideas
- **Issues**: Report bugs and request features
- **Discord**: Join our community (link in README)
- **Twitter**: Follow @paraforge_dev for updates

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

## Questions?

Feel free to:
- Open an issue for bugs or feature requests
- Start a discussion for questions
- Join our Discord community
- Email us at hello@paraforge.dev

Thank you for contributing to ParaForge! 🚀
