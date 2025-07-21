use crate::merge_classes;
use leptos::{html::Span, prelude::*};

/// # Countdown Component
///
/// A reactive wrapper for daisyUI's countdown component that displays animated numbers
/// with smooth transitions when values change between 0-99.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("countdown");
/// ```
///
/// ## Node References
/// - `node_ref` - References the top `<span>` element ([HTMLSpanElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSpanElement))
#[component]
pub fn Countdown(
    /// Additional CSS classes to apply to the countdown container
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the countdown `<span>` element
    #[prop(optional)]
    node_ref: NodeRef<Span>,

    /// Child components, typically multiple [`CountdownValue`] elements
    children: Children,
) -> impl IntoView {
    view! {
        <span node_ref=node_ref class=move || merge_classes!("countdown", class)>
            {children()}
        </span>
    }
}

/// # Countdown Value Component
///
/// An individual animated number within a countdown display. The value animates
/// smoothly when changed and must be between 0-99 for proper animation.
///
/// ## Node References
/// - `node_ref` - References the top `<span>` element ([HTMLSpanElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSpanElement))
#[component]
pub fn CountdownValue(
    /// Reactive signal containing the numeric value (0-99) to display
    value: Signal<u8>,

    /// Additional CSS classes to apply to the value element
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the value `<span>` element
    #[prop(optional)]
    node_ref: NodeRef<Span>,

    /// Optional aria-label for accessibility, defaults to the numeric value
    #[prop(optional, into)]
    aria_label: Signal<Option<String>>,
) -> impl IntoView {
    view! {
        <span
            node_ref=node_ref
            class=class
            style=move || format!("--value:{};", value.get())
            aria-live="polite"
            aria-label=move || {
                if let Some(label) = aria_label.get() { label } else { value.get().to_string() }
            }
        >
            {move || value.get()}
        </span>
    }
}
