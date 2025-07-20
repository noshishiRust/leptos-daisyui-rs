use super::style::{InputColor, InputSize, InputStyle};
use crate::merge_classes;
use leptos::{html::Input as HtmlInput, prelude::*};

/// # Input Component
///
/// A reactive Leptos wrapper for daisyUI's input component that provides a comprehensive
/// set of styling options for text input fields and form elements.
///
/// ## Overview
///
/// The Input component renders an `<input>` element with daisyUI styling. It supports
/// various input types, colors, sizes, and styles for building accessible forms.
///
/// ## CSS Classes Used
///
/// ### Add to `input.css`
/// ```css
/// @source inline("input input-neutral input-primary input-secondary input-accent input-info input-success input-warning input-error input-ghost input-xs input-sm input-md input-lg input-xl input-disabled");
/// ```
///
/// ### daisyUI Classes
/// - `input` - Base input class
/// - Color classes: `input-{color}` (neutral, primary, secondary, accent, info, success, warning, error)
/// - Style classes: `input-ghost`
/// - Size classes: `input-xs`, `input-sm`, `input-md`, `input-lg`, `input-xl`
/// - State classes: `input-disabled`
///
/// ## Usage Example
///
/// ```rust
/// use leptos::*;
/// use leptos_daisyui::input::*;
///
/// #[component]
/// fn InputDemo() -> impl IntoView {
///     let (username, set_username) = signal(String::new());
///     let (email, set_email) = signal(String::new());
///     let (password, set_password) = signal(String::new());
///
///     view! {
///         <div class="form-control">
///             <label class="label">
///                 <span class="label-text">"Username"</span>
///             </label>
///             <Input
///                 color=InputColor::Primary
///                 size=InputSize::Lg
///                 placeholder="Enter username"
///                 value=username
///             />
///         </div>
///         
///         <div class="form-control">
///             <label class="label">
///                 <span class="label-text">"Email"</span>
///             </label>
///             <Input
///                 input_type="email"
///                 color=InputColor::Secondary
///                 placeholder="user@example.com"
///                 value=email
///             />
///         </div>
///         
///         <div class="form-control">
///             <label class="label">
///                 <span class="label-text">"Password"</span>
///             </label>
///             <Input
///                 input_type="password"
///                 style=InputStyle::Ghost
///                 placeholder="Enter password"
///                 value=password
///             />
///         </div>
///         
///         <Input
///             size=InputSize::Sm
///             color=InputColor::Error
///             placeholder="Error state"
///             class="mt-4"
///         />
///     }
/// }
/// ```
///
/// ## Accessibility
///
/// The Input component supports standard HTML input accessibility features:
/// - Use with `<label>` elements or `aria-label` attributes
/// - Supports all standard input types via `input_type` prop
/// - Compatible with form validation and error states
/// - Proper semantic markup for screen readers
///
/// ## Node References
/// - `node_ref` - References the `<input>` element ([HTMLInputElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement))
#[component]
pub fn Input(
    /// Input style variant
    ///
    /// Controls the visual style of the input using InputStyle enum.
    /// Defaults to InputStyle::Default.
    #[prop(optional, into)]
    style: Signal<InputStyle>,

    /// Input color variant
    ///
    /// Controls the color scheme of the input using InputColor enum.
    /// Defaults to InputColor::Default.
    #[prop(optional, into)]
    color: Signal<InputColor>,

    /// Input size variant
    ///
    /// Controls the dimensions of the input using InputSize enum.
    /// Defaults to InputSize::Md (medium).
    #[prop(optional, into)]
    size: Signal<InputSize>,

    /// HTML input type attribute
    ///
    /// Specifies the type of input control. Common values include:
    /// - `"text"` (default) - Plain text input
    /// - `"email"` - Email address input with validation
    /// - `"password"` - Password input with masked characters
    /// - `"number"` - Numeric input with spinner controls
    /// - `"tel"` - Telephone number input
    /// - `"url"` - URL input with validation
    /// - `"search"` - Search input with optional search icon
    #[prop(optional)]
    input_type: Option<&'static str>,

    /// Placeholder text displayed when input is empty
    ///
    /// Provides a hint to the user about what should be entered.
    /// Should be descriptive but concise.
    #[prop(optional, into)]
    placeholder: &'static str,

    /// Current value of the input
    ///
    /// Reactive signal containing the input's current text value.
    /// Use signal() to create a reactive value that updates on input.
    #[prop(optional, into)]
    value: Signal<String>,

    /// Additional CSS classes
    ///
    /// Custom CSS classes to apply alongside daisyUI classes.
    /// Useful for custom styling, spacing, or layout adjustments.
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the input element
    ///
    /// Provides direct access to the underlying HTMLInputElement for
    /// advanced DOM manipulation, focus control, or integration with
    /// other libraries.
    #[prop(optional)]
    node_ref: NodeRef<HtmlInput>,
) -> impl IntoView {
    view! {
        <input
            node_ref=node_ref
            type=input_type.unwrap_or("text")
            class=move || {
                merge_classes!(
                    "input",
                style.get().as_str(),
                color.get().as_str(),
                size.get().as_str(),
                class
                )
            }
            placeholder=placeholder
            value=value
        />
    }
}
