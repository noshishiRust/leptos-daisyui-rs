//! Style definitions for the Loading component.
//!
//! This module provides the style enums and utilities for customizing
//! the appearance of loading indicator components.

/// Color variations for loading indicators.
///
/// ## Variants
///
/// - `Default`: Uses the default theme color
/// - `Neutral`: Neutral gray color scheme
/// - `Primary`: Primary theme color
/// - `Secondary`: Secondary theme color
/// - `Accent`: Accent theme color
/// - `Info`: Information/info theme color
/// - `Success`: Success/green theme color
/// - `Warning`: Warning/yellow theme color
/// - `Error`: Error/red theme color
///
/// ## CSS Classes
///
/// Each variant maps to specific daisyUI CSS classes:
///
/// | Variant | CSS Class | Description |
/// |---------|-----------|-------------|
/// | `Default` | _(none)_ | Default theme color |
/// | `Neutral` | `loading-neutral` | Neutral gray color |
/// | `Primary` | `loading-primary` | Primary theme color |
/// | `Secondary` | `loading-secondary` | Secondary theme color |
/// | `Accent` | `loading-accent` | Accent theme color |
/// | `Info` | `loading-info` | Info theme color |
/// | `Success` | `loading-success` | Success theme color |
/// | `Warning` | `loading-warning` | Warning theme color |
/// | `Error` | `loading-error` | Error theme color |
#[derive(Clone, Debug, Default)]
pub enum LoadingColor {
    /// Default theme color
    #[default]
    Default,
    /// Neutral gray color scheme
    Neutral,
    /// Primary theme color
    Primary,
    /// Secondary theme color
    Secondary,
    /// Accent theme color
    Accent,
    /// Information/info theme color (typically blue)
    Info,
    /// Success theme color (typically green)
    Success,
    /// Warning theme color (typically yellow/orange)
    Warning,
    /// Error theme color (typically red)
    Error,
}

impl LoadingColor {
    /// Returns the CSS class string for the color variant.
    ///
    /// The default variant returns an empty string for optimal performance,
    /// while other variants return their corresponding daisyUI class names.
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            LoadingColor::Default => "",
            LoadingColor::Neutral => "loading-neutral",
            LoadingColor::Primary => "loading-primary",
            LoadingColor::Secondary => "loading-secondary",
            LoadingColor::Accent => "loading-accent",
            LoadingColor::Info => "loading-info",
            LoadingColor::Success => "loading-success",
            LoadingColor::Warning => "loading-warning",
            LoadingColor::Error => "loading-error",
        }
    }
}

/// Animation type variations for loading indicators.
///
/// ## Variants
///
/// - `Spinner`: Classic spinning circle animation
/// - `Dots`: Three bouncing dots animation
/// - `Ring`: Rotating ring with gap animation
/// - `Ball`: Bouncing ball animation
/// - `Bars`: Animated vertical bars
/// - `Infinity`: Infinity symbol animation
///
/// ## CSS Classes
///
/// Each variant maps to specific daisyUI CSS classes:
///
/// | Variant | CSS Class | Description |
/// |---------|-----------|-------------|
/// | `Spinner` | `loading-spinner` | Classic spinning circle |
/// | `Dots` | `loading-dots` | Three bouncing dots |
/// | `Ring` | `loading-ring` | Rotating ring with gap |
/// | `Ball` | `loading-ball` | Bouncing ball animation |
/// | `Bars` | `loading-bars` | Animated vertical bars |
/// | `Infinity` | `loading-infinity` | Infinity symbol animation |
///
/// ## Examples
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::{Loading, LoadingType};
///
/// // Different animation types
/// view! {
///     <Loading loading_type=LoadingType::Dots />
///     <Loading loading_type=LoadingType::Ring />
///     <Loading loading_type=LoadingType::Infinity />
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub enum LoadingType {
    /// Classic spinning circle animation (default)
    #[default]
    Spinner,
    /// Three bouncing dots animation
    Dots,
    /// Rotating ring with gap animation
    Ring,
    /// Bouncing ball animation
    Ball,
    /// Animated vertical bars
    Bars,
    /// Infinity symbol animation
    Infinity,
}

impl LoadingType {
    /// Returns the CSS class string for the animation type.
    ///
    /// All variants return their corresponding daisyUI class names.
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            LoadingType::Spinner => "loading-spinner",
            LoadingType::Dots => "loading-dots",
            LoadingType::Ring => "loading-ring",
            LoadingType::Ball => "loading-ball",
            LoadingType::Bars => "loading-bars",
            LoadingType::Infinity => "loading-infinity",
        }
    }
}

/// Size variations for loading indicators.
///
/// ## Variants
///
/// - `Default`: Standard size (medium)
/// - `Xs`: Extra small size
/// - `Sm`: Small size
/// - `Md`: Medium size (explicit)
/// - `Lg`: Large size
///
/// ## CSS Classes
///
/// Each variant maps to specific daisyUI CSS classes:
///
/// | Variant | CSS Class | Description |
/// |---------|-----------|-------------|
/// | `Default` | _(none)_ | Standard/medium size |
/// | `Xs` | `loading-xs` | Extra small size |
/// | `Sm` | `loading-sm` | Small size |
/// | `Md` | `loading-md` | Medium size (explicit) |
/// | `Lg` | `loading-lg` | Large size |
///
/// ## Examples
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::{Loading, LoadingSize};
///
/// // Different sizes
/// view! {
///     <Loading size=LoadingSize::Xs />
///     <Loading size=LoadingSize::Sm />
///     <Loading size=LoadingSize::Lg />
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub enum LoadingSize {
    /// Standard/medium size
    #[default]
    Default,
    /// Extra small size
    Xs,
    /// Small size
    Sm,
    /// Medium size (explicit)
    Md,
    /// Large size
    Lg,
}

impl LoadingSize {
    /// Returns the CSS class string for the size variant.
    ///
    /// The default variant returns an empty string for optimal performance,
    /// while other variants return their corresponding daisyUI class names.
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            LoadingSize::Default => "",
            LoadingSize::Xs => "loading-xs",
            LoadingSize::Sm => "loading-sm",
            LoadingSize::Md => "loading-md",
            LoadingSize::Lg => "loading-lg",
        }
    }
}
