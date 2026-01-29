# Link component default href causes unwanted page scroll

## Severity
🟠 **MEDIUM** - Security/UX issue

## Description
The `Link` component uses `href.unwrap_or("#")` as a default, which causes the page to scroll to the top when a link without an href is clicked. This is poor UX and unexpected behavior.

## Problem Code
`src/components/link/component.rs` line 45:

```rust
href=href.unwrap_or("#")
```

## Issue
When `href` is `None`, the link gets `href="#"` which:
- Jumps to the top of the page (changes URL to `page#`)
- Adds unwanted history entries
- Breaks single-page app navigation expectations
- Creates unexpected user experience

## Proposed Solution

### Option 1: Use `javascript:void(0)` (Recommended)
```rust
let href_value = href.unwrap_or("javascript:void(0)");
// ...
href=href_value
```

Pros:
- No page jump
- No history pollution
- Standard practice for non-navigating links

### Option 2: Omit href when None
```rust
// Only add href attribute if Some
{move || href.map(|h| view! { href={h} })}
```

Pros:
- Cleanest approach
- Most semantically correct

Cons:
- More complex implementation

### Option 3: Make href required
```rust
#[prop(into)]
href: &'static str,
```

Pros:
- Forces explicit intent
- No ambiguity

Cons:
- Breaking change
- Less flexible

## Impact
- Current behavior causes unexpected page scrolling
- Breaks UX expectations for SPA navigation
- Minor security concern (URL manipulation)

## Use Case
```rust
// When href is optional/conditional
<Link color=LinkColor::Primary>
    "Click me"  // Currently jumps to top!
</Link>
```

## Recommendation
Implement Option 1 (`javascript:void(0)`) as it's a non-breaking change with immediate UX improvement.

## Files Changed
- `src/components/link/component.rs`
