use super::style::ProgressColor;
use crate::merge_classes;
use leptos::{html::Progress as HtmlProgress, prelude::*};

/// # Progress Component
///
/// A reactive Leptos wrapper for daisyUI's progress component that displays the completion
/// progress of tasks or operations with visual progress bars.
///
/// ## Overview
///
/// The Progress component renders a `<progress>` element with daisyUI styling. It supports
/// different colors for semantic meaning and can display determinate progress (with value)
/// or indeterminate progress (without value for loading states).
///
/// ## CSS Classes Used
///
/// ### Add to `input.css`
/// ```css
/// @source inline("progress progress-primary progress-secondary progress-accent progress-info progress-success progress-warning progress-error");
/// ```
///
/// ### daisyUI Classes
/// - `progress` - Base progress bar class
/// - Color classes: `progress-{color}` (primary, secondary, accent, info, success, warning, error)
///
/// ## Usage Example
///
/// ```rust
/// use leptos::*;
/// use leptos_daisyui::progress::*;
///
/// #[component]
/// fn ProgressDemo() -> impl IntoView {
///     let (progress, set_progress) = signal(0.0);
///     let (loading, set_loading) = signal(false);
///
///     view! {
///         // Determinate progress with value
///         <Progress
///             color=ProgressColor::Primary
///             value=progress
///             max=100.0
///         />
///         
///         // Indeterminate loading progress
///         <Progress
///             color=ProgressColor::Info
///             max=100.0
///         />
///         
///         // Success progress
///         <Progress
///             color=ProgressColor::Success
///             value=100.0
///             max=100.0
///         />
///         
///         // Error progress
///         <Progress
///             color=ProgressColor::Error
///             value=25.0
///             max=100.0
///         />
///     }
/// }
/// ```
///
/// ## Node References
/// - `node_ref` - References the `<progress>` element ([HTMLProgressElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLProgressElement))
#[component]
pub fn Progress(
    /// Progress bar color variant
    ///
    /// Controls the semantic color scheme using daisyUI color classes.
    /// - `ProgressColor::Default` - No color class (uses default styling)
    /// - `ProgressColor::Primary` - Primary brand color
    /// - `ProgressColor::Success` - Success/completion color
    /// - `ProgressColor::Error` - Error/failure color
    /// - etc.
    #[prop(optional, into)]
    color: Signal<ProgressColor>,

    /// Current progress value
    ///
    /// The current progress value. When not provided, the progress bar will
    /// display an indeterminate loading state. The value should be between 0 and max.
    #[prop(optional, into)]
    value: MaybeProp<f64>,

    /// Maximum progress value
    ///
    /// The maximum value for the progress bar. Defaults to 100.0 when using Signal::default().
    /// The progress percentage is calculated as (value / max) * 100.
    #[prop(optional, into)]
    max: Signal<f64>,

    /// Node reference for the progress element
    #[prop(optional)]
    node_ref: NodeRef<HtmlProgress>,

    /// Additional CSS classes to apply
    #[prop(optional, into)]
    class: &'static str,
) -> impl IntoView {
    view! {
        <progress
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "progress",
                    color.get().as_str(),
                    class
                )
            }
            value=move || value.get()
            max=max
        />
    }
}
