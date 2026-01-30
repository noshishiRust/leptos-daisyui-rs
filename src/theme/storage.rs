//! Theme configuration storage utilities
//!
//! This module provides functions for persisting theme configuration to localStorage
//! and loading it back. This allows themes to persist across browser sessions.

use crate::theme::types::ThemeConfiguration;

/// Default localStorage key for theme configuration
pub const DEFAULT_THEME_STORAGE_KEY: &str = "leptos-daisyui-theme-config";

/// Save theme configuration to localStorage
///
/// Serializes the theme configuration to JSON and stores it in localStorage
/// using the default key.
///
/// ## Example
/// ```rust,no_run
/// use leptos_daisyui_rs::theme::*;
///
/// let theme = ThemeConfiguration::new("dark");
/// if let Err(e) = save_theme_config(&theme) {
///     eprintln!("Failed to save theme: {}", e);
/// }
/// ```
///
/// ## Errors
/// Returns an error if:
/// - Window or localStorage is not available (non-browser environment)
/// - Serialization fails
/// - localStorage.setItem() fails (quota exceeded, privacy mode, etc.)
#[cfg(target_arch = "wasm32")]
pub fn save_theme_config(config: &ThemeConfiguration) -> Result<(), String> {
    save_theme_config_with_key(config, DEFAULT_THEME_STORAGE_KEY)
}

/// Save theme configuration to localStorage (non-WASM fallback)
///
/// This is a no-op on non-WASM targets.
#[cfg(not(target_arch = "wasm32"))]
pub fn save_theme_config(_config: &ThemeConfiguration) -> Result<(), String> {
    Ok(())
}

/// Save theme configuration to localStorage with a custom key
///
/// Like `save_theme_config()` but allows specifying a custom localStorage key.
/// This is useful if you need to store multiple theme configurations or want
/// to use a different key for namespacing.
///
/// ## Example
/// ```rust,no_run
/// use leptos_daisyui_rs::theme::*;
///
/// let theme = ThemeConfiguration::new("dark");
/// save_theme_config_with_key(&theme, "my-app-theme").unwrap();
/// ```
#[cfg(target_arch = "wasm32")]
pub fn save_theme_config_with_key(
    config: &ThemeConfiguration,
    key: &str,
) -> Result<(), String> {
    use web_sys::window;

    let storage = window()
        .ok_or("Window not available")?
        .local_storage()
        .map_err(|e| format!("Failed to access localStorage: {:?}", e))?
        .ok_or("localStorage not available")?;

    let json = serde_json::to_string(config)
        .map_err(|e| format!("Failed to serialize theme configuration: {}", e))?;

    storage
        .set_item(key, &json)
        .map_err(|e| format!("Failed to save to localStorage: {:?}", e))?;

    Ok(())
}

/// Save theme configuration with custom key (non-WASM fallback)
#[cfg(not(target_arch = "wasm32"))]
pub fn save_theme_config_with_key(
    _config: &ThemeConfiguration,
    _key: &str,
) -> Result<(), String> {
    Ok(())
}

/// Load theme configuration from localStorage
///
/// Retrieves and deserializes the theme configuration from localStorage
/// using the default key.
///
/// ## Example
/// ```rust,no_run
/// use leptos_daisyui_rs::theme::*;
///
/// match load_theme_config() {
///     Ok(theme) => println!("Loaded theme: {}", theme.base_theme),
///     Err(e) => eprintln!("Failed to load theme: {}", e),
/// }
/// ```
///
/// ## Errors
/// Returns an error if:
/// - Window or localStorage is not available
/// - No theme is stored in localStorage
/// - Stored JSON is invalid or cannot be deserialized
///
/// ## Fallback
/// On error, you should typically fall back to a default theme:
/// ```rust,no_run
/// use leptos_daisyui_rs::theme::*;
///
/// let theme = load_theme_config()
///     .unwrap_or_else(|_| ThemeConfiguration::new("light"));
/// ```
#[cfg(target_arch = "wasm32")]
pub fn load_theme_config() -> Result<ThemeConfiguration, String> {
    load_theme_config_with_key(DEFAULT_THEME_STORAGE_KEY)
}

