use super::style::{RadioColor, RadioSize};
use crate::merge_classes;
use leptos::{html::Input, prelude::*};

/// A radio button input component for single-choice selection.
///
/// Radio buttons are used to allow users to select one option from a set of choices.
/// Multiple radio buttons with the same `name` attribute form a radio group where
/// only one option can be selected at a time.
///
/// # Props
///
/// * `color` - The color variant of the radio button (optional, reactive)
/// * `size` - The size variant of the radio button (optional, reactive)
/// * `checked` - Whether the radio button is checked (optional, reactive)
/// * `disabled` - Whether the radio button is disabled (optional, reactive)
/// * `name` - The name attribute for grouping radio buttons (optional)
/// * `value` - The value attribute of the radio button (optional)
/// * `class` - Additional CSS classes to apply (optional)
/// * `node_ref` - Reference to the underlying HTML input element (optional)
/// * `on_change` - Callback function called when the radio button state changes (optional)
///
/// # CSS Classes
///
/// This component applies the following CSS classes:
/// - Base: `radio`
/// - Color: Applied from `RadioColor` enum
/// - Size: Applied from `RadioSize` enum
/// - Additional: Any classes provided via the `class` prop
///
/// # Example
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::radio::{Radio, RadioColor, RadioSize};
///
/// #[component]
/// fn RadioGroup() -> impl IntoView {
///     let (selected_option, set_selected_option) = signal(String::new());
///
///     let handle_change = |value: &'static str| {
///         Box::new(move |checked: bool| {
///             if checked {
///                 set_selected_option.set(value.to_string());
///             }
///         })
///     };
///
///     view! {
///         <div class="space-y-2">
///             <label class="flex items-center space-x-2">
///                 <Radio
///                     name="options"
///                     value="option1"
///                     color=RadioColor::Primary
///                     size=RadioSize::Md
///                     checked=move || selected_option.get() == "option1"
///                     on_change=handle_change("option1")
///                 />
///                 <span>"Option 1"</span>
///             </label>
///             <label class="flex items-center space-x-2">
///                 <Radio
///                     name="options"
///                     value="option2"
///                     color=RadioColor::Secondary
///                     size=RadioSize::Md
///                     checked=move || selected_option.get() == "option2"
///                     on_change=handle_change("option2")
///                 />
///                 <span>"Option 2"</span>
///             </label>
///         </div>
///     }
/// }
/// ```
///
/// # Accessibility
///
/// - Uses semantic `<input type="radio">` element for proper screen reader support
/// - Supports keyboard navigation (arrow keys to navigate between options in the same group)
/// - Properly implements the radio button interaction pattern
/// - Compatible with form validation and submission
/// - Supports focus management and visual focus indicators
#[component]
pub fn Radio(
    /// The color variant of the radio button
    #[prop(optional, into)]
    color: Signal<RadioColor>,
    /// The size variant of the radio button
    #[prop(optional, into)]
    size: Signal<RadioSize>,
    /// Whether the radio button is checked
    #[prop(optional, into)]
    checked: Signal<bool>,
    /// Whether the radio button is disabled
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// The name attribute for grouping radio buttons
    #[prop(optional)]
    name: Option<&'static str>,
    /// The value attribute of the radio button
    #[prop(optional)]
    value: Option<&'static str>,
    /// Additional CSS classes to apply
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Reference to the underlying HTML input element
    #[prop(optional)]
    node_ref: NodeRef<Input>,
    /// Callback function called when the radio button state changes
    #[prop(optional)]
    on_change: Option<Box<dyn Fn(bool)>>,
) -> impl IntoView {
    view! {
        <input
            node_ref=node_ref
            type="radio"
            name=name
            value=value
            class=move || {
                merge_classes!(
                    "radio",
                color.get().as_str(),
                size.get().as_str(),
                class
                )
            }
            prop:checked=checked
            prop:disabled=disabled
            on:change=move |ev| {
                if let Some(handler) = &on_change {
                    handler(event_target_checked(&ev));
                }
            }
        />
    }
}
