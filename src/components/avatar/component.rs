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
/// ### daisyUI Classes
/// - `avatar` - Base avatar class
/// - `avatar-online` - Shows online status indicator
/// - `avatar-offline` - Shows offline status indicator
/// - `avatar-placeholder` - Placeholder avatar styling
///
/// ## Usage Example
///
/// ```rust
/// use leptos::*;
/// use leptos_daisyui::avatar::*;
///
/// view! {
///     // Avatar with image and online status
///     <Avatar modifier=AvatarModifier::Online>
///         <div class="w-16 rounded-full">
///             <img src="/profile.jpg" alt="User profile" />
///         </div>
///     </Avatar>
///     
///     // Avatar with initials
///     <Avatar>
///         <div class="w-12 rounded-full bg-primary text-primary-content flex items-center justify-center">
///             <span class="text-lg font-bold">"AB"</span>
///         </div>
///     </Avatar>
///     
///     // Placeholder avatar
///     <Avatar modifier=AvatarModifier::Placeholder>
///         <div class="w-10 rounded-full bg-neutral text-neutral-content">
///             <svg class="w-6 h-6">{/* user icon */}</svg>
///         </div>
///     </Avatar>
/// }
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
            <div>{children()}</div>
        </div>
    }
}

/// # Avatar Group Component
///
/// A reactive Leptos wrapper for daisyUI's avatar group component that displays
/// multiple avatars in a stacked or overlapping layout.
///
/// ## Usage Example
///
/// ```rust
/// use leptos::*;
/// use leptos_daisyui::avatar::*;
///
/// view! {
///     <AvatarGroup>
///         <Avatar>
///             <div class="w-12 rounded-full">
///                 <img src="/user1.jpg" alt="User 1" />
///             </div>
///         </Avatar>
///         <Avatar>
///             <div class="w-12 rounded-full">
///                 <img src="/user2.jpg" alt="User 2" />
///             </div>
///         </Avatar>
///         <Avatar>
///             <div class="w-12 rounded-full">
///                 <img src="/user3.jpg" alt="User 3" />
///             </div>
///         </Avatar>
///         // More avatars will automatically overlap
///     </AvatarGroup>
/// }
/// ```
#[component]
pub fn AvatarGroup(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Avatar components to display in the group
    children: Children,
) -> impl IntoView {
    view! { <div class=move || merge_classes!("avatar-group", class)>{children()}</div> }
}
