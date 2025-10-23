# Contributing to PolyNeurons

Thank you for your interest in contributing! 🎉

## Development Setup

1. Fork repository
2. Clone fork Anda:
```bash
git clone https://github.com/your-username/polyneurons
cd polyneurons
```

3. Install dependencies:
```bash
make install
```

4. Build project:
```bash
make build
```

## Development Workflow

### Rust Components

```bash
# Format code
cargo fmt --all

# Lint
cargo clippy --all-targets --all-features

# Test
cargo test --workspace

# Build
cargo build --release
```

### Smart Contracts

```bash
# Compile
npm run compile

# Test
npm run test

# Deploy (testnet)
npm run deploy:amoy
```

## Code Style

### Rust
- Follow Rust standard style (rustfmt)
- Use meaningful variable names
- Add comments for complex logic
- Write tests for all public functions

### Solidity
- Follow Solidity style guide
- Use NatSpec comments
- Optimize gas usage
- Add comprehensive tests

## Pull Request Process

1. Create feature branch:
```bash
git checkout -b feature/amazing-feature
```

2. Make changes and commit:
```bash
git add .
git commit -m "feat: add amazing feature"
```

3. Push ke fork:
```bash
git push origin feature/amazing-feature
```

4. Create Pull Request di GitHub

### Commit Message Format

```
<type>: <description>

[optional body]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `style`: Formatting
- `refactor`: Code restructuring
- `test`: Adding tests
- `chore`: Maintenance

## Testing

All PRs must include tests:

```bash
# Run all tests
make test

# Run specific test
cargo test test_name
npm run test -- --grep "test name"
```

## Documentation

Update documentation if you:
- Add new APIs
- Change existing behavior
- Add new features

## Questions?

- Open issue for discussion
- Join Discord: [link]
- Email: [email]

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
