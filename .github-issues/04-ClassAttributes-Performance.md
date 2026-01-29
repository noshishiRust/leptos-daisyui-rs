# ClassAttributes performance issue - unnecessary string cloning on every render

## Severity
🟠 **MEDIUM** - Performance impact

## Description
The `ClassAttributes::to_class()` method unnecessarily clones strings and allocates intermediate vectors on every component render, impacting performance.

## Problem Code
`src/utils/class_attribute.rs` lines 32-54:

```rust
pub fn to_class(&self) -> String {
    self.values
        .iter()
        .filter_map(|attr| match attr {
            ClassAttribute::None => None,
            ClassAttribute::Dynamic(s) => {
                if !s.is_empty() {
                    Some(s.clone())  // ⬅️ Unnecessary clone!
                } else {
                    None
                }
            }
            ClassAttribute::Static(s) => {
                if !s.is_empty() {
                    Some(s.to_string())  // ⬅️ Unnecessary conversion!
                } else {
                    None
                }
            }
        })
        .collect::<Vec<String>>()  // ⬅️ Intermediate allocation!
        .join(" ")
}
```

## Issues
1. **Clones `Dynamic` strings** - Unnecessary memory allocation
2. **Converts `Static` strings** - Already `&'static str`, no need for `String`
3. **Allocates intermediate `Vec<String>`** - Then immediately joins
4. **Called on every render** - Performance impact multiplied

## Optimized Solution

```rust
pub fn to_class(&self) -> String {
    let mut result = String::new();
    let mut first = true;

    for attr in &self.values {
        let s = match attr {
            ClassAttribute::None => continue,
            ClassAttribute::Dynamic(s) if !s.is_empty() => s.as_str(),  // ⬅️ Borrow, don't clone
            ClassAttribute::Static(s) if !s.is_empty() => s,            // ⬅️ Already &str
            _ => continue,
        };

        if !first {
            result.push(' ');
        }
        result.push_str(s);  // ⬅️ Direct append
        first = false;
    }

    result
}
```

## Performance Impact

**Before:**
- Allocates: 1 `Vec<String>` + N `String`s (for clones)
- Operations: N clones + N allocations + 1 join

**After:**
- Allocates: 1 `String` (grows as needed)
- Operations: N borrows + N appends

**Improvement:** Eliminates all clones and intermediate allocations.

## Affected Components
**All components** - This is called on every render for every component that uses `merge_classes!`, which is virtually all components in the library.

## Benchmark Opportunity
For a component with 5 class attributes, this saves:
- 5 string clones
- 1 Vec allocation
- Memory pressure on every render

In a typical app with 100 components rendering, this adds up quickly.

## Files Changed
- `src/utils/class_attribute.rs`

## Testing
All existing tests pass with the optimized version. The function signature and behavior remain identical - only the implementation is optimized.
