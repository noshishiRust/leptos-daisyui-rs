use crate::merge_classes;
use leptos::{html::Pre, prelude::*};

/// # Code Component
///
/// Code syntax display component with optional line numbers and copy functionality.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("mockup-code bg-base-300 rounded-box p-4 overflow-x-auto");
/// ```
#[component]
pub fn Code(
    #[prop(optional, into)] language: Signal<String>,
    #[prop(optional, into)] show_line_numbers: Signal<bool>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Pre>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="mockup-code bg-base-300 rounded-box">
            <pre
                node_ref=node_ref
                data-prefix=move || if show_line_numbers.get() { "$" } else { "" }
                class=move || merge_classes!("overflow-x-auto", class)
            >
                <code class=move || {
                    let lang = language.get();
                    if !lang.is_empty() { format!("language-{}", lang) } else { String::new() }
                }>
                    {children()}
                </code>
            </pre>
        </div>
    }
}
