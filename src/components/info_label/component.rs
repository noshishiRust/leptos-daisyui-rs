use leptos::{attr::Attribute, html, prelude::*};

/// An information label component with icon and customizable styling.
///
/// # Example
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <InfoLabel icon="ℹ️" color=InfoLabelColor::Info>
///             "This is an informational message"
///         </InfoLabel>
///     }
/// }
/// ```
///
/// # CSS Classes
/// Add to your `input.css`:
/// ```css
/// @source inline("bg-info bg-success bg-warning bg-error bg-neutral");
/// @source inline("text-info-content text-success-content text-warning-content text-error-content text-neutral-content");
/// @source inline("rounded-lg p-4 flex items-center gap-2");
/// ```
#[component]
pub fn InfoLabel(
    /// Icon or emoji to display
    #[prop(optional, into)]
    icon: Signal<String>,
    /// Color variant
    #[prop(optional, into)]
    color: Signal<InfoLabelColor>,
    /// Label content
    children: Children,
    /// Reference to the underlying DOM node. See [MDN docs](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/div)
    #[prop(optional)]
    node_ref: NodeRef<html::Div>,
    /// Spread additional attributes onto the `<div>` element
    #[prop(attrs)]
    attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let class_str = move || {
        format!(
            "rounded-lg p-4 flex items-center gap-2 {} {}",
            color.get().bg_class(),
            color.get().text_class()
        )
    };

    view! {
        <div
            node_ref=node_ref
            class=move || class_str()
            {..attributes}
        >
            <Show when=move || !icon.get().is_empty()>
                <span class="text-xl">{move || icon.get()}</span>
            </Show>
            <span>{children()}</span>
        </div>
    }
}

/// Color variants for InfoLabel
#[derive(Clone, Debug, Default, PartialEq)]
pub enum InfoLabelColor {
    /// Info color (bg-info, text-info-content)
    #[default]
    Info,
    /// Success color (bg-success, text-success-content)
    Success,
    /// Warning color (bg-warning, text-warning-content)
    Warning,
    /// Error color (bg-error, text-error-content)
    Error,
    /// Neutral color (bg-neutral, text-neutral-content)
    Neutral,
}

impl InfoLabelColor {
    pub fn bg_class(&self) -> &'static str {
        match self {
            Self::Info => "bg-info",
            Self::Success => "bg-success",
            Self::Warning => "bg-warning",
            Self::Error => "bg-error",
            Self::Neutral => "bg-neutral",
        }
    }

    pub fn text_class(&self) -> &'static str {
        match self {
            Self::Info => "text-info-content",
            Self::Success => "text-success-content",
            Self::Warning => "text-warning-content",
            Self::Error => "text-error-content",
            Self::Neutral => "text-neutral-content",
        }
    }
}
