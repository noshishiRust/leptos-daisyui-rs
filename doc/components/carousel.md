# Carousel

A scrollable container for displaying images or content in a horizontal or vertical arrangement.

## Description

The Carousel component provides an interactive way to present multiple items in a compact space with smooth scrolling behavior. It supports both horizontal and vertical layouts with configurable alignment options. Perfect for image galleries, product showcases, featured content sliders, and testimonial displays.

## Examples

### Horizontal Carousel

```rust
<Carousel>
    <CarouselItem class="w-80">
        <img src="https://picsum.photos/800/400?random=1" class="w-full object-cover" alt="Slide 1" />
    </CarouselItem>
    <CarouselItem class="w-80">
        <img src="https://picsum.photos/800/400?random=2" class="w-full object-cover" alt="Slide 2" />
    </CarouselItem>
    <CarouselItem class="w-80">
        <img src="https://picsum.photos/800/400?random=3" class="w-full object-cover" alt="Slide 3" />
    </CarouselItem>
</Carousel>
```

### Vertical Carousel

```rust
<Carousel direction=CarouselDirection::Vertical class="h-96">
    <CarouselItem class="h-64">
        <img src="https://picsum.photos/400/600?random=1" class="h-full w-full object-cover" alt="Vertical 1" />
    </CarouselItem>
    <CarouselItem class="h-64">
        <img src="https://picsum.photos/400/600?random=2" class="h-full w-full object-cover" alt="Vertical 2" />
    </CarouselItem>
</Carousel>
```

### Alignment Options

```rust
<Carousel modifier=CarouselModifier::Start>
    <CarouselItem class="w-64"><div class="bg-primary h-48 flex items-center justify-center text-white">"Start"</div></CarouselItem>
    <CarouselItem class="w-64"><div class="bg-secondary h-48 flex items-center justify-center text-white">"Item"</div></CarouselItem>
</Carousel>

<Carousel modifier=CarouselModifier::Center>
    <CarouselItem class="w-64"><div class="bg-accent h-48 flex items-center justify-center text-white">"Center"</div></CarouselItem>
    <CarouselItem class="w-64"><div class="bg-info h-48 flex items-center justify-center text-white">"Item"</div></CarouselItem>
</Carousel>

<Carousel modifier=CarouselModifier::End>
    <CarouselItem class="w-64"><div class="bg-success h-48 flex items-center justify-center text-white">"End"</div></CarouselItem>
    <CarouselItem class="w-64"><div class="bg-warning h-48 flex items-center justify-center text-white">"Item"</div></CarouselItem>
</Carousel>
```

## Props

| Prop         | Type                          | Default       | Description                    |
| ------------ | ----------------------------- | ------------- | ------------------------------ |
| `children`    | `Children`                    | -             | CarouselItem components      |
| `class`       | `&'static str`               | `""`          | Additional CSS classes       |
| `direction`   | `Signal<CarouselDirection>`   | `Horizontal` | Scroll direction              |
| `modifier`    | `Signal<CarouselModifier>`    | `Default`     | Alignment modifier           |
| `node_ref`    | `NodeRef<Div>`               | -             | Node reference                |

## Sub Components

### CarouselItem

Individual item within the carousel.

| Prop       | Type           | Default | Description              |
| ---------- | -------------- | ------- | ------------------------ |
| `children` | `Children`     | -       | Item content              |
| `class`    | `&'static str` | `""`    | Additional CSS classes   |
| `node_ref` | `NodeRef<Div>`  | -       | Node reference             |
