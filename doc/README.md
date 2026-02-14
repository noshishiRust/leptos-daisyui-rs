# Component Documentation

This directory contains comprehensive markdown documentation for all leptos-daisyui-rs components. Each component has its own markdown file with examples, props documentation, and best practices.

## Overview

The documentation system is powered by `demo-macros`, which parses markdown files and generates interactive Leptos demo pages. This means:

- **Documentation is Code**: Markdown files are parsed at compile time to generate live demo pages
- **Consistent Structure**: All component docs follow the same template for easy navigation
- **Interactive Examples**: Code blocks become live, runnable components in the demo app
- **Type-Safe Parsing**: The macro system validates documentation structure during build

---

## How It Works

### Macro Processing Pipeline

1. **Markdown Files**: Located in `doc/components/{component_name}.md`
2. **Macro Invocation**: `create_demo_component!("component_name")` in your demo code
3. **Parsing**: The macro uses `comrak` to parse markdown into an AST
4. **Code Generation**: Each markdown element is converted to Leptos view code
5. **Component Page**: A complete Leptos component is generated and compiled

### File Location

Component documentation files must be placed in:
```
doc/components/{component_name}.md
```

The file name must match the component name (e.g., `accordion.md` for the Accordion component).

---

## Supported Markdown Elements

The demo-macros parser supports a subset of CommonMark markdown. Only the following elements are parsed and rendered:

### 1. Headings (h1, h2, h3)

Headings are automatically converted to HTML heading tags with generated IDs.

**Syntax:**
```markdown
# Level 1 Heading
## Level 2 Heading
### Level 3 Heading
```

**Generated Output:**
- `h1`: `class="text-3xl font-bold"` with auto-generated ID
- `h2`: `class="text-xl font-semibold"` with auto-generated ID
- `h3`: `class="text-lg font-medium"` with auto-generated ID

**ID Generation:**
- Text is converted to lowercase
- Alphanumeric characters and hyphens/whitespace are preserved
- Special characters are removed
- Multiple consecutive hyphens are collapsed
- Maximum length: 50 characters

**Example:**
```markdown
### Basic Usage
```
Generates: `<h3 class="text-lg font-medium" id="basic-usage">Basic Usage</h3>`

---

### 2. Paragraphs

Paragraphs are automatically wrapped in `<p>` tags with styling applied.

**Syntax:**
```markdown
A collapsible content component that allows users to expand and collapse sections of information.
```

**Generated Output:**
```html
<p class="text-base-content/70">A collapsible content component...</p>
```

**Important Notes:**
- Each block of text separated by blank lines becomes a paragraph
- Paragraphs automatically receive `text-base-content/70` class for opacity
- Keep paragraphs concise and focused

---

### 3. Code Blocks

Code blocks are parsed and the content is treated as raw Rust code that becomes part of the generated view.

**Syntax:**
```markdown
```rust
<Accordion name="demo1" modifier=AccordionModifier::Arrow>
    <AccordionTitle>"Title"</AccordionTitle>
    <AccordionContent>
        <p>"Content"</p>
    </AccordionContent>
</Accordion>
```
```

**Generated Output:**
The Rust code inside the code block is parsed as a TokenStream and rendered as-is, wrapped in a `<div>`:
```rust
quote! {
    <div>
        #code_raw_stream  // Your code goes here
    </div>
}
```

**Requirements:**
- Code must be valid, parsable Rust syntax
- Code blocks are typically used for live component examples
- The code inside will be executed as part of the demo page
- No syntax highlighting is applied (code is rendered, not displayed)

---

### 4. Tables

Tables are converted to daisyUI Table components with zebra striping enabled.

**Syntax:**
```markdown
| Prop    | Type          | Default   | Description        |
| ------- | ------------- | --------- | ------------------ |
| `color` | `ButtonColor` | `Default` | Button color style |
| `size`  | `ButtonSize`  | `Medium`  | Button size        |
```

**Generated Output:**
```rust
<Table zebra=true>
    <TableHead>
        <TableRow>
            <TableHeader><span>Prop</span></TableHeader>
            <TableHeader><span>Type</span></TableHeader>
            <TableHeader><span>Default</span></TableHeader>
            <TableHeader><span>Description</span></TableHeader>
        </TableRow>
    </TableHead>
    <TableBody>
        <TableRow>
            <TableCell><span>color</span></TableCell>
            <TableCell><span>ButtonColor</span></TableCell>
            <TableCell><span>Default</span></TableCell>
            <TableCell><span>Button color style</span></TableCell>
        </TableRow>
        <!-- more rows... -->
    </TableBody>
</Table>
```

