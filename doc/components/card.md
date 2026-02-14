# Card

A versatile content container component with structured layout options for organizing related information and actions.

## Description

The Card component provides a flexible container for displaying content with consistent styling and spacing. It supports multiple sizes, visual styles, and layout orientations including side-by-side arrangements with images. Perfect for content summaries, user profiles, product listings, and dashboard widgets.

## Examples

### Basic Card Structure

```rust
<Card class="w-96 bg-base-100 shadow-xl">
    <CardBody>
        <CardTitle>"Product Name"</CardTitle>
        <p>"This is a detailed description of the product or content."</p>
        <CardActions>
            <Button color=ButtonColor::Primary>"Buy Now"</Button>
            <Button>"Learn More"</Button>
        </CardActions>
    </CardBody>
</Card>
```

### Sizes and Styles

```rust
<Card size=CardSize::Sm class="shadow-xl">
    <CardBody>"Small card content"</CardBody>
</Card>

<Card size=CardSize::Lg style=CardStyle::Border class="shadow-xl">
    <CardBody>"Large bordered card"</CardBody>
</Card>

<Card style=CardStyle::Dash class="shadow-xl">
    <CardBody>"Dashed border card"</CardBody>
</Card>
```

### Side Layout with Image

```rust
<Card side=true class="w-96 bg-base-100 shadow-xl">
    <figure>
        <img src="https://picsum.photos/400/200?random=1" alt="Demo image" />
    </figure>
    <CardBody>
        <CardTitle>"Featured Article"</CardTitle>
        <p>"Lorem ipsum dolor sit amet, consectetur adipiscing elit."</p>
        <CardActions>
            <Button>"Read More"</Button>
        </CardActions>
    </CardBody>
</Card>
```

## Props

| Prop         | Type                | Default   | Description                    |
| ------------ | ------------------- | --------- | ------------------------------ |
| `class`      | `&'static str`      | `""`      | Additional CSS classes         |
| `children`   | `Children`          | -         | Card content                   |
| `image_full` | `Signal<bool>`      | `false`   | Background image covers card   |
| `node_ref`   | `NodeRef<Div>`      | -         | Node reference                 |
| `side`       | `Signal<bool>`      | `false`   | Horizontal side-by-side layout |
| `size`       | `Signal<CardSize>`  | `Md`      | Size preset                    |
| `style`      | `Signal<CardStyle>` | `Default` | Visual style variant           |

## Sub Components

### CardBody

Main content area of the card.

| Prop       | Type           | Default | Description            |
| ---------- | -------------- | ------- | ---------------------- |
| `children` | `Children`     | -       | Body content           |
| `class`    | `&'static str` | `""`    | Additional CSS classes |
| `node_ref` | `NodeRef<Div>` | -       | Node reference         |

### CardTitle

Heading section for card titles.

| Prop       | Type           | Default | Description            |
| ---------- | -------------- | ------- | ---------------------- |
| `children` | `Children`     | -       | Title content          |
| `class`    | `&'static str` | `""`    | Additional CSS classes |
| `node_ref` | `NodeRef<H2>`  | -       | Node reference         |

### CardActions

Action area at the bottom of the card.

| Prop       | Type           | Default | Description            |
| ---------- | -------------- | ------- | ---------------------- |
| `children` | `Children`     | -       | Action buttons/links   |
| `class`    | `&'static str` | `""`    | Additional CSS classes |
| `node_ref` | `NodeRef<Div>` | -       | Node reference         |
