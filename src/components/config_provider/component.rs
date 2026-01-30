//! Global configuration provider component
//!
//! Provides app-wide configuration including locale, text direction, and accessibility
//! preferences to all child components.

use super::types::{AppConfig, TextDirection};
use leptos::{html, prelude::*};

/// Global configuration context
///
/// This context is provided by the `ConfigProvider` component and can be accessed
/// by child components using `use_config_context()`.
#[derive(Clone, Copy)]
pub struct ConfigContext {
    /// Reactive signal holding the current app configuration
    pub config: RwSignal<AppConfig>,
}

impl ConfigContext {
    /// Create a new configuration context
    pub fn new(config: AppConfig) -> Self {
        Self {
            config: RwSignal::new(config),
        }
    }

    /// Get the current locale
    pub fn locale(&self) -> String {
        self.config.get().locale
    }

    /// Set the locale
    pub fn set_locale(&self, locale: impl Into<String>) {
        let locale_str = locale.into();

        // Auto-detect direction based on locale
        let direction = if locale_str.starts_with("ar") || locale_str.starts_with("he") {
            TextDirection::Rtl
        } else {
            TextDirection::Ltr
        };

        self.config.update(|config| {
            config.locale = locale_str;
            config.direction = direction;
        });
    }

    /// Get the current text direction
    pub fn direction(&self) -> TextDirection {
        self.config.get().direction
    }

    /// Set the text direction
    pub fn set_direction(&self, direction: TextDirection) {
        self.config.update(|config| {
            config.direction = direction;
        });
    }

    /// Check if reduced motion is enabled
    pub fn reduce_motion(&self) -> bool {
        self.config.get().reduce_motion
    }

    /// Set reduced motion preference
    pub fn set_reduce_motion(&self, enabled: bool) {
        self.config.update(|config| {
            config.reduce_motion = enabled;
        });
    }

    /// Check if high contrast mode is enabled
    pub fn high_contrast(&self) -> bool {
        self.config.get().high_contrast
    }

    /// Set high contrast mode
    pub fn set_high_contrast(&self, enabled: bool) {
        self.config.update(|config| {
            config.high_contrast = enabled;
        });
    }

    /// Update the entire configuration
    pub fn update_config(&self, config: AppConfig) {
        self.config.set(config);
    }
}

/// Global configuration provider component
///
/// Wraps your application to provide global configuration settings to all child components.
/// The configuration can be accessed using `use_config_context()`.
///
/// This component integrates seamlessly with the existing `ThemeProvider` for theming.
/// You can nest them in either order depending on your needs.
///
/// ## Features
/// - Locale management for internationalization
/// - Text direction (LTR/RTL) support
/// - Accessibility preferences (reduced motion, high contrast)
/// - Reactive updates throughout the component tree
///
/// ## Example
/// ```rust,ignore
/// use leptos::prelude::*;
/// use leptos_daisyui_rs::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     let config = AppConfig::new("en-US")
///         .with_reduce_motion(false)
///         .with_high_contrast(false);
///
///     view! {
///         <ConfigProvider initial_config=config>
///             <div class="app">
///                 "Your app content"
///             </div>
///         </ConfigProvider>
///     }
/// }
/// ```
///
/// ## With ThemeProvider
/// ```rust,ignore
/// use leptos::prelude::*;
/// use leptos_daisyui_rs::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <ConfigProvider>
///             <ThemeProvider load_from_storage=true>
///                 <div class="app">
///                     "Your app content"
///                 </div>
///             </ThemeProvider>
///         </ConfigProvider>
///     }
/// }
/// ```
///
/// ### Add to `input.css`
/// ```css
/// @source inline("[dir=\"ltr\"] [dir=\"rtl\"]");
/// @source inline("reduce-motion:motion-reduce");
/// @source inline("contrast-more:high-contrast");
/// ```
#[component]
pub fn ConfigProvider(
    /// Initial configuration
    #[prop(optional)]
    initial_config: Option<AppConfig>,

    /// Child components
    children: Children,

    /// Reference to the root element
    #[prop(optional)]
    node_ref: NodeRef<html::Div>,
) -> impl IntoView {
    // Create context with initial config or default
    let config = initial_config.unwrap_or_default();
    let context = ConfigContext::new(config);

    // Provide context to children
    provide_context(context);

    // Get reactive direction for the root element
    let dir = move || context.direction().as_str();

    // Apply accessibility classes based on config
    let container_class = move || {
        let config = context.config.get();
        let mut classes = Vec::new();

        if config.reduce_motion {
            classes.push("motion-reduce");
        }

        if config.high_contrast {
            classes.push("contrast-more");
        }

        classes.join(" ")
    };

    view! {
        <div
            node_ref=node_ref
            dir=dir
            class=container_class
        >
            {children()}
        </div>
    }
}

/// Hook to access the configuration context
///
/// Returns the `ConfigContext` provided by the nearest `ConfigProvider` ancestor.
///
/// ## Example
/// ```rust,ignore
/// use leptos::prelude::*;
/// use leptos_daisyui_rs::*;
///
/// #[component]
/// fn LocaleSwitcher() -> impl IntoView {
///     let config = use_config_context();
///
///     view! {
///         <button on:click=move |_| {
///             config.set_locale("ar-SA");
///         }>
///             "Switch to Arabic"
///         </button>
///     }
/// }
/// ```
pub fn use_config_context() -> ConfigContext {
    use_context::<ConfigContext>()
        .expect("ConfigContext not found. Make sure ConfigProvider wraps your app.")
}

/// Try to access the configuration context without panicking
///
/// Returns `None` if no ConfigProvider is found in the component tree.
pub fn try_use_config_context() -> Option<ConfigContext> {
    use_context::<ConfigContext>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_context_new() {
        let config = AppConfig::new("en-US");
        let context = ConfigContext::new(config.clone());
        assert_eq!(context.locale(), "en-US");
        assert_eq!(context.direction(), TextDirection::Ltr);
    }

    #[test]
    fn test_config_context_set_locale() {
        let context = ConfigContext::new(AppConfig::default());
        assert_eq!(context.locale(), "en-US");

        context.set_locale("ar-SA");
        assert_eq!(context.locale(), "ar-SA");
        assert_eq!(context.direction(), TextDirection::Rtl);
    }

    #[test]
    fn test_config_context_direction() {
        let context = ConfigContext::new(AppConfig::default());
        assert_eq!(context.direction(), TextDirection::Ltr);

        context.set_direction(TextDirection::Rtl);
        assert_eq!(context.direction(), TextDirection::Rtl);
    }

    #[test]
    fn test_config_context_reduce_motion() {
        let context = ConfigContext::new(AppConfig::default());
        assert!(!context.reduce_motion());

        context.set_reduce_motion(true);
        assert!(context.reduce_motion());
    }

    #[test]
    fn test_config_context_high_contrast() {
        let context = ConfigContext::new(AppConfig::default());
        assert!(!context.high_contrast());

        context.set_high_contrast(true);
        assert!(context.high_contrast());
    }

    #[test]
    fn test_config_context_update_config() {
        let context = ConfigContext::new(AppConfig::default());

        let new_config = AppConfig::new("fr-FR")
            .with_reduce_motion(true)
            .with_high_contrast(true);

        context.update_config(new_config);

        assert_eq!(context.locale(), "fr-FR");
        assert!(context.reduce_motion());
        assert!(context.high_contrast());
    }
}
