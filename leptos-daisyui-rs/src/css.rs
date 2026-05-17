//! CSS auto-setup utilities for leptos-daisyui-rs
//!
//! Provides Tailwind CSS v4 `@source inline()` directives for all
//! daisyUI component classes. These directives must be placed in a
//! CSS file that Tailwind CSS processes at build time.
//!
//! ## Important
//!
//! These directives are **build-time** directives for Tailwind CSS v4.
//! They must be placed in a CSS file (e.g., `input.css`) that the
//! Tailwind CSS CLI processes before your application runs.
//! Do NOT inject these into `<style>` tags at runtime — the browser
//! cannot process `@source inline()` directives.

/// The complete CSS file content including the Tailwind header and all
/// `@source inline()` directives for every daisyUI component.
///
/// This is the full content of `stytles/daisyui-components.css`,
/// which includes the `@import "tailwindcss"` and `@plugin "daisyui"`
/// header followed by per-component directives.
pub const FULL_CSS: &str = include_str!("../stytles/daisyui-components.css");

/// Marker comment that precedes the @source directives in daisyui-components.css.
const SOURCE_DIRECTIVES_MARKER: &str = "/* === leptos-daisyui-rs @source directives === */";

/// Returns only the per-component `@source inline()` directives,
/// without the `@import "tailwindcss"` and `@plugin "daisyui"` header.
///
/// Use this when you already have the Tailwind header in your CSS file
/// and only need to add the daisyUI component class directives.
///
/// ## Example usage in `build.rs`
///
/// ```rust,ignore
/// fn main() {
///     let css_path = std::path::PathBuf::from("daisyui-classes.css");
///     std::fs::write(&css_path, leptos_daisyui::css::source_directives_only()).unwrap();
/// }
/// ```
pub fn source_directives_only() -> &'static str {
    const FULL: &str = include_str!("../stytles/daisyui-components.css");
    match FULL.find(SOURCE_DIRECTIVES_MARKER) {
        Some(idx) => &FULL[idx..],
        None => FULL,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_css_contains_header() {
        assert!(FULL_CSS.contains("@import \"tailwindcss\""));
        assert!(FULL_CSS.contains("@plugin \"daisyui\""));
    }

    #[test]
    fn full_css_contains_all_components() {
        assert!(FULL_CSS.contains("@source inline(\"btn"));
        assert!(FULL_CSS.contains("@source inline(\"card"));
        assert!(FULL_CSS.contains("@source inline(\"modal"));
    }

    #[test]
    fn source_directives_excludes_header() {
        let directives = source_directives_only();
        assert!(!directives.contains("@import \"tailwindcss\""));
        assert!(!directives.contains("@plugin \"daisyui\""));
    }

    #[test]
    fn source_directives_contains_components() {
        let directives = source_directives_only();
        assert!(directives.contains(SOURCE_DIRECTIVES_MARKER));
        assert!(directives.contains("/* Accordion */"));
        assert!(directives.contains("@source inline(\"btn"));
        assert!(directives.contains("@source inline(\"card"));
    }
}
