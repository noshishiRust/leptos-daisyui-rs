# Accordion

A collapsible content component that allows users to expand and collapse sections of information.

## Description

The Accordion component provides an organized way to display large amounts of content in a compact space. Users can expand sections they're interested in while keeping other sections collapsed, making it perfect for FAQs, settings panels, and content organization.

## Examples

### Basic Accordion

```rust
<Accordion
    name="demo1"
    checked=true
    modifier=AccordionModifier::Arrow
    class="border border-base-300"
>
    <AccordionTitle>"What is daisyUI?"</AccordionTitle>
    <AccordionContent>
        <p>
            "daisyUI is a Tailwind CSS component library that provides semantic class names for common UI components."
        </p>
    </AccordionContent>
</Accordion>

<Accordion name="demo1" modifier=AccordionModifier::Arrow class="border border-base-300">
    <AccordionTitle>"How do I use it with Leptos?"</AccordionTitle>
    <AccordionContent>
        <p>
            "This library provides Leptos components that wrap daisyUI classes with type-safe, reactive props."
        </p>
    </AccordionContent>
</Accordion>
```

### Modifiers

```rust
<Accordion name="demo2" modifier=AccordionModifier::Default class="border border-base-300">
    <AccordionTitle>"Default (no icon)"</AccordionTitle>
    <AccordionContent>
        <p>"No visual indicator"</p>
    </AccordionContent>
</Accordion>

<Accordion name="demo2" modifier=AccordionModifier::Arrow class="border border-base-300">
    <AccordionTitle>"Arrow indicator"</AccordionTitle>
    <AccordionContent>
        <p>"Shows rotating arrow"</p>
    </AccordionContent>
</Accordion>

<Accordion name="demo2" modifier=AccordionModifier::Plus class="border border-base-300">
    <AccordionTitle>"Plus indicator"</AccordionTitle>
    <AccordionContent>
        <p>"Shows plus/minus toggle"</p>
    </AccordionContent>
</Accordion>
```

## Props

| Prop         | Type               | Default | Description                            |
| ------------ | ------------------ | ------- | -------------------------------------- |
| `open`       | `Signal<bool>`     | `false` | Whether the accordion item is expanded |
| `arrow`      | `Signal<bool>`     | `true`  | Whether to show expand/collapse arrow  |
| `plus_minus` | `Signal<bool>`     | `false` | Use plus/minus icons instead of arrow  |
| `class`      | `&'static str`     | `""`    | Additional CSS classes                 |
| `on_toggle`  | `Option<Callback>` | -       | Callback when accordion is toggled     |
| `children`   | `Children`         | -       | Accordion content                      |

## Sub Components

### AccordionTitle
The clickable header section that toggles the accordion.

| Prop       | Type           | Default | Description            |
| ---------- | -------------- | ------- | ---------------------- |
| `class`    | `&'static str` | `""`    | Additional CSS classes |
| `children` | `Children`     | -       | Title content          |

### AccordionContent
The collapsible content area of the accordion.

| Prop       | Type           | Default | Description            |
| ---------- | -------------- | ------- | ---------------------- |
| `class`    | `&'static str` | `""`    | Additional CSS classes |
| `children` | `Children`     | -       | Content to show/hide   |