**Table Cell Formatting:**
- Plain text in cells is wrapped in `<span>` tags
- Inline code (backticks) is rendered with special styling:
  ```html
  <code class="bg-base-300 px-1 py-0.5 rounded text-sm font-mono">code</code>
  ```

---

## Unsupported Markdown Elements

The following CommonMark elements are **NOT** supported and will be ignored:
- Lists (ordered/unordered)
- Blockquotes
- Horizontal rules
- Images
- Links (except in inline context within tables)
- HTML embedded in markdown

If you need these elements, consider:
- Using tables for structured information
- Using paragraphs for descriptions
- Using code blocks for examples

---

## Documentation Structure

Each component documentation follows a consistent structure:

### Template

```markdown
# {ComponentName}

A brief one-sentence description of what the component does.

## Description

A longer paragraph (2-4 sentences) explaining:
- What the component provides
- When to use it
- What problems it solves
- Common use cases

## Examples

### {Example Name}

```rust
<Component
    prop1=value1
    prop2=value2
>
    {children}
</Component>
```

### {Another Example}

```rust
// More examples showing different configurations
```

## Props

| Prop        | Type               | Default         | Description                       |
| ----------- | ------------------ | --------------- | --------------------------------- |
| `prop_name` | `PropType`         | `default_value` | Description of what the prop does |
| `on_click`  | `Option<Callback>` | -               | Optional callback description     |

## Sub Components

### {SubComponentName}

Brief description of the sub-component.

| Prop   | Type   | Default   | Description |
| ------ | ------ | --------- | ----------- |
| `prop` | `Type` | `default` | Description |
```

---

## Writing Guidelines

### 1. Component Title
- Use the exact component name (PascalCase)
- No additional formatting or decorators

### 2. Description Section
- Write in present tense
- Focus on user benefits
- Mention key features and use cases
- Keep it to 2-4 sentences

**Example:**
> The Accordion component provides an organized way to display large amounts of content in a compact space. Users can expand sections they're interested in while keeping other sections collapsed, making it perfect for FAQs, settings panels, and content organization.

### 3. Examples Section

**Maximum 3 examples per component**

Limit each component documentation to maximum 3 example sections total. This ensures:
- Focused documentation that highlights most important features
- Faster page load times in the demo application
- Consistent documentation across all components

Choose examples that demonstrate:
- Clear, descriptive section names (e.g., "Basic Accordion", "Modifiers")
- Realistic, copy-pasteable code
- Key features and variations
- Prop variations that show different behaviors
- Most common use cases and patterns

**Example Code Style:**
```rust
<Accordion
    name="demo1"
    checked=true
    modifier=AccordionModifier::Arrow
    class="border border-base-300"
>
    <AccordionTitle>"What is daisyUI?"</AccordionTitle>
    <AccordionContent>
        <p>"daisyUI is a Tailwind CSS component library..."</p>
    </AccordionContent>
</Accordion>
```

### 4. Props Table
- List all public props in alphabetical order
- Use exact type names (include Signal wrapping if applicable)
- Show default values (use "-" for no default)
- Write concise descriptions (one line per prop)

**Props Table Format:**
```markdown
| Prop    | Type           | Default | Description                            |
| ------- | -------------- | ------- | -------------------------------------- |
| `open`  | `Signal<bool>` | `false` | Whether the accordion item is expanded |
| `class` | `&'static str` | `""`    | Additional CSS classes                 |
```

### 5. Sub Components Section
- List all child components (e.g., AccordionTitle, AccordionContent)
- Provide a brief description for each
- Include their props tables
- Follow the same table format as main props

---

## Common Patterns

### Optional Props
For props with `#[prop(optional)]`, use these defaults in the documentation:
- `String`/`&str`: `""`
- `bool`: `false` or `true` (whichever makes sense)
- `Signal<T>`: show the default value
- `Option<T>`: `-` (indicating None/null)
- `enum`: show the `#[default]` variant

### Callback Props
For callback/event props:
- Type: `Option<Callback>` or `Option<Fn()>`
- Default: `-`
- Description: "Callback when..." or "Called when..."

### Style Enum Props
When documenting enum props (Color, Size, etc.):
- Don't list all enum variants in the props table
- Consider creating an "Enum Variants" section if needed
- Refer users to the component's style.rs file for full list

