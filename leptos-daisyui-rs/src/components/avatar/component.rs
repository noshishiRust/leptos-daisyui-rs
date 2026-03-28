use super::style::AvatarModifier;
use crate::merge_classes;
use leptos::{html::Div, prelude::*};

/// # Avatar Component
///
/// A reactive Leptos wrapper for daisyUI's avatar component that displays user profile
/// images, initials, or placeholder content in a consistent thumbnail format.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("avatar avatar-group avatar-online avatar-offline avatar-placeholder");
/// ```
///
/// ## Sizing and Shapes
///
/// Use Tailwind utility classes for custom sizing:
/// - `w-8 h-8` - Small avatar
/// - `w-16 h-16` - Large avatar
/// - `mask-squircle`, `mask-hexagon` - Custom shapes
///
/// ## Node References
/// - `node_ref` - References the avatar container `<div>` element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn Avatar(
    /// Status indicator or avatar type modifier
    #[prop(optional, into)]
    modifier: Signal<AvatarModifier>,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the avatar container
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Avatar content (images, text, icons, or other elements)
    children: Children,
) -> impl IntoView {
    view! {
        <div
            aria-label="avatar"
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "avatar",
                    modifier.get().as_str(),
                    class
                )
            }
        >
            {children()}
        </div>
    }
}

/// # Avatar Group Component
///
/// A reactive Leptos wrapper for daisyUI's avatar group component that displays
/// multiple avatars in a stacked or overlapping layout.
#[component]
pub fn AvatarGroup(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the avatar group container
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Avatar components to display in the group
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("avatar-group", class)>
            {children()}
        </div>
    }
}
