use super::style::{ToggleColor, ToggleSize};
use crate::merge_classes;
use leptos::{html::Input, prelude::*};

/// A toggle component that can be used as a checkbox with custom styles.
///
///  Add `@source inline("toggle toggle-primary toggle-secondary toggle-accent toggle-ghost toggle-info toggle-success toggle-warning toggle-error toggle-neutral toggle-xs toggle-sm toggle-md toggle-lg toggle-xl");`
///  to input.css
///
/// This cimpponet is a wrapper an HTML `<input type="checkbox">` element,
/// you can spread [HTMLInputElement](https://developer.mozilla.org/ja/docs/Web/API/HTMLInputElement) attributes to it.
#[component]
pub fn Toggle(
    #[prop(optional, into)] color: Signal<ToggleColor>,
    #[prop(optional, into)] size: Signal<ToggleSize>,
    #[prop(optional, into)] checked: Signal<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Input>,
) -> impl IntoView {
    view! {
        <input
            node_ref=node_ref
            type="checkbox"
            checked=checked
            disabled=disabled
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
