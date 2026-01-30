use crate::merge_classes;
use leptos::{html::Div, prelude::*};

/// # Space Component
///
/// A spacing utility component for consistent gaps between elements.
/// Wraps children with specified spacing using flexbox.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("flex flex-row flex-col gap-1 gap-2 gap-4 gap-8 gap-16");
/// ```
///
/// ## Node References
/// - `node_ref` - References the container div element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn Space(
    /// Direction of spacing (horizontal or vertical)
    #[prop(optional, into)]
    vertical: Signal<bool>,

    /// Gap size between elements
    #[prop(optional, into)]
    size: Signal<u8>,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the container div
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Child elements to space
    children: Children,
) -> impl IntoView {
    let direction_class = Signal::derive(move || {
        if vertical.get() {
            "flex-col"
        } else {
            "flex-row"
        }
    });

    let gap_class = Signal::derive(move || {
        let s = size.get();
        match s {
            0 => "",
            1 => "gap-1",
            2 => "gap-2",
            4 => "gap-4",
            8 => "gap-8",
            16 => "gap-16",
            _ => "gap-4", // default
        }
    });

    view! {
        <div
            node_ref=node_ref
            class=move || merge_classes!("flex", direction_class.get(), gap_class.get(), class)
        >
            {children()}
        </div>
    }
}
