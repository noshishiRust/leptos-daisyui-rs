use super::style::{TextAlign, TextColor, TextSize, TextWeight};
use leptos::{attr::Attribute, html, prelude::*};

/// A typography component for styled text content.
///
/// # Example
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <Text size=TextSize::Large weight=TextWeight::Bold color=TextColor::Primary>
///             "Important heading"
///         </Text>
///     }
/// }
/// ```
///
/// # CSS Classes
/// Add to your `input.css`:
/// ```css
/// @source inline("text-xs text-sm text-base text-lg text-xl text-2xl text-3xl");
/// @source inline("font-thin font-normal font-medium font-semibold font-bold");
/// @source inline("text-primary text-secondary text-accent text-neutral text-info text-success text-warning text-error");
/// @source inline("truncate text-left text-center text-right text-justify");
/// ```
#[component]
pub fn Text(
    /// Size of the text
    #[prop(optional, into)]
    size: Signal<TextSize>,
    /// Weight of the text
    #[prop(optional, into)]
    weight: Signal<TextWeight>,
    /// Color of the text
    #[prop(optional, into)]
    color: Signal<TextColor>,
    /// Alignment of the text
    #[prop(optional, into)]
    align: Signal<TextAlign>,
    /// Truncate text with ellipsis
    #[prop(optional, into)]
    truncate: Signal<bool>,
    /// Text content
    children: Children,
    /// Reference to the underlying DOM node. See [MDN docs](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/span)
    #[prop(optional)]
    node_ref: NodeRef<html::Span>,
    /// Spread additional attributes onto the `<span>` element
    #[prop(attrs)]
    attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let class = move || {
        let mut classes = vec![
            size.get().as_str(),
            weight.get().as_str(),
            color.get().as_str(),
            align.get().as_str(),
        ];
        if truncate.get() {
            classes.push("truncate");
        }
        classes.join(" ")
    };

    view! {
        <span
            node_ref=node_ref
            class=move || class()
            {..attributes}
        >
            {children()}
        </span>
    }
}
