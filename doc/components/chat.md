# Chat

A conversation interface component for displaying chat messages with flexible placement and bubble styling.

## Description

The Chat component provides a structured way to display chat conversations with support for sender placement, message colors, and optional avatars. It includes sub-components for images, headers, footers, and message bubbles. Perfect for messaging applications, support chat interfaces, comment threads, and AI conversation displays.

## Examples

### Conversation Layout

```rust
<Chat placement=ChatPlacement::Start>
    <ChatImage>
        <Avatar class="w-10 h-10">
            <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Alice" alt="Alice" />
        </Avatar>
    </ChatImage>
    <ChatHeader>
        <span class="font-semibold">"Alice "</span>
        <span class="text-xs opacity-70">"10:30 AM"</span>
    </ChatHeader>
    <ChatBubble color=ChatBubbleColor::Primary>
        <p>"Hey! How are you doing today?"</p>
    </ChatBubble>
</Chat>

<Chat placement=ChatPlacement::End>
    <ChatBubble color=ChatBubbleColor::Secondary>
        <p>"I'm doing great, thanks for asking!"</p>
    </ChatBubble>
    <ChatFooter>
        <span class="text-xs opacity-70">"10:31 AM"</span>
    </ChatFooter>
</Chat>
```

### Message Types

```rust
<Chat placement=ChatPlacement::Start>
    <ChatImage>
        <Avatar class="w-8 h-8">
            <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Bob" alt="Bob" />
        </Avatar>
    </ChatImage>
    <ChatBubble color=ChatBubbleColor::Success>
        <p>"Task completed successfully!"</p>
    </ChatBubble>
</Chat>

<Chat placement=ChatPlacement::Start>
    <ChatBubble color=ChatBubbleColor::Warning>
        <p>"Warning: Please review before proceeding"</p>
    </ChatBubble>
</Chat>

<Chat placement=ChatPlacement::Start>
    <ChatBubble color=ChatBubbleColor::Error>
        <p>"Error: Unable to process request"</p>
    </ChatBubble>
</Chat>
```

### Rich Messages

```rust
<Chat placement=ChatPlacement::End>
    <ChatHeader>
        <span class="font-semibold">"You "</span>
        <span class="text-xs opacity-70">"2:15 PM"</span>
    </ChatHeader>
    <ChatBubble color=ChatBubbleColor::Info>
        <div class="space-y-2">
            <p>"Here's the update you requested:"</p>
            <ul class="list-disc list-inside">
                <li>"Feature 1: Complete"</li>
                <li>"Feature 2: In progress"</li>
                <li>"Feature 3: Pending"</li>
            </ul>
        </div>
    </ChatBubble>
    <ChatFooter>
        <span class="text-xs opacity-70">"2:16 PM • "</span>
        <span class="text-xs">"Read"</span>
    </ChatFooter>
</Chat>
```

## Props

| Prop        | Type                      | Default  | Description                    |
| ----------- | ------------------------- | -------- | ------------------------------ |
| `children`   | `Children`                | -        | Chat sub-components          |
| `class`      | `&'static str`           | `""`     | Additional CSS classes       |
| `node_ref`    | `NodeRef<Div>`            | -        | Node reference                |
| `placement`   | `Signal<ChatPlacement>`  | `Start`  | Left or right alignment       |

## Sub Components

### ChatImage

Container for user avatar or profile image.

| Prop       | Type           | Default | Description              |
| ---------- | -------------- | ------- | ------------------------ |
| `children` | `Children`     | -       | Avatar content            |
| `class`    | `&'static str` | `""`    | Additional CSS classes   |
| `node_ref` | `NodeRef<Div>`  | -       | Node reference             |

### ChatHeader

Header section for metadata like username and timestamp.

| Prop       | Type           | Default | Description              |
| ---------- | -------------- | ------- | ------------------------ |
| `children` | `Children`     | -       | Header content           |
| `class`    | `&'static str` | `""`    | Additional CSS classes   |
| `node_ref` | `NodeRef<Div>`  | -       | Node reference             |

### ChatBubble

The main message bubble containing chat content.

| Prop       | Type                        | Default   | Description              |
| ---------- | --------------------------- | --------- | ------------------------ |
| `children` | `Children`                  | -         | Message content           |
| `class`    | `&'static str`             | `""`      | Additional CSS classes   |
| `color`      | `Signal<ChatBubbleColor>`   | `Default` | Bubble color theme       |
| `node_ref` | `NodeRef<Div>`              | -         | Node reference             |

### ChatFooter

Footer section for delivery status, reactions, or timestamps.

| Prop       | Type           | Default | Description              |
| ---------- | -------------- | ------- | ------------------------ |
| `children` | `Children`     | -       | Footer content           |
| `class`    | `&'static str` | `""`    | Additional CSS classes   |
| `node_ref` | `NodeRef<Div>`  | -       | Node reference             |
