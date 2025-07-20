use super::style::{CheckboxColor, CheckboxSize};
use crate::merge_classes;
use leptos::prelude::*;

/// A reactive checkbox component that wraps daisyUI's checkbox styles.
///
/// This component provides a type-safe, reactive interface for creating checkboxes
/// with daisyUI styling. It supports color variants, size options, controlled state,
/// and event handling.
///
/// ## Features
///
/// - **Reactive State**: All props are reactive using Leptos signals
/// - **Color Variants**: Support for all daisyUI color themes
/// - **Size Options**: Five size variants from extra small to extra large
/// - **Event Handling**: Change events with boolean values
/// - **Accessibility**: Proper semantic checkbox with keyboard navigation
/// - **Customizable**: Additional CSS classes can be applied
///
/// ## Basic Usage
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::Checkbox;
///
/// #[component]
/// fn SimpleCheckbox() -> impl IntoView {
///     view! {
///         <Checkbox />
///     }
/// }
/// ```

#[component]
pub fn Checkbox(
    /// Color variant for the checkbox (reactive)
    #[prop(optional, into)]
    color: Signal<CheckboxColor>,

    /// Size variant for the checkbox (reactive)
    #[prop(optional, into)]
    size: Signal<CheckboxSize>,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
) -> impl IntoView {
    view! {
        <input
            type="checkbox"
            class=move || {
                merge_classes!(
                    "checkbox",
                    color.get().as_str(),
                    size.get().as_str(),
                    class
                )
            }
        />
    }
}
