# Diff

A side-by-side comparison component with an interactive resizer to adjust view proportions.

## Description

The Diff component provides a drag-to-resize interface for comparing two items side by side. Users can drag the resizer handle to adjust the proportion of space allocated to each item, making it perfect for before/after comparisons, file diffs, image comparisons, and split-view interfaces.

## Examples

### Basic Comparison

```rust
<Diff class="aspect-video">
    <DiffItem1>
        <div class="bg-warning text-warning-content p-8 text-center h-full flex flex-col justify-center">
            <h2 class="text-2xl font-bold mb-4">"Before"</h2>
            <p class="mt-2">"Contact us at: info@example.com"</p>
            <p class="mt-2">"Phone: (555) 123-4567"</p>
        </div>
    </DiffItem1>
    <DiffItem2>
        <div class="bg-success text-success-content p-8 text-center h-full flex flex-col justify-center">
            <h2 class="text-2xl font-bold mb-4">"After"</h2>
            <p class="mt-2">"Contact us at: hello@newdomain.com"</p>
            <p class="mt-2">"Phone: (555) 987-6543"</p>
            <p class="mt-2">"Live Chat Available 24/7"</p>
        </div>
    </DiffItem2>
    <DiffResizer />
</Diff>
```

### Image Comparison

```rust
<Diff class="aspect-video">
    <DiffItem1>
        <div class="bg-primary text-primary-content text-9xl font-black grid place-content-center h-full">
            "DAISY"
        </div>
    </DiffItem1>
    <DiffItem2>
        <div class="bg-base-200 text-9xl font-black grid place-content-center h-full">
            "DAISY"
        </div>
    </DiffItem2>
    <DiffResizer />
</Diff>
```

### Code Comparison

```rust
<Diff class="aspect-video">
    <DiffItem1>
        <div class="bg-base-300 p-6 h-full overflow-auto">
            <h3 class="font-bold mb-4 text-error">"❌ Before (HTML)"</h3>
            <div class="mockup-code text-sm">
                <pre data-prefix="1">
                    <code>"<div class=\"container\">"</code>
                </pre>
                <pre data-prefix="2">
                    <code>"  <h1>Title</h1>"</code>
                </pre>
                <pre data-prefix="3">
                    <code>"  <p>Content</p>"</code>
                </pre>
                <pre data-prefix="4">
                    <code>"  <button onclick=\"alert('Hi')\">"</code>
                </pre>
                <pre data-prefix="5">
                    <code>"    Click me"</code>
                </pre>
                <pre data-prefix="6">
                    <code>"  </button>"</code>
                </pre>
                <pre data-prefix="7">
                    <code>"</div>"</code>
                </pre>
            </div>
        </div>
    </DiffItem1>
    <DiffItem2>
        <div class="bg-base-100 p-6 h-full overflow-auto">
            <h3 class="font-bold mb-4 text-success">
                "✅ After (Leptos + daisyUI)"
            </h3>
            <div class="mockup-code text-sm">
                <pre data-prefix="1">
                    <code>"<Card class=\"shadow-xl\">"</code>
                </pre>
                <pre data-prefix="2">
                    <code>"  <CardBody>"</code>
                </pre>
                <pre data-prefix="3">
                    <code>"    <h1 class=\"card-title\">"</code>
                </pre>
                <pre data-prefix="4">
                    <code>"      \"Title\""</code>
                </pre>
                <pre data-prefix="5">
                    <code>"    </h1>"</code>
                </pre>
                <pre data-prefix="6">
                    <code>"    <p>\"Content\"</p>"</code>
                </pre>
                <pre data-prefix="7">
                    <code>"    <Button color=Primary>"</code>
                </pre>
                <pre data-prefix="8">
                    <code>"      \"Click me\""</code>
                </pre>
                <pre data-prefix="9">
                    <code>"    </Button>"</code>
                </pre>
                <pre data-prefix="10">
                    <code>"  </CardBody>"</code>
                </pre>
                <pre data-prefix="11">
                    <code>"</Card>"</code>
                </pre>
            </div>
        </div>
    </DiffItem2>
    <DiffResizer />
</Diff>
```

## Props

| Prop       | Type              | Default | Description            |
| ---------- | ----------------- | ------- | ---------------------- |
| `children` | `Children`        | -       | Diff sub-components    |
| `class`    | `&'static str`    | `""`    | Additional CSS classes |
| `node_ref` | `NodeRef<Figure>` | -       | Node reference         |

## Sub Components

### DiffItem1

The first (left) item in the diff comparison.

| Prop       | Type           | Default | Description            |
| ---------- | -------------- | ------- | ---------------------- |
| `children` | `Children`     | -       | Content for first item |
| `class`    | `&'static str` | `""`    | Additional CSS classes |
| `node_ref` | `NodeRef<Div>` | -       | Node reference         |

### DiffItem2

The second (right) item in the diff comparison.

| Prop       | Type           | Default | Description             |
| ---------- | -------------- | ------- | ----------------------- |
| `children` | `Children`     | -       | Content for second item |
| `class`    | `&'static str` | `""`    | Additional CSS classes  |
| `node_ref` | `NodeRef<Div>` | -       | Node reference          |

### DiffResizer

Interactive resizer handle that users drag to adjust proportions.

| Prop       | Type           | Default | Description            |
| ---------- | -------------- | ------- | ---------------------- |
| `class`    | `&'static str` | `""`    | Additional CSS classes |
| `node_ref` | `NodeRef<Div>` | -       | Node reference         |
