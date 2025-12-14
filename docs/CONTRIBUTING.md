# Contributing Guide

Thanks for wanting to contribute! This project is built by the community, for the community. Whether you're fixing a typo or adding a major feature, we appreciate your help.

## Quick Start

```bash
# Fork & clone
git clone https://github.com/your-username/social-fi-contract.git
cd social-fi-contract

# Install & setup
make install
make check          # Verify tools are installed

# Create a branch
git checkout -b feature/your-cool-feature

# Make changes, then test
make build
make test           # All 18 tests should pass
make lint           # Code should be clean
```

## What Can I Contribute?

- 🐛 **Bug fixes** - Found something broken? Fix it!
- ✨ **New features** - Have an idea? Build it!
- 📚 **Documentation** - Clarify confusing parts
- 🧪 **Tests** - More test coverage is always good
- 💡 **Ideas** - Open an issue to discuss

## Code Style

**Rust:**
```bash
make format         # Auto-format code
make lint           # Check for issues
make lint-fix       # Auto-fix where possible
```

**TypeScript (tests):**
```bash
pnpm lint
pnpm lint:fix
```

**Commit messages:** Use conventional commits
```
feat: add cooldown period to share trading
fix: bonding curve overflow with large supply
docs: add examples to API reference
test: add edge cases for governance voting
```

## Pull Request Process

1. **Fork & branch** - Create a feature branch from `main`
2. **Make changes** - Write code, add tests
3. **Test everything** - `make test` should pass 18/18
4. **Lint code** - `make lint` should pass
5. **Update docs** - If you changed behavior, update README/docs
6. **Open PR** - Describe what you changed and why

### PR Checklist

- [ ] All tests pass (`make test`)
- [ ] Code is formatted (`make format`)
- [ ] No lint errors (`make lint`)
- [ ] Documentation updated (if needed)
- [ ] Commit messages are clear
- [ ] PR description explains the change

## Reporting Bugs

**Before opening an issue:**
- Search existing issues to avoid duplicates
- Try to reproduce on latest `main` branch

**When reporting, include:**
- What you expected to happen
- What actually happened
- Steps to reproduce
- Error messages or logs
- Your environment (OS, Rust/Solana versions)

**Example:**
```
**Bug:** `buy_shares` fails with large amounts

**Expected:** Should buy shares successfully
**Actual:** Transaction fails with "SlippageExceeded" error

**Steps:**
1. Deploy contract on localnet
2. Try to buy 1000 shares at once
3. Transaction reverts

**Environment:** macOS, Solana 1.18.0, Anchor 0.32.1
```

## Feature Requests

Open an issue with:
- **Problem:** What pain point does this solve?
- **Solution:** How would the feature work?
- **Example:** Show a code example of the API
- **Alternatives:** Other approaches you considered

## Development Tips

**Run tests in watch mode:**
```bash
make test-watch
```

**View program logs:**
```bash
make logs
```

**Check program ID:**
```bash
make program-id
```

**Clean build if things get weird:**
```bash
make clean
make build
```

## Questions?

- 💬 **Discussion:** [GitHub Discussions](https://github.com/your-org/social-fi-contract/discussions)
- 🐛 **Bugs:** [GitHub Issues](https://github.com/your-org/social-fi-contract/issues)
- 📖 **Docs:** Check [README](../README.md) and [docs/](.)

## Code of Conduct

Be respectful and constructive. We're all here to build something cool together.

---

**Thanks for contributing! Every bit helps make this project better.** 🚀
