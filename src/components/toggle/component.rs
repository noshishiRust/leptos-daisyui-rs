use super::style::{ToggleColor, ToggleSize};
use crate::merge_classes;
use leptos::{html::Input, prelude::*};

/// # Toggle Component
///
/// A checkbox styled as a switch that provides binary state selection.
/// Uses semantic checkbox input with toggle appearance.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("toggle toggle-primary toggle-secondary toggle-accent toggle-neutral toggle-success toggle-warning toggle-info toggle-error toggle-xs toggle-sm toggle-md toggle-lg toggle-xl");
/// ```
///
/// ## Node References
/// - `node_ref` - References the top `<input>` element ([HTMLInputElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement))
#[component]
pub fn Toggle(
    /// Color theme of the toggle
    #[prop(optional, into)]
    color: Signal<ToggleColor>,

    /// Size of the toggle
    #[prop(optional, into)]
    size: Signal<ToggleSize>,

    /// Additional CSS classes to apply to the toggle
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the top `<input>` element
    #[prop(optional)]
    node_ref: NodeRef<Input>,
) -> impl IntoView {
    view! {
        <input
            node_ref=node_ref
            type="checkbox"
            class=move || {
                merge_classes!(
                    "toggle",
                    color.get().as_str(),
                    size.get().as_str(),
                    class
                )
            }
        />
    }
}
