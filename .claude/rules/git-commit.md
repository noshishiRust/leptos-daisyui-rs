# Git Commit Standards

Standards for Git commits in this project.

## Pre-Commit Checklist

Before committing:

1. [ ] Run `leptosfmt .` to format code
2. [ ] Run `cargo clippy -- -D warnings` to lint
3. [ ] Run `cargo build` to verify build
4. [ ] Review staged changes: `git diff --staged`
5. [ ] Ensure commit message is descriptive

## Commit Message Style

Use clear, conventional commit messages:

```
type: brief description

Optional detailed explanation

- Bullet points for specific changes
- Each point on its own line
```

### Commit Types

- `feat` - New feature or component
- `fix` - Bug fix
- `refactor` - Code restructuring without behavior change
- `docs` - Documentation only
- `style` - Code style changes (formatting, etc.)
- `test` - Adding or updating tests
- `chore` - Maintenance tasks

### Examples

```
feat: add Button component with color and size variants

- Implement Button and LinkButton components
- Add ButtonColor, ButtonSize, ButtonStyle, ButtonShape enums
- Include node_ref support for DOM access
- Add CSS class documentation
```

```
fix: resolve missing Default variant in ButtonColor

- Add #[default] attribute to Default variant
- This was causing compilation errors in consumer code
```

```
refactor: simplify merge_classes! macro implementation

- Remove redundant intermediate allocations
- Improve performance by ~15%
- No behavior changes
```

## Commit Size Guidelines

- Keep commits focused on one logical change
- Split large changes into smaller, focused commits
- Each commit should pass `cargo build`
- Each commit should pass `cargo clippy`

## Commit Often

- Commit frequently with small, focused changes
- Easier to review and revert if needed
- Better git history for debugging

## Push Standards

Before pushing to remote:

1. [ ] Verify all commits in branch pass CI
2. [ ] Ensure branch name is descriptive
3. [ ] Reference related issue in branch or PR if applicable

## Common Mistakes to Avoid

### ❌ Don't commit unformatted code

```bash
# Before committing
leptosfmt .
git add .
git commit -m "..."
```

### ❌ Don't commit with clippy warnings

```bash
# Before committing
cargo clippy -- -D warnings --fix  # Auto-fix what's possible
# Then manually fix remaining issues
```

### ❌ Don't mix unrelated changes

```bash
# GOOD: Separate commits
git commit -m "feat: add Button component"
git commit -m "fix: typo in README"

# BAD: Combined commit
git commit -m "add Button and fix typo"
```

## Amending Commits

If you need to fix the most recent commit:

```bash
# Make additional changes
# Stage them
git add .

# Amend (combines with previous commit)
git commit --amend

# If already pushed, force push (use carefully!)
git push --force-with-lease
```

## Viewing Commit History

```bash
# View recent commits
git log --oneline -10

# View commit with changes
git show HEAD

# View commit stats
git log --stat -10
```
