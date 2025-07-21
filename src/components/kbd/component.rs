use super::style::KbdSize;
use crate::merge_classes;
use leptos::{html::Kbd as HtmlKbd, prelude::*};

/// # Kbd Component
///
/// A component for displaying keyboard shortcuts and key combinations.
/// Renders keys with appropriate styling to indicate user input.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("kbd kbd-xs kbd-sm kbd-md kbd-lg kbd-xl");
/// ```
///
/// ## Node References
/// - `node_ref` - References the `<kbd>` element ([HTMLElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement))
#[component]
pub fn Kbd(
    /// Size of the keyboard key display
    #[prop(optional, into)] 
    size: Signal<KbdSize>,

    /// Additional CSS classes to apply to the kbd element
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the `<kbd>` element
    #[prop(optional)] 
    node_ref: NodeRef<HtmlKbd>,

    /// Key text or symbol content
    children: Children,
) -> impl IntoView {
    view! {
        <kbd
            node_ref=node_ref
            class=move || {
                merge_classes!("kbd",
                size.get().as_str(),
                class)
            }
        >
            {children()}
        </kbd>
    }
}
