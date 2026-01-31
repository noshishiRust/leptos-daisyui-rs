//! CSS variable injection system
//!
//! This module provides functionality to inject theme overrides as CSS custom properties
//! into the document root, allowing dynamic theme customization at runtime.

use crate::theme::types::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

/// Inject CSS variables from theme configuration into the document
///
/// This function takes a theme configuration and injects all overrides as CSS custom properties
/// on the document's root element (`<html>`). This allows the theme to be dynamically updated
/// at runtime.
///
/// ## Example
/// ```rust,no_run
/// use leptos_daisyui_rs::theme::*;
///
/// let theme = ThemeConfiguration::new("dark");
/// if let Err(e) = inject_css_variables(&theme) {
///     eprintln!("Failed to inject CSS variables: {}", e);
/// }
/// ```
#[cfg(target_arch = "wasm32")]
pub fn inject_css_variables(config: &ThemeConfiguration) -> Result<(), String> {
    use web_sys::window;

    let window = window().ok_or("No window found")?;
    let document = window.document().ok_or("No document found")?;
    let root = document
        .document_element()
        .ok_or("No document element found")?;

    let style = root
        .dyn_ref::<web_sys::HtmlElement>()
        .ok_or("Root element is not an HtmlElement")?
        .style();

    // Inject base theme as data attribute
    root.set_attribute("data-theme", &config.base_theme)
        .map_err(|e| format!("Failed to set data-theme attribute: {:?}", e))?;

    // Inject overrides if present
    if let Some(overrides) = &config.overrides {
        inject_color_overrides(&style, overrides)?;
        inject_typography_overrides(&style, overrides)?;
        inject_border_overrides(&style, overrides)?;
        inject_component_overrides(&style, overrides)?;
    }

    Ok(())
}

/// Inject CSS variables from theme configuration (non-WASM fallback)
///
/// This is a no-op on non-WASM targets.
#[cfg(not(target_arch = "wasm32"))]
pub fn inject_css_variables(_config: &ThemeConfiguration) -> Result<(), String> {
    Ok(())
}

/// Inject color overrides into CSS variables (WASM only)
#[cfg(target_arch = "wasm32")]
fn inject_color_overrides(
    style: &web_sys::CssStyleDeclaration,
    overrides: &ThemeOverrides,
) -> Result<(), String> {
    if let Some(colors) = &overrides.colors {
        set_css_var_if_some(style, "--p", &colors.primary)?;
        set_css_var_if_some(style, "--s", &colors.secondary)?;
        set_css_var_if_some(style, "--a", &colors.accent)?;
        set_css_var_if_some(style, "--n", &colors.neutral)?;
        set_css_var_if_some(style, "--b1", &colors.base_100)?;
        set_css_var_if_some(style, "--b2", &colors.base_200)?;
        set_css_var_if_some(style, "--b3", &colors.base_300)?;
        set_css_var_if_some(style, "--bc", &colors.base_content)?;
        set_css_var_if_some(style, "--in", &colors.info)?;
        set_css_var_if_some(style, "--su", &colors.success)?;
        set_css_var_if_some(style, "--wa", &colors.warning)?;
        set_css_var_if_some(style, "--er", &colors.error)?;
    }
    Ok(())
}

/// Inject typography overrides into CSS variables (WASM only)
#[cfg(target_arch = "wasm32")]
fn inject_typography_overrides(
    style: &web_sys::CssStyleDeclaration,
    overrides: &ThemeOverrides,
) -> Result<(), String> {
    if let Some(typography) = &overrides.typography {
        // Font families
        if let Some(font_family) = &typography.font_family {
            set_css_var_if_some(style, "--font-sans", &font_family.primary)?;
            set_css_var_if_some(style, "--font-heading", &font_family.heading)?;
            set_css_var_if_some(style, "--font-mono", &font_family.monospace)?;
        }

        // Font scale
        if let Some(scale) = &typography.font_scale {
            set_css_var_if_some(style, "--text-xs", &scale.xs.map(|v| format!("{}rem", v)))?;
            set_css_var_if_some(style, "--text-sm", &scale.sm.map(|v| format!("{}rem", v)))?;
            set_css_var_if_some(
                style,
                "--text-base",
                &scale.base.map(|v| format!("{}rem", v)),
            )?;
            set_css_var_if_some(style, "--text-lg", &scale.lg.map(|v| format!("{}rem", v)))?;
            set_css_var_if_some(style, "--text-xl", &scale.xl.map(|v| format!("{}rem", v)))?;
            set_css_var_if_some(style, "--text-2xl", &scale.xl2.map(|v| format!("{}rem", v)))?;
            set_css_var_if_some(style, "--text-3xl", &scale.xl3.map(|v| format!("{}rem", v)))?;
        }

        // Base typography settings
        set_css_var_if_some(
            style,
            "--base-font-size",
            &typography.base_font_size.map(|v| format!("{}rem", v)),
        )?;
        set_css_var_if_some(
            style,
            "--line-height",
            &typography.line_height.map(|v| v.to_string()),
        )?;
        set_css_var_if_some(
            style,
            "--letter-spacing",
            &typography.letter_spacing.map(|v| format!("{}em", v)),
        )?;
    }
    Ok(())
}