### Children Prop
Always include `children` in props tables:
- Type: `Children`
- Default: `-` (required)
- Description: "Component content" or similar

---

## Validation Checklist

Before committing documentation, verify:

1. **Macro Parsing**: The file must parse correctly with `comrak`
   - Run `cargo build` to ensure demo-macros can parse it
   - Check for syntax errors in code blocks

2. **Code Validity**: All code blocks must be valid Rust
   - Props must match actual component signatures
   - Component names must be properly imported/in scope

3. **Coverage**: All public props should be documented
   - Compare with component.rs and style.rs
   - Ensure all variants and options are covered

4. **Consistency**: Follow the accordion.md template structure
   - Same heading levels
   - Same table format
   - Same writing style

---

## Troubleshooting

### Macro Fails to Parse
- **Issue**: Build error when running `cargo build`
- **Solution**: Check that code blocks contain valid Rust syntax
- **Solution**: Ensure tables have proper header/body rows

### Generated Page Shows Nothing
- **Issue**: Macro runs but demo page is empty
- **Solution**: Verify file path matches component name exactly
- **Solution**: Check that markdown uses supported elements only

### Tables Don't Render Correctly
- **Issue**: Table cells missing or misaligned
- **Solution**: Ensure proper pipe (`|`) separators
- **Solution**: Include header row (first row defines columns)

### Code Doesn't Execute
- **Issue**: Code blocks show as text instead of running
- **Solution**: Remove language identifier after opening ``` if needed
- **Solution**: Verify code is valid Leptos view syntax

---

## File Naming Convention

- Component files: `{component-name}.md` (kebab-case)
- Match the component module name from `src/components/`
- Use descriptive names that match the actual component names

Examples:
- `accordion.md` → Accordion component
- `button.md` → Button component
- `input-group.md` → InputGroup component

---

## Contributing to Documentation

When adding new component documentation:

1. **Follow the Template**: Use `accordion.md` as the canonical example
2. **Include Comprehensive Examples**: Show basic usage, variants, and real-world scenarios
3. **Document All Props**: Include every prop with accurate types and descriptions
4. **Add Accessibility Notes**: Consider screen readers and keyboard navigation
5. **Provide Best Practices**: Share implementation wisdom and common patterns

---

## Available Components

### Core Components
- [Accordion](./components/accordion.md) - Collapsible content sections

### Form Components
- [Checkbox](./components/checkbox.md) - Boolean selection with indeterminate support
- [Input](./components/input.md) - Text inputs with validation and different types

### Layout Components
- [Card](./components/card.md) - Content containers with headers, bodies, and actions

### Feedback Components
- [Alert](./components/alert.md) - Notification messages with different severity levels

### Interactive Components
- [Button](./components/button.md) - Interactive buttons with multiple styles and states
- [Modal](./components/modal.md) - Dialog overlays for forms and confirmations

---

## Usage in Demo Application

These markdown files are designed to be parsed and rendered in the demo application. The structure allows for:

1. **Dynamic Content Generation**: Parse markdown to create interactive examples
2. **Code Execution**: Extract code blocks and render them as live components
3. **Navigation**: Use file structure for component navigation
4. **Search**: Index content for component and feature search

### Example: Creating a Demo Page

```rust
use demo_macros::create_demo_component;

create_demo_component!("accordion");

// This generates:
#[component]
pub fn AccordionPage() -> impl IntoView {
    view! {
        // Content from doc/components/accordion.md
        // parsed and rendered as Leptos components
    }
}
```

---

## Additional Resources

- **Macro Implementation**: See `demo-macros/src/markdown/` for parsing logic
- **Component Template**: Refer to `doc/components/accordion.md` as canonical example
- **Styling Guide**: See `doc/css-integration-guide.md` for daisyUI class usage
- **Project Setup**: See `doc/user-project-setup.md` for development environment

---

## Future Enhancements

Potential improvements to the documentation system:

1. **Interactive Examples**: Live code editing and preview
2. **API Reference**: Auto-generated from Rust docs
3. **Testing Examples**: Unit test patterns and examples
4. **Performance Guides**: Optimization tips and benchmarks
5. **Integration Guides**: How to use with forms, routing, etc.

This documentation system provides a solid foundation for understanding and using leptos-daisyui-rs components while maintaining excellent developer experience through comprehensive coverage and interactive examples.
