//! Theme utility functions
//!
//! This module provides utility functions for working with themes, including
//! color conversion, validation, and JSON export.

use crate::theme::types::ThemeConfiguration;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

/// Convert hex color to Oklahoma LCH format
///
/// This function converts a hex color string (e.g., "#3b82f6") to Oklahoma LCH
/// color space format (e.g., "oklch(0.608 0.163 251.12)").
///
/// ## Example
/// ```rust
/// use leptos_daisyui_rs::theme::hex_to_oklch;
///
/// let oklch = hex_to_oklch("#3b82f6");
/// assert!(oklch.starts_with("oklch("));
/// ```
///
/// ## Color Space
/// Oklahoma LCH is a perceptual color space that provides better color consistency
/// across different lightness levels compared to traditional RGB/HSL.
///
/// ## Errors
/// Returns a fallback color if the hex string is invalid. Use `validate_hex_color()`
/// to check validity before conversion.
pub fn hex_to_oklch(hex: &str) -> String {
    // Remove # prefix if present
    let hex = hex.trim_start_matches('#');

    // Validate hex length
    if hex.len() != 6 && hex.len() != 3 {
        return "oklch(0.5 0 0)".to_string(); // Fallback gray
    }

    // Expand short hex format (#abc -> #aabbcc)
    let hex = if hex.len() == 3 {
        format!(
            "{}{}{}{}{}{}",
            &hex[0..1],
            &hex[0..1],
            &hex[1..2],
            &hex[1..2],
            &hex[2..3],
            &hex[2..3]
        )
    } else {
        hex.to_string()
    };

    // Parse hex to RGB
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(128);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(128);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(128);

    // Convert RGB to linear RGB (0.0-1.0)
    let r_linear = srgb_to_linear(r as f32 / 255.0);
    let g_linear = srgb_to_linear(g as f32 / 255.0);
    let b_linear = srgb_to_linear(b as f32 / 255.0);

    // Convert linear RGB to OKLab
    // Note: These constants have high precision for accurate color conversion
    #[allow(clippy::excessive_precision)]
    let l = 0.4122214708 * r_linear + 0.5363325363 * g_linear + 0.0514459929 * b_linear;
    #[allow(clippy::excessive_precision)]
    let m = 0.2119034982 * r_linear + 0.6806995451 * g_linear + 0.1073969566 * b_linear;
    #[allow(clippy::excessive_precision)]
    let s = 0.0883024619 * r_linear + 0.2817188376 * g_linear + 0.6299787005 * b_linear;

    let l_ = l.cbrt();
    let m_ = m.cbrt();
    let s_ = s.cbrt();

    #[allow(clippy::excessive_precision)]
    let l_oklab = 0.2104542553 * l_ + 0.7936177850 * m_ - 0.0040720468 * s_;
    #[allow(clippy::excessive_precision)]
    let a_oklab = 1.9779984951 * l_ - 2.4285922050 * m_ + 0.4505937099 * s_;
    #[allow(clippy::excessive_precision)]
    let b_oklab = 0.0259040371 * l_ + 0.7827717662 * m_ - 0.8086757660 * s_;

    // Convert OKLab to OKLCH
    let lightness = l_oklab;
    let chroma = (a_oklab * a_oklab + b_oklab * b_oklab).sqrt();
    let hue = b_oklab.atan2(a_oklab).to_degrees();
    let hue = if hue < 0.0 { hue + 360.0 } else { hue };

    format!(
        "oklch({:.3} {:.3} {:.2})",
        lightness.clamp(0.0, 1.0),
        chroma.clamp(0.0, 0.4),
        hue
    )
}

