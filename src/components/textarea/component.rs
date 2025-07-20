use super::style::{TextareaColor, TextareaSize};
use crate::merge_classes;
use leptos::{html::Textarea as HtmlTextarea, prelude::*};

/// A multi-line text input component for entering longer text content.
///
/// The Textarea component provides a styled multi-line text input that is perfect
/// for forms, comments, descriptions, and any other use case requiring longer text input.
/// It supports various styling options and integrates well with forms and validation.
///
/// # Props
///
/// * `color` - The color variant of the textarea (optional, reactive)
/// * `size` - The size variant of the textarea (optional, reactive)
/// * `class` - Additional CSS classes to apply (optional)
/// * `node_ref` - Reference to the underlying HTML textarea element (optional)
///
/// # CSS Classes
///
/// This component applies the following CSS classes:
/// - Base: `textarea`
/// - Color: Applied from `TextareaColor` enum
/// - Size: Applied from `TextareaSize` enum
/// - Additional: Any classes provided via the `class` prop
///
/// # Example
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::textarea::{Textarea, TextareaColor, TextareaSize};
///
/// #[component]
/// fn CommentForm() -> impl IntoView {
///     view! {
///         <div class="form-control">
///             <label class="label">
///                 <span class="label-text">"Your comment"</span>
///                 <span class="label-text-alt">"Optional"</span>
///             </label>
///             <Textarea
///                 color=TextareaColor::Primary
///                 size=TextareaSize::Md
///                 class="h-24"
///             />
///             <label class="label">
///                 <span class="label-text-alt">"Your comment helps us improve."</span>
///             </label>
///         </div>
///     }
/// }
/// ```
///
/// # Advanced Example with State
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::textarea::{Textarea, TextareaColor};
///
/// #[component]
/// fn MessageEditor() -> impl IntoView {
///     let (message, set_message) = signal(String::new());
///     let char_count = move || message.get().len();
///     let max_chars = 500;
///     
///     let color = move || {
///         if char_count() > max_chars {
///             TextareaColor::Error
///         } else if char_count() > max_chars * 3 / 4 {
///             TextareaColor::Warning
///         } else {
///             TextareaColor::Default
///         }
///     };
///
///     view! {
///         <div class="form-control">
///             <label class="label">
///                 <span class="label-text">"Message"</span>
///                 <span class="label-text-alt">
///                     {move || format!("{}/{}", char_count(), max_chars)}
///                 </span>
///             </label>
///             <Textarea
///                 color=move || color()
///                 class="h-32 resize-none"
///             />
///         </div>
///     }
/// }
/// ```
///
/// # Accessibility
///
/// - Uses semantic `<textarea>` element for proper screen reader support
/// - Supports keyboard navigation and standard text editing shortcuts
/// - Properly integrates with labels and form controls
/// - Compatible with form validation and submission
/// - Supports focus management and visual focus indicators
/// - Respects user preferences for text scaling and contrast
#[component]
pub fn Textarea(
    /// The color variant of the textarea
    #[prop(optional, into)]
    color: Signal<TextareaColor>,
    /// The size variant of the textarea
    #[prop(optional, into)]
    size: Signal<TextareaSize>,
    /// Additional CSS classes to apply
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Reference to the underlying HTML textarea element
    #[prop(optional)]
    node_ref: NodeRef<HtmlTextarea>,
) -> impl IntoView {
    view! {
        <textarea
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "textarea",
                color.get().as_str(),
                size.get().as_str(),
                class
                )
            }
        />
    }
}