/// Load theme configuration from localStorage (non-WASM fallback)
///
/// Returns an error on non-WASM targets since localStorage is not available.
#[cfg(not(target_arch = "wasm32"))]
pub fn load_theme_config() -> Result<ThemeConfiguration, String> {
    Err("localStorage not available on non-WASM targets".to_string())
}

/// Load theme configuration from localStorage with a custom key
///
/// Like `load_theme_config()` but allows specifying a custom localStorage key.
///
/// ## Example
/// ```rust,no_run
/// use leptos_daisyui_rs::theme::*;
///
/// let theme = load_theme_config_with_key("my-app-theme")
///     .unwrap_or_else(|_| ThemeConfiguration::new("light"));
/// ```
#[cfg(target_arch = "wasm32")]
pub fn load_theme_config_with_key(key: &str) -> Result<ThemeConfiguration, String> {
    use web_sys::window;

    let storage = window()
        .ok_or("Window not available")?
        .local_storage()
        .map_err(|e| format!("Failed to access localStorage: {:?}", e))?
        .ok_or("localStorage not available")?;

    let json = storage
        .get_item(key)
        .map_err(|e| format!("Failed to read from localStorage: {:?}", e))?
        .ok_or_else(|| format!("No theme configuration found in localStorage (key: {})", key))?;

    serde_json::from_str(&json)
        .map_err(|e| format!("Failed to deserialize theme configuration: {}", e))
}

/// Load theme configuration with custom key (non-WASM fallback)
#[cfg(not(target_arch = "wasm32"))]
pub fn load_theme_config_with_key(_key: &str) -> Result<ThemeConfiguration, String> {
    Err("localStorage not available on non-WASM targets".to_string())
}

/// Remove theme configuration from localStorage
///
/// Deletes the stored theme configuration from localStorage using the default key.
///
/// ## Example
/// ```rust,no_run
/// use leptos_daisyui_rs::theme::*;
///
/// if let Err(e) = remove_theme_config() {
///     eprintln!("Failed to remove theme: {}", e);
/// }
/// ```
#[cfg(target_arch = "wasm32")]
pub fn remove_theme_config() -> Result<(), String> {
    remove_theme_config_with_key(DEFAULT_THEME_STORAGE_KEY)
}

/// Remove theme configuration from localStorage (non-WASM fallback)
#[cfg(not(target_arch = "wasm32"))]
pub fn remove_theme_config() -> Result<(), String> {
    Ok(())
}

/// Remove theme configuration from localStorage with a custom key
///
/// ## Example
/// ```rust,no_run
/// use leptos_daisyui_rs::theme::*;
///
/// remove_theme_config_with_key("my-app-theme").unwrap();
/// ```
#[cfg(target_arch = "wasm32")]
pub fn remove_theme_config_with_key(key: &str) -> Result<(), String> {
    use web_sys::window;

    let storage = window()
        .ok_or("Window not available")?
        .local_storage()
        .map_err(|e| format!("Failed to access localStorage: {:?}", e))?
        .ok_or("localStorage not available")?;

    storage
        .remove_item(key)
        .map_err(|e| format!("Failed to remove from localStorage: {:?}", e))?;

    Ok(())
}

/// Remove theme configuration with custom key (non-WASM fallback)
#[cfg(not(target_arch = "wasm32"))]
pub fn remove_theme_config_with_key(_key: &str) -> Result<(), String> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_and_load_theme_config() {
        let theme = ThemeConfiguration::new("dark");

        // Non-WASM targets will return Ok(()) for save
        assert!(save_theme_config(&theme).is_ok());

        // Non-WASM targets will return Err for load
        // WASM targets should successfully load if save worked
        #[cfg(not(target_arch = "wasm32"))]
        assert!(load_theme_config().is_err());
    }

    #[test]
    fn test_save_with_custom_key() {
        let theme = ThemeConfiguration::new("cupcake");
        assert!(save_theme_config_with_key(&theme, "test-key").is_ok());
    }

    #[test]
    fn test_remove_theme_config() {
        assert!(remove_theme_config().is_ok());
        assert!(remove_theme_config_with_key("test-key").is_ok());
    }

    #[test]
    fn test_default_storage_key() {
        assert_eq!(DEFAULT_THEME_STORAGE_KEY, "leptos-daisyui-theme-config");
    }
}
