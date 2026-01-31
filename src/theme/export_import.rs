//! Theme import and export functionality
//!
//! This module provides utilities for exporting theme configurations to JSON files
//! and importing them back, enabling theme sharing and backup.

use crate::theme::ThemeConfiguration;
use serde_json;
use wasm_bindgen::JsCast;
use web_sys::{Blob, BlobPropertyBag, HtmlAnchorElement, Url, window};

/// Export a theme configuration as a JSON string
///
/// # Arguments
/// * `config` - The theme configuration to export
/// * `pretty` - Whether to format the JSON with indentation (default: true)
///
/// # Returns
/// Result containing the JSON string or an error message
///
/// # Example
/// ```rust,no_run
/// use leptos_daisyui_rs::theme::{ThemeConfiguration, export_theme_json};
///
/// let config = ThemeConfiguration::new("dark");
/// let json = export_theme_json(&config, true).unwrap();
/// ```
pub fn export_theme_json(config: &ThemeConfiguration, pretty: bool) -> Result<String, String> {
    if pretty {
        serde_json::to_string_pretty(config)
            .map_err(|e| format!("Failed to serialize theme: {}", e))
    } else {
        serde_json::to_string(config).map_err(|e| format!("Failed to serialize theme: {}", e))
    }
}

/// Download a theme configuration as a JSON file
///
/// This function creates a downloadable JSON file in the browser.
///
/// # Arguments
/// * `config` - The theme configuration to export
/// * `filename` - Optional filename (default: "theme-config.json")
///
/// # Returns
/// Result indicating success or error message
///
/// # Example
/// ```rust,no_run
/// use leptos_daisyui_rs::theme::{ThemeConfiguration, download_theme};
///
/// let config = ThemeConfiguration::new("dark");
/// download_theme(&config, Some("my-dark-theme.json")).unwrap();
/// ```
pub fn download_theme(config: &ThemeConfiguration, filename: Option<&str>) -> Result<(), String> {
    let json = export_theme_json(config, true)?;
    let filename = filename.unwrap_or("theme-config.json");

    // Get the window and document
    let window = window().ok_or("No window object available")?;
    let document = window.document().ok_or("No document object available")?;

    // Create a blob from the JSON string
    let array = js_sys::Array::new();
    array.push(&wasm_bindgen::JsValue::from_str(&json));

    let blob_props = BlobPropertyBag::new();
    blob_props.set_type("application/json");

    let blob = Blob::new_with_str_sequence_and_options(&array, &blob_props)
        .map_err(|_| "Failed to create blob")?;

    // Create an object URL for the blob
    let url = Url::create_object_url_with_blob(&blob).map_err(|_| "Failed to create object URL")?;

    // Create a temporary anchor element to trigger download
    let anchor = document
        .create_element("a")
        .map_err(|_| "Failed to create anchor element")?
        .dyn_into::<HtmlAnchorElement>()
        .map_err(|_| "Failed to cast to HtmlAnchorElement")?;

    anchor.set_href(&url);
    anchor.set_download(filename);
    anchor.style().set_property("display", "none").ok();

    // Append to body, click, and remove
    let body = document.body().ok_or("No body element available")?;

    body.append_child(&anchor)
        .map_err(|_| "Failed to append anchor to body")?;

    anchor.click();

    body.remove_child(&anchor)
        .map_err(|_| "Failed to remove anchor from body")?;

    // Revoke the object URL to free memory
    Url::revoke_object_url(&url).map_err(|_| "Failed to revoke object URL")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::{ThemeMetadata, ThemeOverrides};

    #[test]
    fn test_export_theme_json() {
        let config = ThemeConfiguration::new("dark");
        let json = export_theme_json(&config, false).unwrap();

        assert!(json.contains("\"baseTheme\":\"dark\""));
    }

    #[test]
    fn test_export_theme_json_pretty() {
        let config =
            ThemeConfiguration::new("light").with_metadata(ThemeMetadata::new("Test Theme"));

        let json = export_theme_json(&config, true).unwrap();

        // Pretty JSON should have newlines
        assert!(json.contains('\n'));
        assert!(json.contains("\"baseTheme\": \"light\""));
        assert!(json.contains("\"name\": \"Test Theme\""));
    }

    #[test]
    fn test_export_theme_with_overrides() {
        let mut config = ThemeConfiguration::new("cyberpunk");
        config.overrides = Some(ThemeOverrides::default());

        let json = export_theme_json(&config, false).unwrap();

        assert!(json.contains("\"baseTheme\":\"cyberpunk\""));
        assert!(json.contains("\"overrides\""));
    }

    #[test]
    fn test_roundtrip_serialization() {
        let original =
            ThemeConfiguration::new("forest").with_metadata(ThemeMetadata::new("Forest Theme"));

        let json = export_theme_json(&original, false).unwrap();
        let deserialized: ThemeConfiguration = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
    }
}
