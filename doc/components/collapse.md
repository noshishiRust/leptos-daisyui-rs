# Collapse

A collapsible container component that expands and contracts to show or hide content with smooth transitions.

## Description

The Collapse component provides an interactive way to toggle content visibility with optional visual indicators. It uses tabindex-based interaction for accessibility and supports arrow or plus/minus indicators to show expand/collapse state. Perfect for FAQs, accordion menus, settings panels, and content organization.

## Examples

### Basic Collapse

```rust
<Collapse>
    <CollapseTitle>"Click to expand"</CollapseTitle>
    <CollapseContent>
        <p>"This content is hidden by default and shown when the collapse is expanded."</p>
    </CollapseContent>
</Collapse>
```

### Visual Indicators

```rust
<Collapse modifier=CollapseModifier::Arrow>
    <CollapseTitle>"With Arrow Indicator"</CollapseTitle>
    <CollapseContent>
        <p>"The arrow rotates when you expand this section."</p>
    </CollapseContent>
</Collapse>

<Collapse modifier=CollapseModifier::Plus>
    <CollapseTitle>"With Plus/Minus Indicator"</CollapseTitle>
    <CollapseContent>
        <p>"A plus sign changes to minus when expanded."</p>
    </CollapseContent>
</Collapse>
```

### Forced States

```rust
<Carousel modifier=CarouselModifier::Start class="w-full">
    <CarouselItem class="w-1/3">
        <img
            src="https://picsum.photos/200/200?random=7"
            alt="Item 1"
            class="w-full"
        />
    </CarouselItem>
    <CarouselItem class="w-1/3">
        <img
            src="https://picsum.photos/200/200?random=8"
            alt="Item 2"
            class="w-full"
        />
    </CarouselItem>
    <CarouselItem class="w-1/3">
        <img
            src="https://picsum.photos/200/200?random=9"
            alt="Item 3"
            class="w-full"
        />
    </CarouselItem>
    <CarouselItem class="w-1/3">
        <img
            src="https://picsum.photos/200/200?random=10"
            alt="Item 4"
            class="w-full"
        />
    </CarouselItem>
</Carousel>
```

## Props

| Prop             | Type                            | Default   | Description                    |
| ---------------- | ------------------------------- | --------- | ------------------------------ |
| `checked`        | `Signal<bool>`                  | `false`   | Checked state for open         |
| `children`       | `Children`                      | -         | Collapse sub-components        |
| `class`          | `&'static str`                  | `""`      | Additional CSS classes         |
| `focus_open`     | `Signal<bool>`                  | `false`   | Whether to focus open or not   |
| `force`          | `Signal<CollapseForceModifier>` | `Default` | Force open/close state         |
| `inner_node_ref` | `NodeRef<Input>`                | -         | Node reference for inner input |
| `modifier`       | `Signal<CollapseModifier>`      | `Default` | Visual style modifier          |
| `outer_node_ref` | `NodeRef<Div>`                  | -         | Node reference for outer div   |

## Sub Components

### CollapseTitle

Clickable title section that toggles the collapse.

| Prop       | Type           | Default | Description            |
| ---------- | -------------- | ------- | ---------------------- |
| `children` | `Children`     | -       | Title content          |
| `class`    | `&'static str` | `""`    | Additional CSS classes |
| `node_ref` | `NodeRef<Div>` | -       | Node reference         |

### CollapseContent

Content section that shows/hides based on collapse state.

| Prop       | Type           | Default | Description            |
| ---------- | -------------- | ------- | ---------------------- |
| `children` | `Children`     | -       | Content to show/hide   |
| `class`    | `&'static str` | `""`    | Additional CSS classes |
| `node_ref` | `NodeRef<Div>` | -       | Node reference         |
