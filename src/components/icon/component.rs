use super::style::IconSize;
use leptos::{html, prelude::*};

/// An icon component with Lucide icon support.
///
/// This component provides a wrapper for displaying icons from the Lucide icon library.
/// You need to include Lucide icons in your project separately.
///
/// # Example
/// ```rust,ignore
/// use leptos::prelude::*;
/// use leptos_daisyui_rs::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <Icon name="heart" size=IconSize::Large color="text-error" />
///         <Icon name="star" size=IconSize::Medium />
///     }
/// }
/// ```
///
/// # Setup Lucide Icons
///
/// Include the Lucide icon library in your `index.html`:
/// ```html
/// <script src="https://unpkg.com/lucide@latest"></script>
/// <script>
///   lucide.createIcons();
/// </script>
/// ```
///
/// # CSS Classes
/// Add to your `input.css`:
/// ```css
/// @source inline("w-4 h-4 w-5 h-5 w-6 h-6 w-8 h-8 w-12 h-12");
/// @source inline("inline-block");
/// ```
#[component]
pub fn Icon(
    /// Icon name from Lucide icons (e.g., "heart", "star", "user")
    #[prop(into)]
    name: Signal<String>,
    /// Size of the icon
    #[prop(optional, into)]
    size: Signal<IconSize>,
    /// Color class for the icon (e.g., "text-primary", "text-error")
    #[prop(optional, into)]
    color: Signal<String>,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Reference to the underlying DOM node
    #[prop(optional)]
    node_ref: NodeRef<html::I>,
) -> impl IntoView {
    let computed_class = move || {
        let mut classes = vec!["inline-block", size.get().as_str()];
        let color_val = color.get();
        if !color_val.is_empty() {
            classes.push(&color_val);
        }
        if !class.is_empty() {
            classes.push(class);
        }
        classes.join(" ")
    };

    view! {
        <i
            node_ref=node_ref
            data-lucide=move || name.get()
            class=computed_class
        ></i>
    }
}
