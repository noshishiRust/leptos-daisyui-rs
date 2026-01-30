//! Theme provider component and context
//!
//! Provides a reactive context for theme configuration that can be accessed
//! throughout the component tree.

use leptos::prelude::*;
use crate::theme::css_injection::inject_css_variables;
use crate::theme::storage::{load_theme_config_with_key, save_theme_config_with_key};
use crate::theme::types::ThemeConfiguration;

/// Theme context that holds the current theme configuration
///
/// This context is provided by the `ThemeProvider` component and can be accessed
/// by child components using `use_theme_context()`.
#[derive(Clone, Copy)]
pub struct ThemeContext {
    /// Reactive signal holding the current theme configuration
    pub config: RwSignal<ThemeConfiguration>,
}

impl ThemeContext {
    /// Create a new theme context with a configuration
    pub fn new(config: ThemeConfiguration) -> Self {
        Self {
            config: RwSignal::new(config),
        }
    }

    /// Apply a new theme configuration
    ///
    /// This also injects the theme CSS variables into the document.
    pub fn apply_theme(&self, theme: ThemeConfiguration) {
        self.config.set(theme.clone());
        if let Err(e) = inject_css_variables(&theme) {
            leptos::logging::error!("Failed to inject CSS variables: {}", e);
        }
    }

    /// Reset to default theme
    pub fn reset_theme(&self) {
        self.config.set(ThemeConfiguration::new("light"));
    }

    /// Export current theme as JSON string
    pub fn export_theme(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.config.get())
    }

    /// Import theme from JSON string
    pub fn import_theme(&self, json: &str) -> Result<(), serde_json::Error> {
        let theme: ThemeConfiguration = serde_json::from_str(json)?;
        self.apply_theme(theme);
        Ok(())
    }

    /// Get the current base theme name
    pub fn base_theme(&self) -> String {
        self.config.get().base_theme
    }

    /// Update just the base theme
    ///
    /// This also updates the CSS variables in the document.
    pub fn set_base_theme(&self, theme_name: impl Into<String>) {
        self.config.update(|config| {
            config.base_theme = theme_name.into();
        });
        if let Err(e) = inject_css_variables(&self.config.get()) {
            leptos::logging::error!("Failed to inject CSS variables: {}", e);
        }
    }
}

/// Theme provider component
///
/// Wraps your application to provide theme configuration to all child components.
/// The theme can be accessed using `use_theme_context()`.
///
/// ## Example
/// ```rust,no_run
/// use leptos::prelude::*;
/// use leptos_daisyui_rs::theme::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     let initial_theme = ThemeConfiguration::new("dark");
///
///     view! {
///         <ThemeProvider initial_theme=initial_theme>
///             <div class="app">
///                 "Your app content"
///             </div>
///         </ThemeProvider>
///     }
/// }
/// ```
#[component]
pub fn ThemeProvider(
    /// Initial theme configuration
    #[prop(optional)]
    initial_theme: Option<ThemeConfiguration>,

    /// Load theme from localStorage on mount
    #[prop(optional, into)]
    load_from_storage: bool,

    /// LocalStorage key for theme persistence
    #[prop(optional, into)]
    storage_key: String,

    /// Child components
    children: Children,
) -> impl IntoView {
    // Determine initial theme
    let theme = if load_from_storage {
        load_theme_config_with_key(&storage_key)
            .ok()
            .or(initial_theme)
            .unwrap_or_else(|| ThemeConfiguration::new("light"))
    } else {
        initial_theme.unwrap_or_else(|| ThemeConfiguration::new("light"))
    };

    // Create context
    let context = ThemeContext::new(theme);

    // Provide context to children
    provide_context(context);

    // Inject initial CSS variables
    if let Err(e) = inject_css_variables(&context.config.get()) {
        leptos::logging::error!("Failed to inject initial CSS variables: {}", e);
    }

    // Watch for theme changes and inject CSS + save to localStorage
    Effect::new(move |_| {
        let config = context.config.get();

        // Inject CSS variables
        if let Err(e) = inject_css_variables(&config) {
            leptos::logging::error!("Failed to inject CSS variables: {}", e);
        }

        // Save to localStorage if enabled
        if load_from_storage && let Err(e) = save_theme_config_with_key(&config, &storage_key) {
            leptos::logging::error!("Failed to save theme to localStorage: {}", e);
        }
    });

    view! {
        {children()}
    }
}

/// Hook to access the theme context
///
/// Returns the `ThemeContext` provided by the nearest `ThemeProvider` ancestor.
///
/// ## Example
/// ```rust,no_run
/// use leptos::prelude::*;
/// use leptos_daisyui_rs::theme::*;
///
/// #[component]
/// fn ThemeButton() -> impl IntoView {
///     let theme = use_theme_context();
///
///     view! {
///         <button on:click=move |_| {
///             theme.set_base_theme("dark");
///         }>
///             "Switch to Dark"
///         </button>
///     }
/// }
/// ```
pub fn use_theme_context() -> ThemeContext {
    use_context::<ThemeContext>()
        .expect("ThemeContext not found. Make sure ThemeProvider wraps your app.")
}

/// Try to access the theme context without panicking
///
/// Returns `None` if no ThemeProvider is found in the component tree.
pub fn try_use_theme_context() -> Option<ThemeContext> {
    use_context::<ThemeContext>()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_context_new() {
        let config = ThemeConfiguration::new("dark");
        let context = ThemeContext::new(config.clone());
        assert_eq!(context.config.get().base_theme, "dark");
    }

    #[test]
    fn test_theme_context_apply() {
        let context = ThemeContext::new(ThemeConfiguration::new("light"));
        assert_eq!(context.base_theme(), "light");

        context.apply_theme(ThemeConfiguration::new("dark"));
        assert_eq!(context.base_theme(), "dark");
    }

    #[test]
    fn test_theme_context_reset() {
        let context = ThemeContext::new(ThemeConfiguration::new("cyberpunk"));
        context.reset_theme();
        assert_eq!(context.base_theme(), "light");
    }

    #[test]
    fn test_theme_context_export_import() {
        let context = ThemeContext::new(ThemeConfiguration::new("cupcake"));

        let exported = context.export_theme().unwrap();
        assert!(exported.contains("cupcake"));

        context.set_base_theme("dark");
        assert_eq!(context.base_theme(), "dark");

        context.import_theme(&exported).unwrap();
        assert_eq!(context.base_theme(), "cupcake");
    }

    #[test]
    fn test_theme_context_set_base_theme() {
        let context = ThemeContext::new(ThemeConfiguration::new("light"));
        context.set_base_theme("forest");
        assert_eq!(context.base_theme(), "forest");
    }
}
