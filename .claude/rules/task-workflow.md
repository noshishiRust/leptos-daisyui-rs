# Task Workflow

General workflow for completing tasks in this project.

## Task Start

### Before Starting

1. [ ] Read and understand requirements
2. [ ] Check related issues/PRs
3. [ ] Review relevant documentation:
   - `.claude/rules/components.md` for component patterns
   - `.claude/rules/rust.md` for Rust standards
   - `.claude/rules/git-commit.md` for commit standards

### Planning

1. Identify files to create/modify
2. Estimate complexity and time
3. Plan implementation approach
4. Identify dependencies or blockers

## During Implementation

### Progress Tracking

- Break complex tasks into smaller sub-tasks
- Mark tasks as in-progress when starting
- Update task status as you progress

### Code Changes

1. Make targeted changes related to task
2. Run `leptosfmt .` frequently
3. Run `cargo clippy -- -D warnings` frequently
4. Build regularly to catch issues early

### When Blocked

1. Document the blocker clearly
2. Note what's needed to unblock
3. Mark task as blocked if appropriate
4. Communicate to team if external blocker

## Pre-Completion

### Verification Steps

1. [ ] Run `leptosfmt .`
2. [ ] Run `leptosfmt --check .`
3. [ ] Run `cargo clippy -- -D warnings`
4. [ ] Run `cargo build`
5. [ ] Manual testing (if applicable)
6. [ ] Update documentation
7. [ ] Update README.md (if API changed)

### Code Review Self-Check

Before requesting review:
- [ ] Review your own diff
- [ ] Check for commented-out code to remove
- [ ] Verify all TODOs have issues or are removed
- [ ] Ensure commit history is clean

## Completion Criteria

A task is complete when:

- [ ] Code changes are formatted
- [ ] Code passes linting
- [ ] Code builds successfully
- [ ] New files are documented
- [ ] Modified files have updated docs
- [ ] Manual testing passed (if applicable)
- [ ] Task-specific checklist complete

See also:
- `.claude/rules/pre-commit.md` - Pre-commit validation
- `.claude/rules/component-completion.md` - Component-specific checks
- `.claude/rules/testing.md` - Testing guidelines
- `.claude/rules/documentation.md` - Documentation requirements

## Post-Completion

### After Marking Complete

1. [ ] Update task tracking system
2. [ ] Notify relevant team members
3. [ ] Close related issues
4. [ ] Archive reference materials

### After Merge

1. [ ] Delete development branches
2. [ ] Clean up any temporary files
3. [ ] Update project metrics (if applicable)

## Quick Reference Commands

```bash
# Format check
leptosfmt --check .

# Format code
leptosfmt .

# Lint check
cargo clippy -- -D warnings

# Build
cargo build

# WASM build
cargo build --target wasm32-unknown-unknown

# View staged changes
git diff --staged

# View recent commits
git log --oneline -5
```

## Task Types

### Bug Fix Workflow

1. Reproduce the bug
2. Identify root cause
3. Implement minimal fix
4. Test fix resolves issue
5. Add regression test (if appropriate)
6. Update docs if behavior changed
7. Commit with `fix:` prefix

### Feature Workflow

1. Understand requirements
2. Review daisyUI documentation
3. Plan component structure
4. Implement style.rs (enums)
5. Implement component.rs (components)
6. Implement mod.rs (exports)
7. Update module exports
8. Add documentation
9. Test manually in browser
10. Update README.md
11. Commit with `feat:` prefix

### Refactor Workflow

1. Understand current implementation
2. Identify improvement opportunities
3. Plan refactoring approach
4. Make targeted changes (no behavior changes)
5. Verify no breaking changes
6. Update docs if API changes (should be minimal)
7. Commit with `refactor:` prefix

## Getting Help

If stuck:
1. Check existing components for examples
2. Review `.claude/rules/` files
3. Consult daisyUI documentation
4. Consult Leptos documentation
5. Ask team for guidance

## Communication

### When to Ask for Review

- After implementing significant feature
- Before refactoring public API
- When unsure about approach
- After resolving blocking issue
- Before major changes

### When to Request Input

- Design decisions affecting multiple components
- Breaking changes to public API
- Approaches with trade-offs
- Performance concerns
- Accessibility considerations
