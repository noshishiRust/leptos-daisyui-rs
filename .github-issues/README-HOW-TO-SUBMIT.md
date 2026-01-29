# How to Submit Issues to Upstream Repository

This directory contains ready-to-submit GitHub issue templates for contributing fixes back to the original repository.

## Repository Information
- **Original Repo:** https://github.com/noshishiRust/leptos-daisyui-rs
- **Your Fork:** https://github.com/davidcforbes/leptos-daisyui-rs

## Submission Priority

Submit issues in this order for maximum impact:

### 🔴 CRITICAL (Submit First)
1. **01-CRITICAL-RadialProgress-Compilation-Error.md** - Library doesn't compile!
2. **02-README-Doctest-Failures.md** - Tests fail

### 🟠 HIGH PRIORITY (Submit Next)
3. **03-Link-Component-href-Security-UX.md** - UX/Security issue
4. **04-ClassAttributes-Performance.md** - Performance on every render

### 🟢 NICE TO HAVE (Submit When Time Permits)
5. **05-Documentation-Typos.md** - Professional appearance

## How to Submit an Issue

### Step 1: Navigate to the Original Repository
Visit: https://github.com/noshishiRust/leptos-daisyui-rs/issues

### Step 2: Click "New Issue"
Click the green "New issue" button

### Step 3: Copy-Paste Template Content
1. Open the issue template file (e.g., `01-CRITICAL-RadialProgress-Compilation-Error.md`)
2. Copy the entire contents
3. Paste into the GitHub issue description

### Step 4: Add a Title
Use the first line (the `# heading`) as the issue title:
- Example: "RadialProgress component fails to compile - Invalid style attribute syntax"

### Step 5: Add Labels (if you have permissions)
Suggested labels:
- Issue 1: `bug`, `critical`, `help wanted`
- Issue 2: `documentation`, `bug`
- Issue 3: `bug`, `enhancement`
- Issue 4: `performance`, `enhancement`
- Issue 5: `documentation`

### Step 6: Mention You Have a Fix
At the bottom of each issue, add:

```markdown
---

## 📝 Note
I have a working fix for this issue in my fork and am happy to submit a PR if desired.

Fork: https://github.com/davidcforbes/leptos-daisyui-rs
```

## Alternative: Submit Pull Requests Directly

If you prefer to submit PRs instead of issues:

### For Issue #1 (RadialProgress)
```bash
# Create a branch
git checkout -b fix/radial-progress-style-syntax

# Commit only the RadialProgress fix
git add src/components/radial_progress/component.rs
git commit -m "Fix RadialProgress style attribute syntax

Use style: directive instead of style= tuple format.
This fixes compilation error in Leptos 0.8.

Fixes: Compilation error E0277"

# Push to your fork
git push fork fix/radial-progress-style-syntax

# Then create PR on GitHub
```

### For Issue #2 (README)
```bash
git checkout -b fix/readme-doctest-errors
git add README.md
git commit -m "Fix README.md doctest syntax errors

- Fix quote character and module path (line 27)
- Remove double semicolon (line 75)
- Mark incomplete examples as ignore

Fixes: cargo test --doc failures"
git push fork fix/readme-doctest-errors
```

## Batch Submission Option

You could also:
1. Submit all 5 issues at once
2. Wait for maintainer response
3. Submit PRs based on their preference

## Communication Template

When submitting issues, you can add this introduction:

```markdown
Hi @noshishiRust,

I recently did a comprehensive code review of leptos-daisyui-rs and found
several issues. I've documented them with detailed analysis and have working
fixes for all of them in my fork.

I'm submitting these as separate issues for clarity, but I'm happy to:
- Submit individual PRs for each fix
- Combine related fixes into single PRs
- Discuss the approach before submitting

Let me know what works best for you!

Fork: https://github.com/davidcforbes/leptos-daisyui-rs
```

## Expected Response Time

Be patient! Open source maintainers often:
- Respond within 1-7 days
- May be busy with other commitments
- Greatly appreciate quality contributions

## After Submitting

1. **Star the repository** ⭐ (if you haven't already)
2. **Watch for responses** (GitHub will email you)
3. **Be responsive** to feedback on your issues/PRs
4. **Be friendly and professional** in all communications

## Questions?

If the maintainer asks questions or requests changes:
- Respond promptly
- Be open to feedback
- Reference the detailed analysis in UPSTREAM_CONTRIBUTIONS.md

Good luck with your contributions! 🎉
