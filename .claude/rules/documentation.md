# Documentation Standards

Documentation requirements for the project.

## Component Documentation

### Rustdoc Requirements

Every public component must have rustdoc:

```rust
/// # Component Name
///
/// A clear, concise description of what the component does
/// and when to use it.
///
/// Additional context about behavior, special considerations,
/// or implementation details can go here.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("all class names space-separated");
/// ```
///
/// ## Node References
/// - `node_ref` - References the `<element>` ([ElementType](https://developer.mozilla.org/en-US/docs/Web/API/ElementType))
///
/// ## Example
/// ```rust,ignore
/// view! {
///     <ComponentName prop=value>
///         {children()}
///     </ComponentName>
/// }
/// ```
#[component]
pub fn ComponentName(...) -> impl IntoView {
    // ...
}
```

Required sections:
- [ ] Title with `#` heading
- [ ] Description paragraph
- [ ] CSS `@source inline()` directive
- [ ] Node References with MDN links

### Props Documentation

Each prop should be documented:

```rust
/// Brief description of the prop's purpose and effect
#[prop(optional, into)]
prop_name: Signal<PropType>,
```

## README.md Updates

### When Adding Components

Update the implementation table in `README.md`:

```markdown
| Component | Status | Source Path | daisyUI Docs |
| ComponentName | ✅ | [src](src/components/component_name/) | [docs](https://daisyui.com/components/component-name/) |
```

### When Modifying Components

Update `README.md` only if:
- Public API changes
- New usage patterns are introduced
- Breaking changes occur

### When Removing/Deprecating

- Mark as deprecated in table
- Add migration guide if applicable
- Update version notes

## Component Examples

Users need clear, working examples:

### Basic Usage

```rust,ignore
use leptos_daisyui_rs as daisyui;

#[component]
fn Demo() -> impl IntoView {
    view! {
        <daisyui::Button color=daisyui::ButtonColor::Primary>
            "Click me"
        </daisyui::Button>
    }
}
```

### With All Props

```rust,ignore
#[component]
fn AdvancedDemo() -> impl IntoView {
    let (color, set_color) = signal(ButtonColor::Primary);
    let node_ref = NodeRef::<HTMLButton>::new();

    view! {
        <daisyui::Button
            color=color
            on:click=move |_| { set_color.update(|c| *c = ButtonColor::Secondary); }
            node_ref=node_ref
            class="my-custom-class"
        >
            "Advanced Button"
        </daisyui::Button>
    }
}
```

## CSS Documentation

Each component must document its CSS classes:

```rust
/// ### Add to `input.css`
/// ```css
/// @source inline("btn btn-neutral btn-primary btn-secondary ...");
/// ```
```

**Why**: Users need to add these to their CSS file for the component to work.

## Style Enum Documentation

Each style enum should be documented:

```rust
/// # Button Color Variants
///
/// Style enum for daisyUI button color classes that control
/// the semantic color scheme of button components.
#[derive(Clone, Debug, Default)]
pub enum ButtonColor {
    // ...
}
```

## Code Examples Requirements

When providing examples:

- [ ] Use `rust,ignore` to prevent doctest attempts
- [ ] Show imports when needed
- [ ] Compile and run before documenting
- [ ] Keep examples focused on one concept
- [ ] Use realistic, practical scenarios

## External Documentation References

Link to official docs where applicable:

- **daisyUI**: https://daisyui.com/components/{component_name}/
- **MDN API**: https://developer.mozilla.org/en-US/docs/Web/API/{ElementType}
- **Leptos Book**: https://book.leptos.dev/

## Documentation Files

### doc/components/{component_name}.md

If component documentation files exist, they should include:

- [ ] Props table with types and defaults
- [ ] Style variant tables
- [ ] Code examples (collapsible with `<details>`)
- [ ] Accessibility guidelines
- [ ] Best practices

## Commenting Code

### When to Add Comments

- [ ] Complex logic that isn't self-evident
- [ ] Workarounds for bugs or limitations
- [ ] Performance-critical sections
- [ ] Non-obvious ownership or borrowing

### When NOT to Add Comments

- [ ] Restating the code in English
- [ ] Obvious implementations
- [ ] Deprecated information

### Comment Style

```rust
// Single-line comments for brief notes

/// Doc comments for public items
/// Use this for rustdoc

/* Block comments for longer
   explanations that span multiple lines
*/
```

## Documentation Review Checklist

Before considering documentation complete:

- [ ] All public components have rustdoc
- [ ] All props are documented
- [ ] CSS classes listed with `@source inline()`
- [ ] Examples compile and run
- [ ] External links are correct
- [ ] README.md updated if API changed
- [ ] Code comments are helpful, not redundant
