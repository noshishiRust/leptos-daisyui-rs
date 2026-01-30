use leptos::{prelude::*, tachys::html::class::class as class_fn};

/// # Theme Controller Wrapper Component
///
/// This component itself does not have a container, only a validator class for its child components.
/// As such, it is intended to be used with form elements such as `Input` and `Button`.
///
/// Works seamlessly with both standalone daisyUI theme switching and the new `ThemeProvider`
/// component for advanced theming features.
///
/// ## Example
/// ```rust,no_run
/// use leptos::prelude::*;
/// use leptos_daisyui_rs::components::ThemeController;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <ThemeController theme_name="dark">
///             <input type="radio" name="theme" class="radio" />
///         </ThemeController>
///     }
/// }
/// ```
///
/// ### Add to `input.css`
/// ```css
/// @source inline("theme-controller");
/// ```
#[component]
pub fn ThemeController(
    /// Valid daisyUI theme name to activate when checked
    #[prop(into)]
    theme_name: String,

    /// Form element children (such as input (checkbox, toggle), button etc...)
    children: Children,
) -> impl IntoView {
    children()
        .add_any_attr(class_fn(("theme-controller", true)))
        .attr("value", theme_name)
}
