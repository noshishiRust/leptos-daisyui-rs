# README.md contains syntax errors causing doctest failures

## Severity
🟡 **HIGH** - Tests fail, poor first impression

## Description
The README.md contains three code examples with syntax errors that cause `cargo test --doc` to fail.

## Problem
Running `cargo test` shows:

```
test src\../README.md - (line 148) ... FAILED
test src\../README.md - (line 75) ... FAILED
test src\../README.md - (line 27) ... FAILED
```

## Issues Found

### Issue 1: Line 27-28 - Wrong quote character and typo
**Current:**
```rust
use leptos-daisyui-rs as daisyui'

use daisyui:components:::Accordion;
```

**Should be:**
```rust
use leptos_daisyui_rs as daisyui;

use daisyui::components::Accordion;
```

Problems:
- Single quote `'` instead of semicolon `;`
- Triple colons `:::` instead of double `::`
- Wrong component name `#[components]` instead of `#[component]`

### Issue 2: Line 75 - Double semicolon and invalid context
**Current:**
```rust
use leptos_daisyui_rs::components::*;;

let active = Signal::derive(move || some_condition());
```

Problems:
- Double semicolon `;;`
- Code not wrapped in a function (invalid at module level)

**Should be:**
```rust,ignore
// Mark as ignore or wrap in a proper function
```

### Issue 3: Line 148 - Incomplete code
**Current:**
```rust
...

<FullWrapperButton>
```

Problem:
- Uses `...` which is not valid Rust syntax

**Should be:**
```rust,ignore
// Complete example or mark as ignore
```

## Solution
Add `,ignore` attribute to examples that are incomplete or pseudo-code:

```rust,ignore
// Example code here
```

## Impact
- ❌ `cargo test` fails
- ❌ CI/CD likely fails
- ❌ Poor first impression for new users
- ❌ Documentation appears broken

## To Reproduce
```bash
cargo test --doc
```

## Fix Available
I have corrected all three examples and can submit a PR.

## Files Changed
- `README.md`
