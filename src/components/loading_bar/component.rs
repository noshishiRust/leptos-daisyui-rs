use super::style::{LoadingBarColor, LoadingBarSize};
use leptos::{html, prelude::*};

/// A horizontal loading bar component for showing progress.
///
/// # Example
/// ```rust,ignore
/// use leptos::prelude::*;
/// use leptos_daisyui_rs::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     let (progress, set_progress) = signal(50.0);
///
///     view! {
///         <LoadingBar
///             progress=progress
///             color=LoadingBarColor::Primary
///             size=LoadingBarSize::Medium
///         />
///     }
/// }
/// ```
///
/// # CSS Classes
/// Add to your `input.css`:
/// ```css
/// @source inline("w-full h-1 h-2 h-3 h-4 rounded-full overflow-hidden");
/// @source inline("bg-primary bg-secondary bg-accent bg-success bg-warning bg-error bg-info");
/// @source inline("bg-base-200 transition-all duration-300");
/// ```
#[component]
pub fn LoadingBar(
    /// Progress percentage (0-100)
    #[prop(into)]
    progress: Signal<f64>,
    /// Color of the loading bar
    #[prop(optional, into)]
    color: Signal<LoadingBarColor>,
    /// Size of the loading bar
    #[prop(optional, into)]
    size: Signal<LoadingBarSize>,
    /// Whether to show indeterminate animation
    #[prop(optional, into)]
    indeterminate: Signal<bool>,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Reference to the underlying DOM node
    #[prop(optional)]
    node_ref: NodeRef<html::Div>,
) -> impl IntoView {
    let container_class = move || {
        let mut classes = vec!["w-full", "rounded-full", "overflow-hidden", "bg-base-200"];
        classes.push(size.get().as_str());
        if !class.is_empty() {
            classes.push(class);
        }
        classes.join(" ")
    };

    let bar_class = move || {
        let mut classes = vec!["h-full", "transition-all", "duration-300"];
        classes.push(color.get().as_str());
        classes.join(" ")
    };

    let bar_style = move || {
        if indeterminate.get() {
            "width: 30%; animation: loading-bar-slide 1.5s ease-in-out infinite".to_string()
        } else {
            let clamped = progress.get().clamp(0.0, 100.0);
            format!("width: {}%", clamped)
        }
    };

    view! {
        <div
            node_ref=node_ref
            class=container_class
        >
            <div
                class=bar_class
                style=bar_style
            ></div>
        </div>
    }
}
