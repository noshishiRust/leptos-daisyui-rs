//! # ThemeProvider + use_theme
//!
//! Global theme state management with `data-theme` attribute and localStorage persistence.

use leptos::prelude::*;

/// Reactive theme state provided by [`ThemeProvider`].
///
/// Access the current theme name via `ctx.theme` (a `RwSignal<String>`).
/// Set a new theme with `ctx.theme.set("dark".to_string())`.
#[derive(Clone, Copy)]
pub struct ThemeContext {
    /// Current theme name as a reactive signal.
    pub theme: RwSignal<String>,
}

/// Provider component that manages daisyUI theme switching.
///
/// Place this at the root of your app. It:
/// - Sets the `data-theme` attribute on the `<html>` element
/// - Persists the chosen theme to localStorage
/// - Restores the saved theme on mount
///
/// ## Example
///
/// ```rust,ignore
/// view! {
///     <ThemeProvider default_theme="light">
///         <App />
///     </ThemeProvider>
/// }
/// ```
#[component]
pub fn ThemeProvider(
    /// Initial theme name. Must be a valid daisyUI theme (e.g. "light", "dark", "cupcake").
    /// Defaults to `"light"` if not specified.
    #[prop(optional, into)]
    default_theme: &'static str,

    /// Key used for localStorage persistence. Defaults to `"daisyui-theme"`.
    #[prop(optional, into)]
    storage_key: &'static str,

    children: Children,
) -> impl IntoView {
    let resolved_default = if cfg!(target_arch = "wasm32") {
        // Try to restore from localStorage, fall back to default
        window()
            .local_storage()
            .ok()
            .flatten()
            .and_then(|storage| storage.get_item(storage_key).ok().flatten())
            .unwrap_or_else(|| default_theme.to_string())
    } else {
        default_theme.to_string()
    };

    let theme = RwSignal::new(resolved_default);
    let storage_key = storage_key.to_string();

    // Sync theme to data-theme attribute and localStorage
    Effect::new(move || {
        let theme_name = theme.get();
        if cfg!(target_arch = "wasm32") {
            let doc = document();
            if let Some(html_el) = doc.document_element() {
                html_el
                    .set_attribute("data-theme", &theme_name)
                    .unwrap_or_default();
            }
            if let Ok(Some(storage)) = window().local_storage() {
                let _ = storage.set_item(&storage_key, &theme_name);
            }
        }
    });

    provide_context(ThemeContext { theme });

    children()
}

/// Hook to access the current theme from a [`ThemeProvider`] ancestor.
///
/// Returns [`ThemeContext`] with a reactive `theme` signal.
/// Panics if no `ThemeProvider` is found in the ancestor scope.
///
/// ## Example
///
/// ```rust,ignore
/// let ctx = use_theme();
/// ctx.theme.set("dark".to_string());
/// ```
pub fn use_theme() -> ThemeContext {
    use_context::<ThemeContext>().expect("use_theme requires a ThemeProvider ancestor")
}
