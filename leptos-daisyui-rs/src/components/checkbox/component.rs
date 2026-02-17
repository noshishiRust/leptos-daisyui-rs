use super::style::{CheckboxColor, CheckboxSize};
use crate::merge_classes;
use leptos::{html::Input as HtmlInput, prelude::*};

/// # Checkbox Component
///
/// ### Add to `input.css`
/// ```css
/// @source inline("checkbox checkbox-primary checkbox-secondary checkbox-accent checkbox-neutral checkbox-success checkbox-warning checkbox-info checkbox-error");
/// ```
///
/// ## Node References
/// - `node_ref` - References the `<input>` element ([HTMLInputElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement))
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

    /// Node reference for the input element
    #[prop(optional)]
    node_ref: NodeRef<HtmlInput>,
) -> impl IntoView {
    view! {
        <input
            node_ref=node_ref
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
