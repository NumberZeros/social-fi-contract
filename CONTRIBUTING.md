# Contributing

Thank you for your interest in contributing to Social Fi Contract! This document provides guidelines and instructions for contributing.

## Development Setup

1. Clone the repository:
```bash
git clone https://github.com/NumberZeros/social-fi-contract.git
cd social-fi-contract
```

2. Run setup script:
```bash
./setup.sh
```

3. Create a new branch:
```bash
git checkout -b feature/your-feature-name
```

## Code Guidelines

### Rust Code
- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Add tests for new functionality

### TypeScript Code
- Follow ESLint rules defined in `.eslintrc.json`
- Use Prettier for formatting
- Run `pnpm lint:fix` before committing

### Commit Messages
- Use meaningful commit messages
- Format: `type: description`
- Types: feat, fix, docs, style, refactor, test, chore

Example:
```
feat: add user authentication instruction
fix: resolve wallet initialization issue
docs: update deployment guide
```

## Pull Request Process

1. Update documentation for any changes
2. Add tests for new functionality
3. Ensure all tests pass: `pnpm test`
4. Ensure code passes linting: `pnpm lint`
5. Create pull request with clear description

## Testing

Before submitting a PR:

```bash
# Run all tests
pnpm test

# Run with coverage (if available)
pnpm test --coverage

# Lint code
pnpm lint
pnpm lint:fix
```

## Reporting Issues

When reporting bugs, please include:
- Description of the issue
- Steps to reproduce
- Expected behavior
- Actual behavior
- Environment (OS, versions, etc.)

## Questions?

- Open an issue for questions
- Check existing issues for answers
- Refer to [Anchor Documentation](https://docs.anchor-lang.com/)

Thank you for contributing! 🙏
