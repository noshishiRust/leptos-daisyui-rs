# Avatar

A user profile image component that displays photos, initials, or placeholder content in a consistent thumbnail format.

## Description

The Avatar component provides a standardized way to display user profile images with optional online/offline status indicators. It supports custom sizing through Tailwind classes and can be grouped together for team member displays. Perfect for user profiles, chat interfaces, comment sections, and team member lists.

## Examples

### Basic Avatars

```rust
<Avatar class="w-12 h-12">
    <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Felix" alt="User avatar" />
</Avatar>

<Avatar class="w-16 h-16" modifier=AvatarModifier::Online>
    <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Aneka" alt="User avatar" />
</Avatar>

<Avatar class="w-20 h-20" modifier=AvatarModifier::Offline>
    <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=John" alt="User avatar" />
</Avatar>
```

### Avatar Group

```rust
<AvatarGroup>
    <Avatar class="w-12 h-12">
        <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Felix" alt="User 1" />
    </Avatar>
    <Avatar class="w-12 h-12">
        <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Aneka" alt="User 2" />
    </Avatar>
    <Avatar class="w-12 h-12">
        <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=John" alt="User 3" />
    </Avatar>
    <Avatar class="w-12 h-12" modifier=AvatarModifier::Placeholder>
        <span>"+5"</span>
    </Avatar>
</AvatarGroup>
```

### Placeholder Avatar

```rust
<Avatar class="w-16 h-16" modifier=AvatarModifier::Placeholder>
    <div class="flex items-center justify-center w-full h-full">
        <span class="text-2xl">"JD"</span>
    </div>
</Avatar>
```

## Props

| Prop        | Type                    | Default   | Description                        |
| ----------- | ----------------------- | --------- | ---------------------------------- |
| `class`     | `&'static str`         | `""`      | Additional CSS classes           |
| `children`  | `Children`              | -         | Avatar content                  |
| `modifier`  | `Signal<AvatarModifier>` | `Default` | Status indicator or type        |
| `node_ref`  | `NodeRef<Div>`          | -         | Node reference for avatar        |

## Sub Components

### AvatarGroup

Displays multiple avatars in a stacked or overlapping layout.

| Prop       | Type           | Default | Description            |
| ---------- | -------------- | ------- | ---------------------- |
| `class`    | `&'static str` | `""`    | Additional CSS classes |
| `children` | `Children`     | -       | Avatar components      |
| `node_ref` | `NodeRef<Div>`  | -       | Node reference          |