/// Inject border overrides into CSS variables (WASM only)
#[cfg(target_arch = "wasm32")]
fn inject_border_overrides(
    style: &web_sys::CssStyleDeclaration,
    overrides: &ThemeOverrides,
) -> Result<(), String> {
    if let Some(borders) = &overrides.borders {
        set_css_var_if_some(
            style,
            "--border-width",
            &borders.border_width.map(|v| format!("{}px", v)),
        )?;
        set_css_var_if_some(
            style,
            "--rounded-sm",
            &borders.radius_sm.map(|v| format!("{}rem", v)),
        )?;
        set_css_var_if_some(
            style,
            "--rounded-md",
            &borders.radius_md.map(|v| format!("{}rem", v)),
        )?;
        set_css_var_if_some(
            style,
            "--rounded-lg",
            &borders.radius_lg.map(|v| format!("{}rem", v)),
        )?;
        set_css_var_if_some(
            style,
            "--spacing-scale",
            &borders.spacing_scale.map(|v| v.to_string()),
        )?;
    }
    Ok(())
}

/// Inject component-specific overrides into CSS variables (WASM only)
#[cfg(target_arch = "wasm32")]
fn inject_component_overrides(
    style: &web_sys::CssStyleDeclaration,
    overrides: &ThemeOverrides,
) -> Result<(), String> {
    if let Some(components) = &overrides.components {
        // Button overrides
        if let Some(button) = &components.button {
            set_css_var_if_some(
                style,
                "--btn-border-radius",
                &button.border_radius.map(|v| format!("{}rem", v)),
            )?;
            set_css_var_if_some(style, "--btn-padding", &button.padding)?;
            set_css_var_if_some(
                style,
                "--btn-font-weight",
                &button.font_weight.map(|v| v.to_string()),
            )?;
            set_css_var_if_some(style, "--btn-text-transform", &button.text_transform)?;
        }

        // Card overrides
        if let Some(card) = &components.card {
            set_css_var_if_some(
                style,
                "--card-border-radius",
                &card.border_radius.map(|v| format!("{}rem", v)),
            )?;
            set_css_var_if_some(style, "--card-shadow", &card.shadow)?;
            set_css_var_if_some(
                style,
                "--card-bg-opacity",
                &card.background_opacity.map(|v| v.to_string()),
            )?;
        }

        // Input overrides
        if let Some(input) = &components.input {
            set_css_var_if_some(
                style,
                "--input-border-radius",
                &input.border_radius.map(|v| format!("{}rem", v)),
            )?;
            set_css_var_if_some(
                style,
                "--input-border-width",
                &input.border_width.map(|v| format!("{}px", v)),
            )?;
            set_css_var_if_some(
                style,
                "--input-focus-ring-width",
                &input.focus_ring_width.map(|v| format!("{}px", v)),
            )?;
            set_css_var_if_some(style, "--input-focus-ring-color", &input.focus_ring_color)?;
        }
    }
    Ok(())
}

/// Helper function to set a CSS variable if the value is Some
#[cfg(target_arch = "wasm32")]
fn set_css_var_if_some(
    style: &web_sys::CssStyleDeclaration,
    property: &str,
    value: &Option<String>,
) -> Result<(), String> {
    if let Some(val) = value {
        style
            .set_property(property, val)
            .map_err(|e| format!("Failed to set CSS property {}: {:?}", property, e))?;
    }
    Ok(())
}

/// Remove all theme-related CSS variables from the document
///
/// This clears all custom CSS properties that were set by `inject_css_variables()`.
#[cfg(target_arch = "wasm32")]
pub fn clear_css_variables() -> Result<(), String> {
    use web_sys::window;

    let window = window().ok_or("No window found")?;
    let document = window.document().ok_or("No document found")?;
    let root = document
        .document_element()
        .ok_or("No document element found")?;

    let style = root
        .dyn_ref::<web_sys::HtmlElement>()
        .ok_or("Root element is not an HtmlElement")?
        .style();

    // List of all CSS variables to clear
    let vars = vec![
        // Colors
        "--p",
        "--s",
        "--a",
        "--n",
        "--b1",
        "--b2",
        "--b3",
        "--bc",
        "--in",
        "--su",
        "--wa",
        "--er",
        // Typography
        "--font-sans",
        "--font-heading",
        "--font-mono",
        "--text-xs",
        "--text-sm",
        "--text-base",
        "--text-lg",
        "--text-xl",
        "--text-2xl",
        "--text-3xl",
        "--base-font-size",
        "--line-height",
        "--letter-spacing",
        // Borders
        "--border-width",
        "--rounded-sm",
        "--rounded-md",
        "--rounded-lg",
        "--spacing-scale",
        // Components
        "--btn-border-radius",
        "--btn-padding",
        "--btn-font-weight",
        "--btn-text-transform",
        "--card-border-radius",
        "--card-shadow",
        "--card-bg-opacity",
        "--input-border-radius",
        "--input-border-width",
        "--input-focus-ring-width",
        "--input-focus-ring-color",
    ];

    for var in vars {
        style
            .remove_property(var)
            .map_err(|e| format!("Failed to remove CSS property {}: {:?}", var, e))?;
    }

    Ok(())
}

/// Remove all theme-related CSS variables (non-WASM fallback)
///
/// This is a no-op on non-WASM targets.
#[cfg(not(target_arch = "wasm32"))]
pub fn clear_css_variables() -> Result<(), String> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inject_css_variables_basic() {
        let theme = ThemeConfiguration::new("dark");
        // Non-WASM will return Ok(())
        assert!(inject_css_variables(&theme).is_ok());
    }

    #[test]
    fn test_inject_css_variables_with_overrides() {
        let overrides = ThemeOverrides {
            colors: Some(ColorOverrides {
                primary: Some("oklch(0.5 0.2 180)".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };

        let theme = ThemeConfiguration::with_overrides("light", overrides);
        assert!(inject_css_variables(&theme).is_ok());
    }

    #[test]
    fn test_clear_css_variables() {
        assert!(clear_css_variables().is_ok());
    }
}
