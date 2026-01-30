use leptos::{attr::Attribute, html, prelude::*};

/// A component for displaying code with optional syntax highlighting support.
///
/// # Example
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <Code language="rust">
///             "fn main() {\n    println!(\"Hello, world!\");\n}"
///         </Code>
///     }
/// }
/// ```
///
/// # CSS Classes
/// Add to your `input.css`:
/// ```css
/// @source inline("mockup-code bg-base-300 rounded-box");
/// ```
#[component]
pub fn Code(
    /// Programming language for syntax highlighting class
    #[prop(optional, into)]
    language: Signal<String>,
    /// Whether to show line numbers
    #[prop(optional, into)]
    line_numbers: Signal<bool>,
    /// Code content
    children: Children,
    /// Reference to the underlying DOM node. See [MDN docs](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/div)
    #[prop(optional)]
    node_ref: NodeRef<html::Div>,
    /// Spread additional attributes onto the `<div>` element
    #[prop(attrs)]
    attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let code_class = move || {
        let lang = language.get();
        if !lang.is_empty() {
            format!("language-{}", lang)
        } else {
            String::new()
        }
    };

    view! {
        <div
            node_ref=node_ref
            class="mockup-code bg-base-300 rounded-box"
            {..attributes}
        >
            <pre data-prefix=move || if line_numbers.get() { ">" } else { "" }>
                <code class=code_class>
                    {children()}
                </code>
            </pre>
        </div>
    }
}
