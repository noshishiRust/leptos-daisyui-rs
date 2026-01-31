use crate::merge_classes;
use leptos::{html::Div, prelude::*};

/// # SpinButton Component
///
/// A numeric input with increment/decrement buttons.
/// Built on daisyUI's join and button components.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("join input join-item btn btn-primary");
/// ```
///
/// ## Node References
/// - `node_ref` - References the container div element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn SpinButton(
    /// Current numeric value
    #[prop(optional, into)]
    value: Signal<f64>,

    /// Minimum value
    #[prop(optional, into)]
    min: Signal<Option<f64>>,

    /// Maximum value
    #[prop(optional, into)]
    max: Signal<Option<f64>>,

    /// Step increment
    #[prop(optional, into)]
    step: Signal<f64>,

    /// Whether the spin button is disabled
    #[prop(optional, into)]
    disabled: Signal<bool>,

    /// Callback when value changes
    #[prop(optional, into)]
    on_change: Option<Callback<f64>>,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the container div
    #[prop(optional)]
    node_ref: NodeRef<Div>,
) -> impl IntoView {
    let step = Signal::derive(move || {
        let s = step.get();
        if s == 0.0 { 1.0 } else { s }
    });

    let can_decrement = Signal::derive(move || {
        if let Some(min_val) = min.get() {
            value.get() > min_val
        } else {
            true
        }
    });

    let can_increment = Signal::derive(move || {
        if let Some(max_val) = max.get() {
            value.get() < max_val
        } else {
            true
        }
    });

    let handle_decrement = move |_| {
        if can_decrement.get() && !disabled.get() {
            let new_val = value.get() - step.get();
            if let Some(ref callback) = on_change {
                callback.run(new_val);
            }
        }
    };

    let handle_increment = move |_| {
        if can_increment.get() && !disabled.get() {
            let new_val = value.get() + step.get();
            if let Some(ref callback) = on_change {
                callback.run(new_val);
            }
        }
    };

    let handle_input = move |ev| {
        if let Ok(val) = event_target_value(&ev).parse::<f64>()
            && let Some(ref callback) = on_change
        {
            callback.run(val);
        }
    };

    view! {
        <div node_ref=node_ref class=move || merge_classes!("join", class)>
            <button
                class="btn join-item"
                disabled=move || disabled.get() || !can_decrement.get()
                on:click=handle_decrement
            >
                "\u{2212}"
            </button>
            <input
                type="number"
                class="input input-bordered join-item w-24 text-center"
                value=move || value.get().to_string()
                disabled=disabled
                on:input=handle_input
            />
            <button
                class="btn join-item"
                disabled=move || disabled.get() || !can_increment.get()
                on:click=handle_increment
            >
                "+"
            </button>
        </div>
    }
}
