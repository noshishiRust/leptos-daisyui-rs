# Rules Directory

This directory contains focused rule files for different aspects of the leptos-daisyui-rs project.

## Rule Files

### Core Rules

- **`components.md`** - Component implementation patterns and structure
- **`rust.md`** - Rust coding standards and Leptos-specific patterns

### Process Rules

- **`pre-commit.md`** - Validation checks before committing code
- **`git-commit.md`** - Git commit message standards and practices
- **`task-workflow.md`** - General workflow for completing tasks

### Quality Assurance

- **`component-completion.md`** - Checklist for component completion
- **`code-review.md`** - Guidelines for reviewing code
- **`documentation.md`** - Documentation requirements and standards
- **`testing.md`** - Testing guidelines (currently manual)

## When to Use Each File

### Starting a Task

1. Read `task-workflow.md` for general workflow
2. Read topic-specific rules before starting:
   - Component work → `components.md`, `component-completion.md`
   - Rust code → `rust.md`
   - Documentation → `documentation.md`

### During Implementation

1. Reference `rust.md` for coding patterns
2. Reference `components.md` for component structure
3. Reference `documentation.md` for documentation requirements

### Before Committing

1. Run checks from `pre-commit.md`
2. Follow standards from `git-commit.md`

### Before Completing

1. Use `component-completion.md` checklist for component work
2. Use `code-review.md` for self-review
3. Use `testing.md` for manual verification

## Quick Reference

### Common Commands

```bash
# Format and lint
leptosfmt .
cargo clippy -- -D warnings

# Pre-commit check
leptosfmt --check . && cargo clippy -- -D warnings && cargo build
```

### File Structure

```
.claude/
├── rules/           # These files
│   ├── README.md    # This file
│   ├── components.md
│   ├── rust.md
│   ├── pre-commit.md
│   ├── git-commit.md
│   ├── task-workflow.md
│   ├── component-completion.md
│   ├── code-review.md
│   ├── documentation.md
│   └── testing.md
└── skills/          # Implementation skills
    ├── create-components.md
    └── install-component-guide.md
```

## Rule File Sizes

Each rule file is focused and concise:

- **Smallest**: `pre-commit.md` (1.4 KB)
- **Largest**: `rust.md` (8.9 KB)
- **Average**: ~4-5 KB per file

This makes each file easier to reference and maintain compared to one large file.
