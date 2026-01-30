use super::style::{SliderColor, SliderSize};
use crate::merge_classes;
use leptos::{html::Input as HtmlInput, prelude::*};

/// # Slider Component
///
/// A range slider for selecting numeric values within a min/max range.
/// Built on HTML5 range input with daisyUI theming.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("range range-xs range-sm range-md range-lg range-primary range-secondary range-accent range-success range-warning range-info range-error");
/// ```
///
/// ## Node References
/// - `node_ref` - References the input element ([HTMLInputElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement))
#[component]
pub fn Slider(
    /// Current value of the slider
    #[prop(optional, into)]
    value: Signal<f64>,

    /// Minimum value
    #[prop(optional, into)]
    min: Signal<f64>,

    /// Maximum value
    #[prop(optional, into)]
    max: Signal<f64>,

    /// Step increment
    #[prop(optional, into)]
    step: Signal<f64>,

    /// Slider color variant
    #[prop(optional, into)]
    color: Signal<SliderColor>,

    /// Slider size variant
    #[prop(optional, into)]
    size: Signal<SliderSize>,

    /// Whether the slider is disabled
    #[prop(optional, into)]
    disabled: Signal<bool>,

    /// Callback when value changes
    #[prop(optional, into)]
    on_change: Option<Callback<f64>>,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the input element
    #[prop(optional)]
    node_ref: NodeRef<HtmlInput>,
) -> impl IntoView {
    // Default values
    let min = Signal::derive(move || {
        let m = min.get();
        if m == 0.0 {
            0.0
        } else {
            m
        }
    });

    let max = Signal::derive(move || {
        let m = max.get();
        if m == 0.0 {
            100.0
        } else {
            m
        }
    });

    let step = Signal::derive(move || {
        let s = step.get();
        if s == 0.0 {
            1.0
        } else {
            s
        }
    });

    view! {
        <input
            type="range"
            node_ref=node_ref
            value=move || value.get().to_string()
            min=move || min.get().to_string()
            max=move || max.get().to_string()
            step=move || step.get().to_string()
            disabled=disabled
            on:input=move |ev| {
                if let Ok(val) = event_target_value(&ev).parse::<f64>()
                    && let Some(ref callback) = on_change {
                        callback.run(val);
                    }
            }

            class=move || {
                merge_classes!(
                    "range",
                    color.get().as_str(),
                    size.get().as_str(),
                    class
                )
            }
        />
    }
}
