# Pre-Commit Validation

All code changes must pass these checks before committing.

## Required Checks

Run these commands before any commit:

```bash
# 1. Format check (CI will fail if this doesn't pass)
leptosfmt --check .

# 2. Lint check (CI treats warnings as errors)
cargo clippy -- -D warnings

# 3. Build check
cargo build
```

## Fixing Issues

### Formatting Issues

If `leptosfmt --check .` fails:

```bash
leptosfmt .
```

### Clippy Warnings

If `cargo clippy -- -D warnings` fails:

Common issues to fix:
- Unused variables → remove or prefix with `_`
- Dead code → remove or mark with `#[allow(dead_code)]`
- Missing docs → add rustdoc comments
- Unnecessary clones → use references instead
- Potential panics → handle with `Result` or `Option`

### Build Failures

If `cargo build` fails:

1. Check for typos in code
2. Verify all modules declared in `mod.rs`
3. Check for missing dependencies in `Cargo.toml`
4. Verify imports are correct

## CI/CD Context

These checks match the CI pipeline in `.github/workflows/rust.yml`:

```yaml
- name: Run rustfmt
  run: leptosfmt --check .

- name: Run clippy
  run: cargo clippy -- -D warnings
```

If these pass locally, CI will likely pass.

## Quick Reference

```bash
# Full pre-commit check
leptosfmt --check . && cargo clippy -- -D warnings && cargo build

# Format and fix everything
leptosfmt . && cargo clippy -- -D warnings --fix
```
