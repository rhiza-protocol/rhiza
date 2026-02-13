# Contributing to Rhiza Protocol

Thank you for your interest in contributing to Rhiza! We welcome everyone — developers, researchers, writers, and enthusiasts.

## Getting Started

```bash
# Fork and clone
git clone https://github.com/YOUR_USERNAME/rhiza.git
cd rhiza

# Build
cargo build --workspace

# Run all tests
cargo test --workspace
```

## Ways to Contribute

### 🐛 Bug Reports
Open an [issue](https://github.com/rhiza-protocol/rhiza/issues) with:
- Steps to reproduce
- Expected vs actual behavior
- Your OS and Rust version

### 💡 Feature Requests
Open an issue with the `enhancement` label describing:
- The problem it solves
- Your proposed approach
- Any alternatives considered

### 🔧 Code Contributions
1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Ensure all tests pass (`cargo test --workspace`)
5. Commit with clear messages (`git commit -m 'Add amazing feature'`)
6. Push and open a Pull Request

### 📝 Documentation
Improvements to docs, README, whitepaper, and code comments are always welcome.

## Code Guidelines

- **Rust stable** — No nightly-only features
- **`cargo fmt`** — Format before committing
- **`cargo clippy`** — Fix all warnings
- **Tests** — Add tests for new functionality
- **Comments** — Document public APIs with doc comments

## Project Structure

| Crate | Purpose |
|-------|---------|
| `rhiza-core` | Core protocol: crypto, DAG, consensus, networking, wallet |
| `rhiza-node` | Node daemon with REST API and web wallet |
| `rhiza-cli` | Command-line interface for wallet operations |

## Questions?

Open an issue or start a discussion. We're happy to help!

---

*Every contribution makes the root system stronger. 🌿*
