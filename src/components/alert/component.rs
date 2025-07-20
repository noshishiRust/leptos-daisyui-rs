use super::style::{AlertColor, AlertDirection, AlertStyle};
use crate::merge_classes;
use leptos::{html::Div, prelude::*};

/// # Alert Component
///
/// A reactive Leptos wrapper for daisyUI's alert component that displays important messages,
/// notifications, and contextual feedback to users.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("alert alert-outline alert-dash alert-soft alert-info alert-success alert-warning alert-error alert-vertical alert-horizontal");
/// ```
///
/// ## Usage Example
///
/// ```rust
/// use leptos::*;
/// use leptos_daisyui::alert::*;
///
/// #[component]
/// fn AlertDemo() -> impl IntoView {
///     let (show_success, set_show_success) = signal(false);
///
///     view! {
///         // Success alert with horizontal layout
///         <Alert
///             color=AlertColor::Success
///             direction=AlertDirection::Horizontal
///         >
///             <svg class="w-6 h-6">{/* success icon */}</svg>
///             <span>"Your changes have been saved successfully!"</span>
///         </Alert>
///
///         // Error alert with outline style
///         <Alert
///             style=AlertStyle::Outline
///             color=AlertColor::Error
///         >
///             <div>
///                 <h3 class="font-bold">"Error!"</h3>
///                 <div class="text-xs">"Please check your input and try again."</div>
///             </div>
///         </Alert>
///
///         // Warning alert with vertical layout
///         <Alert
///             color=AlertColor::Warning
///             direction=AlertDirection::Vertical
///         >
///             <h3>"Important Notice"</h3>
///             <p>"This action will permanently delete your data."</p>
///             <div>
///                 <button class="btn btn-sm btn-primary">"Confirm"</button>
///                 <button class="btn btn-sm btn-ghost">"Cancel"</button>
///             </div>
///         </Alert>
///     }
/// }
/// ```
///
/// ## Node References
/// - `node_ref` - References the alert `<div>` element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn Alert(
    /// Visual style of the alert
    #[prop(optional, into)]
    style: Signal<AlertStyle>,

    /// Semantic color of the alert
    #[prop(optional, into)]
    color: Signal<AlertColor>,

    /// Layout direction of alert content
    #[prop(optional, into)]
    direction: Signal<AlertDirection>,

    /// Node reference for the alert element
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Alert content (text, icons, buttons, or other elements)
    children: Children,
) -> impl IntoView {
    view! {
        <div
            role="alert"
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "alert",
                    style.get().as_str(),
                    color.get().as_str(),
                    direction.get().as_str(),
                    class
                )
            }
        >
            {children()}
        </div>
    }
}
