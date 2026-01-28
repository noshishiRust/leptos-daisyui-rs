use leptos::{html::Div, prelude::*};

/// # Calendar Component
///
/// A reactive Leptos wrapper for daisyUI's calendar component. This component provides
/// styling for popular calendar libraries rather than implementing its own calendar widget.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("cally pika-single react-day-picker");
/// ```
///
/// ## Supported Libraries
///
/// daisyUI Calendar supports three calendar solutions:
///
/// 1. **Cally** - A web component that works universally across frameworks
/// 2. **Pikaday** - A JavaScript date picker library with broad compatibility
/// 3. **React Day Picker** - A React-specific flexible date picker component
///
/// ## Usage
///
/// This component provides the daisyUI styling container. You'll need to:
/// 1. Install and configure your chosen calendar library
/// 2. Place the calendar widget as a child of this component
/// 3. The component will apply appropriate daisyUI styling
///
/// ### Integration Examples
///
/// **Cally Web Component:**
/// ```html
/// <Calendar>
///   <calendar-date></calendar-date>
/// </Calendar>
/// ```
///
/// **Native HTML:**
/// ```html
/// <Calendar>
///   <input type="date" />
/// </Calendar>
/// ```
///
/// For JavaScript library integration (Pikaday, React Day Picker), you'll need to use
/// `leptos_use` or custom JavaScript interop to initialize the library.
///
/// ## Node References
/// - `node_ref` - References the wrapper `<div>` element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn Calendar(
    /// Additional CSS classes for the calendar wrapper
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the wrapper div element
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Calendar widget content (web component, input, or library integration)
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=class>
            {children()}
        </div>
    }
}