/// Helper function to convert sRGB to linear RGB
fn srgb_to_linear(c: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

/// Validate hex color format
///
/// Checks if a string is a valid hex color in format `#RRGGBB` or `#RGB`.
///
/// ## Example
/// ```rust
/// use leptos_daisyui_rs::theme::validate_hex_color;
///
/// assert!(validate_hex_color("#3b82f6"));
/// assert!(validate_hex_color("#fff"));
/// assert!(!validate_hex_color("not-a-color"));
/// assert!(!validate_hex_color("#gg0000"));
/// ```
pub fn validate_hex_color(hex: &str) -> bool {
    let hex = hex.trim_start_matches('#');

    // Check length (3 or 6 characters)
    if hex.len() != 3 && hex.len() != 6 {
        return false;
    }

    // Check all characters are valid hex digits
    hex.chars().all(|c| c.is_ascii_hexdigit())
}

/// Download theme configuration as JSON file
///
/// Creates a downloadable JSON file containing the theme configuration.
/// This triggers a browser download with the specified filename.
///
/// ## Example
/// ```rust,no_run
/// use leptos_daisyui_rs::theme::*;
///
/// let theme = ThemeConfiguration::new("dark");
/// if let Err(e) = download_json(&theme, "my-theme.json") {
///     eprintln!("Failed to download: {}", e);
/// }
/// ```
///
/// ## Platform
/// This function only works on WASM targets with browser access.
#[cfg(target_arch = "wasm32")]
pub fn download_json(config: &ThemeConfiguration, filename: &str) -> Result<(), String> {
    use web_sys::{window, Blob, BlobPropertyBag, HtmlAnchorElement, Url};
    use wasm_bindgen::JsValue;

    // Serialize to JSON
    let json = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize theme: {}", e))?;

    // Create blob
    let blob_parts = js_sys::Array::new();
    blob_parts.push(&JsValue::from_str(&json));

    let mut blob_props = BlobPropertyBag::new();
    blob_props.set_type("application/json");

    let blob = Blob::new_with_str_sequence_and_options(&blob_parts, &blob_props)
        .map_err(|e| format!("Failed to create blob: {:?}", e))?;

    // Create object URL
    let url = Url::create_object_url_with_blob(&blob)
        .map_err(|e| format!("Failed to create object URL: {:?}", e))?;

    // Create download link and trigger
    let document = window()
        .ok_or("Window not available")?
        .document()
        .ok_or("Document not available")?;

    let anchor: HtmlAnchorElement = document
        .create_element("a")
        .map_err(|e| format!("Failed to create anchor: {:?}", e))?
        .dyn_into()
        .map_err(|_| "Failed to cast to HtmlAnchorElement")?;

    anchor.set_href(&url);
    anchor.set_download(filename);
    anchor.click();

    // Cleanup
    Url::revoke_object_url(&url)
        .map_err(|e| format!("Failed to revoke object URL: {:?}", e))?;

    Ok(())
}

/// Download theme configuration as JSON (non-WASM fallback)
///
/// This is a no-op on non-WASM targets.
#[cfg(not(target_arch = "wasm32"))]
pub fn download_json(_config: &ThemeConfiguration, _filename: &str) -> Result<(), String> {
    Err("Download not available on non-WASM targets".to_string())
}

/// Get available daisyUI theme names
///
/// Returns a list of all built-in daisyUI theme names that can be used
/// as base themes.
///
/// ## Example
/// ```rust
/// use leptos_daisyui_rs::theme::get_available_themes;
///
/// let themes = get_available_themes();
/// assert!(themes.contains(&"light"));
/// assert!(themes.contains(&"dark"));
/// ```
pub fn get_available_themes() -> Vec<&'static str> {
    vec![
        "light",
        "dark",
        "cupcake",
        "bumblebee",
        "emerald",
        "corporate",
        "synthwave",
        "retro",
        "cyberpunk",
        "valentine",
        "halloween",
        "garden",
        "forest",
        "aqua",
        "lofi",
        "pastel",
        "fantasy",
        "wireframe",
        "black",
        "luxury",
        "dracula",
        "cmyk",
        "autumn",
        "business",
        "acid",
        "lemonade",
        "night",
        "coffee",
        "winter",
        "dim",
        "nord",
        "sunset",
    ]
}

/// Check if a theme name is a valid daisyUI theme
///
/// ## Example
/// ```rust
/// use leptos_daisyui_rs::theme::is_valid_theme_name;
///
/// assert!(is_valid_theme_name("dark"));
/// assert!(is_valid_theme_name("cupcake"));
/// assert!(!is_valid_theme_name("invalid-theme"));
/// ```
pub fn is_valid_theme_name(name: &str) -> bool {
    get_available_themes().contains(&name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_oklch_basic() {
        let oklch = hex_to_oklch("#000000");
        assert!(oklch.starts_with("oklch("));
        assert!(oklch.contains("0.000"));

        let oklch = hex_to_oklch("#ffffff");
        assert!(oklch.starts_with("oklch("));
        assert!(oklch.contains("1.000"));
    }

    #[test]
    fn test_hex_to_oklch_short_format() {
        let oklch = hex_to_oklch("#fff");
        assert!(oklch.starts_with("oklch("));
        assert!(oklch.contains("1.000"));
    }

    #[test]
    fn test_hex_to_oklch_invalid() {
        let oklch = hex_to_oklch("invalid");
        assert_eq!(oklch, "oklch(0.5 0 0)"); // Fallback gray
    }

    #[test]
    fn test_validate_hex_color() {
        assert!(validate_hex_color("#3b82f6"));
        assert!(validate_hex_color("#fff"));
        assert!(validate_hex_color("3b82f6")); // Without #
        assert!(!validate_hex_color("#gg0000"));
        assert!(!validate_hex_color("#12"));
        assert!(!validate_hex_color("not-a-color"));
    }

    #[test]
    fn test_get_available_themes() {
        let themes = get_available_themes();
        assert!(themes.contains(&"light"));
        assert!(themes.contains(&"dark"));
        assert!(themes.contains(&"cupcake"));
        assert!(themes.len() > 20);
    }

    #[test]
    fn test_is_valid_theme_name() {
        assert!(is_valid_theme_name("light"));
        assert!(is_valid_theme_name("dark"));
        assert!(is_valid_theme_name("cyberpunk"));
        assert!(!is_valid_theme_name("invalid"));
        assert!(!is_valid_theme_name(""));
    }

    #[test]
    fn test_download_json_non_wasm() {
        let theme = ThemeConfiguration::new("dark");
        // Non-WASM should return error
        #[cfg(not(target_arch = "wasm32"))]
        assert!(download_json(&theme, "test.json").is_err());
    }
}
