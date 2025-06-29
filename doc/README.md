# Component Documentation

This directory contains comprehensive markdown documentation for all leptos-daisyui-rs components. Each component has its own markdown file with examples, props documentation, and best practices.

## Documentation Structure

Each component documentation follows a consistent structure:

### 1. Component Title and Description
- Clear component name and purpose
- Brief description of use cases and functionality

### 2. Props Table
- Comprehensive table of all component props
- Type information with Rust Signal types
- Default values and descriptions
- Optional vs required props clearly marked

### 3. Style Variants
- Enums and their possible values
- Visual style options (colors, sizes, shapes)
- Behavioral variants (styles, states)

### 4. Examples with Collapsible Code
- Multiple practical examples showing component usage
- Code examples wrapped in collapsible `<details>` sections
- Progressive complexity from basic to advanced usage
- Real-world scenarios and patterns

### 5. Accessibility Guidelines
- Screen reader compatibility notes
- Keyboard navigation support
- ARIA attributes and semantic HTML
- Focus management guidance

### 6. Best Practices
- Numbered list of implementation recommendations
- UX considerations and common pitfalls
- Performance tips and optimization advice

## Available Components

### Core Components
- [Button](./components/button.md) - Interactive buttons with multiple styles and states
- [Card](./components/card.md) - Content containers with headers, bodies, and actions
- [Modal](./components/modal.md) - Dialog overlays for forms and confirmations

### Form Components  
- [Input](./components/input.md) - Text inputs with validation and different types
- [Checkbox](./components/checkbox.md) - Boolean selection with indeterminate support

### Layout Components
- [Accordion](./components/accordion.md) - Collapsible content sections

### Feedback Components
- [Alert](./components/alert.md) - Notification messages with different severity levels

## Documentation Features

### Collapsible Code Examples
All code examples are wrapped in HTML `<details>` elements, making them collapsible:

```html
<details>
<summary>View Code</summary>

```rust
// Your Rust/Leptos code here
```

</details>
```

This allows users to:
- Quickly scan through examples without code clutter
- Expand only the examples they're interested in
- See code when needed while maintaining document readability

### Props Tables
Consistent table format for all component props:

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `prop_name` | `Signal<PropType>` | `Default::default()` | Clear description |

### Code Formatting
- Rust code blocks with proper syntax highlighting
- Inline code for prop names and types
- Consistent indentation and formatting

## Usage in Demo Application

These markdown files are designed to be parsed and rendered in the demo application. The structure allows for:

1. **Dynamic Content Generation**: Parse markdown to create interactive examples
2. **Code Execution**: Extract code blocks and render them as live components
3. **Navigation**: Use file structure for component navigation
4. **Search**: Index content for component and feature search

## Parsing Guidelines

When parsing these files for the demo application:

### Headers
- `# Component Name` - Main component title
- `## Section Name` - Major sections (Description, Props, Examples, etc.)
- `### Subsection` - Example categories or style variants

### Tables
- Props tables use consistent pipe-separated format
- First row is always the header
- Props are in backticks for code formatting

### Code Blocks
- Language specified as `rust` for syntax highlighting
- Examples are complete, runnable component code
- Import statements included for clarity

### Details/Summary
- `<summary>View Code</summary>` is standard for all examples
- Code blocks immediately follow the summary
- Closing `</details>` tag ends collapsible section

## Contributing to Documentation

When adding new component documentation:

1. **Follow the Template**: Use existing docs as templates for consistency
2. **Include Comprehensive Examples**: Show basic usage, variants, and real-world scenarios
3. **Document All Props**: Include every prop with accurate types and descriptions
4. **Add Accessibility Notes**: Consider screen readers and keyboard navigation
5. **Provide Best Practices**: Share implementation wisdom and common patterns

## File Naming Convention

- Component files: `component-name.md` (kebab-case)
- Match the component module name from `src/components/`
- Use descriptive names that match the actual component names

## Future Enhancements

Potential improvements to the documentation system:

1. **Interactive Examples**: Live code editing and preview
2. **Visual Design System**: Screenshots and visual guidelines  
3. **API Reference**: Auto-generated from Rust docs
4. **Testing Examples**: Unit test patterns and examples
5. **Performance Guides**: Optimization tips and benchmarks
6. **Integration Guides**: How to use with forms, routing, etc.

This documentation system provides a solid foundation for understanding and using leptos-daisyui-rs components while maintaining excellent developer experience through collapsible examples and comprehensive coverage.