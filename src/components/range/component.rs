use super::style::{RangeColor, RangeSize};
use crate::merge_classes;
use leptos::{html::Input, prelude::*};

/// A range slider input component for selecting numeric values within a specified range.
///
/// The Range component provides a visual slider interface that allows users to select
/// numeric values by dragging a thumb along a track. It supports floating-point values
/// and provides precise control over the selectable range and step increments.
///
/// # Props
///
/// * `color` - The color variant of the range slider (optional, reactive)
/// * `size` - The size variant of the range slider (optional, reactive)
/// * `value` - The current value of the range slider (optional, reactive)
/// * `min` - The minimum value of the range (optional, reactive, defaults to 0.0)
/// * `max` - The maximum value of the range (optional, reactive, defaults to 100.0)
/// * `step` - The step increment for value changes (optional, reactive, defaults to 1.0)
/// * `class` - Additional CSS classes to apply (optional)
/// * `node_ref` - Reference to the underlying HTML input element (optional)
/// * `on_input` - Callback function called when the slider value changes (optional)
///
/// # CSS Classes
///
/// This component applies the following CSS classes:
/// - Base: `range`
/// - Color: Applied from `RangeColor` enum
/// - Size: Applied from `RangeSize` enum
/// - Additional: Any classes provided via the `class` prop
///
/// # Example
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::range::{Range, RangeColor, RangeSize};
///
/// #[component]
/// fn VolumeControl() -> impl IntoView {
///     let (volume, set_volume) = signal(50.0);
///
///     view! {
///         <div class="form-control w-full max-w-xs">
///             <label class="label">
///                 <span class="label-text">"Volume"</span>
///                 <span class="label-text-alt">{move || format!("{:.0}%", volume.get())}</span>
///             </label>
///             <Range
///                 color=RangeColor::Primary
///                 size=RangeSize::Md
///                 value=volume
///                 min=0.0
///                 max=100.0
///                 step=1.0
///                 on_input=Box::new(move |val| set_volume.set(val))
///             />
///             <div class="flex justify-between text-xs px-2">
///                 <span>"0%"</span>
///                 <span>"100%"</span>
///             </div>
///         </div>
///     }
/// }
/// ```
///
/// # Precision Control Example
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::range::{Range, RangeColor};
///
/// #[component]
/// fn PrecisionSlider() -> impl IntoView {
///     let (value, set_value) = signal(2.5);
///
///     view! {
///         <Range
///             color=RangeColor::Accent
///             value=value
///             min=0.0
///             max=10.0
///             step=0.1
///             on_input=Box::new(move |val| set_value.set(val))
///         />
///     }
/// }
/// ```
///
/// # Accessibility
///
/// - Uses semantic `<input type="range">` element for proper screen reader support
/// - Supports keyboard navigation (arrow keys for fine adjustment, Page Up/Down for larger steps)
/// - Properly announces current value, minimum, and maximum to screen readers
/// - Compatible with assistive technologies
/// - Supports focus management and visual focus indicators
#[component]
pub fn Range(
    /// The color variant of the range slider
    #[prop(optional, into)]
    color: Signal<RangeColor>,
    /// The size variant of the range slider
    #[prop(optional, into)]
    size: Signal<RangeSize>,
    /// The current value of the range slider
    #[prop(optional, into)]
    value: Signal<f64>,
    /// The minimum value of the range
    #[prop(optional, into)]
    min: Signal<f64>,
    /// The maximum value of the range
    #[prop(optional, into)]
    max: Signal<f64>,
    /// The step increment for value changes
    #[prop(optional, into)]
    step: Signal<f64>,
    /// Additional CSS classes to apply
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Reference to the underlying HTML input element
    #[prop(optional)]
    node_ref: NodeRef<Input>,
    /// Callback function called when the slider value changes
    #[prop(optional)]
    on_input: Option<Box<dyn Fn(f64)>>,
) -> impl IntoView {
    view! {
        <input
            node_ref=node_ref
            type="range"
            class=move || {
                merge_classes!(
                    "range",
                color.get().as_str(),
                size.get().as_str(),
                class
                )
            }
            value=value
            min=min
            max=max
            step=step
            on:input=move |ev| {
                if let Some(handler) = &on_input {
                    if let Ok(val) = event_target_value(&ev).parse::<f64>() {
                        handler(val);
                    }
                }
            }
        />
    }
}
