# Breadcrumbs

A navigation component that displays hierarchical page paths with automatic separator styling.

## Description

The Breadcrumbs component provides a clear visual representation of navigation hierarchy, helping users understand their current location within the application. It automatically handles separator styling and supports both text and linked breadcrumb items. Perfect for nested pages, category hierarchies, and multi-level navigation.

## Examples

### Basic Breadcrumbs

```rust
<Breadcrumbs>
    <BreadcrumbItem>"Home"</BreadcrumbItem>
    <BreadcrumbItem>"Library"</BreadcrumbItem>
    <BreadcrumbItem>"Data"</BreadcrumbItem>
    <BreadcrumbItem>"Add New"</BreadcrumbItem>
</Breadcrumbs>
```

### With Links

```rust
<Breadcrumbs>
    <BreadcrumbItem href="/">"Home"</BreadcrumbItem>
    <BreadcrumbItem href="/library">"Library"</BreadcrumbItem>
    <BreadcrumbItem href="/library/data">"Data"</BreadcrumbItem>
    <BreadcrumbItem>"Add New"</BreadcrumbItem>
</Breadcrumbs>
```

### Custom Styling

```rust
<Breadcrumbs outer_class="text-sm" inner_class="gap-2">
    <BreadcrumbItem href="/dashboard" class="link-primary">"Dashboard"</BreadcrumbItem>
    <BreadcrumbItem href="/dashboard/projects" class="link-primary">"Projects"</BreadcrumbItem>
    <BreadcrumbItem href="/dashboard/projects/active" class="link-primary">"Active"</BreadcrumbItem>
    <BreadcrumbItem class="font-semibold">"Project Alpha"</BreadcrumbItem>
</Breadcrumbs>
```

## Props

| Prop            | Type           | Default | Description                         |
| --------------- | -------------- | ------- | ----------------------------------- |
| `children`      | `Children`     | -       | Breadcrumb items                    |
| `inner_class`   | `&'static str` | `""`    | Additional CSS classes for `<ul>`   |
| `inner_node_ref` | `NodeRef<Ul>`  | -       | Node reference for `<ul>` element   |
| `outer_class`   | `&'static str` | `""`    | Additional CSS classes for `<div>`  |
| `outer_node_ref` | `NodeRef<Div>` | -       | Node reference for `<div>` element  |

## Sub Components

### BreadcrumbItem

Individual breadcrumb item, optionally rendered as a link.

| Prop       | Type                 | Default | Description                    |
| ---------- | -------------------- | ------- | ------------------------------ |
| `children` | `Option<Children>`  | -       | Item content                  |
| `class`    | `&'static str`      | `""`    | Additional CSS classes       |
| `href`     | `MaybeProp<String>`  | -       | Optional link URL            |
| `node_ref` | `NodeRef<Li>`        | -       | Node reference for `<li>`     |
