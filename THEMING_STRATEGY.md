# Advanced Theming Strategy for leptos-daisyui-rs

## Overview

This document outlines a comprehensive strategy for expanding the current ThemeController component into a full-featured runtime theming system that allows users to:

1. Select a base daisyUI theme
2. Customize specific design tokens (fonts, colors, borders, spacing)
3. Preview changes in real-time
4. Persist customizations across sessions
5. Export/import theme configurations

## Architecture

### 1. Three-Layer Theming System

```
┌─────────────────────────────────────────────┐
│ Layer 3: User Overrides (Runtime)          │
│ - Font family, sizes                        │
│ - Custom colors                             │
│ - Border widths/colors                      │
│ - Component-specific overrides              │
├─────────────────────────────────────────────┤
│ Layer 2: Base Theme (daisyUI preset)       │
│ - light, dark, cupcake, cyberpunk, etc.    │
├─────────────────────────────────────────────┤
│ Layer 1: Default Values (CSS variables)    │
│ - Fallback values                           │
│ - Browser defaults                          │
└─────────────────────────────────────────────┘
```

### 2. Data Structure Design

```rust
/// Complete theme configuration combining base theme + overrides
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThemeConfiguration {
    /// Base daisyUI theme name
    pub base_theme: String,

    /// User customizations that override base theme
    pub overrides: ThemeOverrides,

    /// Metadata
    pub metadata: ThemeMetadata,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ThemeOverrides {
    // Typography
    pub font_family: Option<FontFamily>,
    pub font_scale: Option<FontScale>,

    // Colors
    pub colors: ColorOverrides,

    // Borders
    pub borders: BorderOverrides,

    // Spacing
    pub spacing_scale: Option<f32>, // multiplier: 0.5 to 2.0

    // Component-specific
    pub components: ComponentOverrides,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FontFamily {
    pub sans: Option<String>,
    pub serif: Option<String>,
    pub mono: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FontScale {
    pub base_size: u8,        // 12-20px
    pub scale_ratio: f32,     // 1.125 (major second) to 1.618 (golden ratio)
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ColorOverrides {
    pub primary: Option<String>,
    pub secondary: Option<String>,
    pub accent: Option<String>,
    pub neutral: Option<String>,
    pub base_100: Option<String>,
    pub base_200: Option<String>,
    pub base_300: Option<String>,
    pub info: Option<String>,
    pub success: Option<String>,
    pub warning: Option<String>,
    pub error: Option<String>,
    // Allow custom color tokens
    pub custom: HashMap<String, String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct BorderOverrides {
    pub default_width: Option<String>,  // "1px", "2px", etc.
    pub default_color: Option<String>,  // color value
    pub default_radius: Option<String>, // "0.25rem", "0.5rem", etc.
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ComponentOverrides {
    pub card_border_width: Option<String>,
    pub button_border_radius: Option<String>,
    pub input_border_width: Option<String>,
    // ... expandable per component
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThemeMetadata {
    pub name: String,
    pub created_at: String,
    pub modified_at: String,
    pub version: String,
}
```

### 3. Implementation Components

#### A. Theme Provider (Context)

```rust
/// Global theme context accessible to all components
#[derive(Clone)]
pub struct ThemeContext {
    /// Current theme configuration
    pub config: RwSignal<ThemeConfiguration>,

    /// Apply theme changes
    pub apply_theme: Action<ThemeConfiguration, ()>,

    /// Reset to defaults
    pub reset_theme: Action<(), ()>,

    /// Export theme JSON
    pub export_theme: Action<(), String>,

    /// Import theme from JSON
    pub import_theme: Action<String, Result<(), String>>,
}

#[component]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    // Load theme from localStorage or use default
    let config = create_rw_signal(load_theme_config());

    // Apply theme by injecting CSS variables
    let apply_theme = create_action(|config: &ThemeConfiguration| {
        let config = config.clone();
        async move {
            inject_css_variables(&config);
            set_daisyui_theme(&config.base_theme);
            save_theme_config(&config);
        }
    });

    let reset_theme = create_action(|_: &()| async move {
        let default = ThemeConfiguration::default();
        config.set(default.clone());
        inject_css_variables(&default);
    });

    let export_theme = create_action(|_: &()| {
        let current = config.get();
        async move {
            serde_json::to_string_pretty(&current).unwrap_or_default()
        }
    });

    let import_theme = create_action(|json: &String| {
        let json = json.clone();
        async move {
            match serde_json::from_str::<ThemeConfiguration>(&json) {
                Ok(theme) => {
                    config.set(theme.clone());
                    inject_css_variables(&theme);
                    Ok(())
                }
                Err(e) => Err(e.to_string()),
            }
        }
    });

    let context = ThemeContext {
        config,
        apply_theme,
        reset_theme,
        export_theme,
        import_theme,
    };

    provide_context(context);

    view! {
        <div id="theme-root" data-theme=move || config.get().base_theme.clone()>
            {children()}
        </div>
    }
}
```

#### B. CSS Variable Injection

```rust
/// Injects CSS custom properties into the document
fn inject_css_variables(config: &ThemeConfiguration) {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(root) = document.document_element() {
                let html_elem = root.dyn_ref::<web_sys::HtmlElement>().unwrap();
                let style = html_elem.style();

                // Typography
                if let Some(ref font) = config.overrides.font_family {
                    if let Some(ref sans) = font.sans {
                        let _ = style.set_property("--font-sans", sans);
                    }
                    if let Some(ref mono) = font.mono {
                        let _ = style.set_property("--font-mono", mono);
                    }
                }

                if let Some(ref scale) = config.overrides.font_scale {
                    let _ = style.set_property(
                        "--font-size-base",
                        &format!("{}px", scale.base_size)
                    );

                    // Calculate scale
                    let ratio = scale.scale_ratio;
                    let _ = style.set_property("--font-size-xs", &format!("{}rem", 1.0 / ratio));
                    let _ = style.set_property("--font-size-sm", &format!("{}rem", 1.0 / ratio.sqrt()));
                    let _ = style.set_property("--font-size-lg", &format!("{}rem", ratio));
                    let _ = style.set_property("--font-size-xl", &format!("{}rem", ratio * ratio));
                }

                // Colors
                let colors = &config.overrides.colors;
                if let Some(ref primary) = colors.primary {
                    let _ = style.set_property("--color-primary", primary);
                }
                if let Some(ref secondary) = colors.secondary {
                    let _ = style.set_property("--color-secondary", secondary);
                }
                // ... other colors

                // Borders
                let borders = &config.overrides.borders;
                if let Some(ref width) = borders.default_width {
                    let _ = style.set_property("--border-width", width);
                }
                if let Some(ref color) = borders.default_color {
                    let _ = style.set_property("--border-color", color);
                }
                if let Some(ref radius) = borders.default_radius {
                    let _ = style.set_property("--border-radius", radius);
                }

                // Component overrides
                let components = &config.overrides.components;
                if let Some(ref width) = components.card_border_width {
                    let _ = style.set_property("--card-border-width", width);
                }
                // ... other component overrides
            }
        }
    }
}
```

#### C. Enhanced Theme Controller UI

```rust
#[component]
pub fn ThemeCustomizer() -> impl IntoView {
    let theme_ctx = use_context::<ThemeContext>()
        .expect("ThemeCustomizer must be used within ThemeProvider");

    let config = theme_ctx.config;
    let (active_tab, set_active_tab) = signal("base-theme");

    view! {
        <div class="theme-customizer card bg-base-100 shadow-xl">
            <div class="card-body">
                <h2 class="card-title">"Theme Customizer"</h2>

                // Tab Navigation
                <div class="tabs tabs-boxed">
                    <a
                        class="tab"
                        class:tab-active=move || active_tab.get() == "base-theme"
                        on:click=move |_| set_active_tab.set("base-theme")
                    >
                        "Base Theme"
                    </a>
                    <a
                        class="tab"
                        class:tab-active=move || active_tab.get() == "typography"
                        on:click=move |_| set_active_tab.set("typography")
                    >
                        "Typography"
                    </a>
                    <a
                        class="tab"
                        class:tab-active=move || active_tab.get() == "colors"
                        on:click=move |_| set_active_tab.set("colors")
                    >
                        "Colors"
                    </a>
                    <a
                        class="tab"
                        class:tab-active=move || active_tab.get() == "borders"
                        on:click=move |_| set_active_tab.set("borders")
                    >
                        "Borders & Spacing"
                    </a>
                    <a
                        class="tab"
                        class:tab-active=move || active_tab.get() == "components"
                        on:click=move |_| set_active_tab.set("components")
                    >
                        "Components"
                    </a>
                </div>

                // Tab Content
                <div class="mt-4">
                    {move || match active_tab.get() {
                        "base-theme" => view! { <BaseThemeSelector /> }.into_view(),
                        "typography" => view! { <TypographyCustomizer /> }.into_view(),
                        "colors" => view! { <ColorCustomizer /> }.into_view(),
                        "borders" => view! { <BorderCustomizer /> }.into_view(),
                        "components" => view! { <ComponentCustomizer /> }.into_view(),
                        _ => view! { <div>"Unknown tab"</div> }.into_view(),
                    }}
                </div>

                // Actions
                <div class="card-actions justify-end mt-4">
                    <Button
                        color=ButtonColor::Ghost
                        on:click=move |_| theme_ctx.reset_theme.dispatch(())
                    >
                        "Reset to Defaults"
                    </Button>
                    <Button
                        color=ButtonColor::Secondary
                        on:click=move |_| {
                            let json = theme_ctx.export_theme.dispatch(());
                            // Trigger download
                            download_json(&json, "theme.json");
                        }
                    >
                        "Export Theme"
                    </Button>
                    <Button
                        color=ButtonColor::Primary
                        on:click=move |_| theme_ctx.apply_theme.dispatch(config.get())
                    >
                        "Apply Changes"
                    </Button>
                </div>
            </div>
        </div>
    }
}
```

#### D. Individual Customizer Components

```rust
#[component]
fn BaseThemeSelector() -> impl IntoView {
    let theme_ctx = use_context::<ThemeContext>().unwrap();
    let config = theme_ctx.config;

    let themes = vec![
        "light", "dark", "cupcake", "bumblebee", "emerald",
        "corporate", "synthwave", "retro", "cyberpunk", "valentine",
        "halloween", "garden", "forest", "aqua", "lofi",
        "pastel", "fantasy", "wireframe", "black", "luxury",
        "dracula", "cmyk", "autumn", "business", "acid",
        "lemonade", "night", "coffee", "winter", "dim",
        "nord", "sunset",
    ];

    view! {
        <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
            {themes.into_iter().map(|theme| {
                let theme_name = theme.to_string();
                view! {
                    <button
                        class="btn btn-outline"
                        class:btn-active=move || config.get().base_theme == theme_name
                        on:click=move |_| {
                            config.update(|c| c.base_theme = theme_name.clone());
                        }
                    >
                        {theme}
                    </button>
                }
            }).collect_view()}
        </div>
    }
}

#[component]
fn TypographyCustomizer() -> impl IntoView {
    let theme_ctx = use_context::<ThemeContext>().unwrap();
    let config = theme_ctx.config;

    let font_families = vec![
        ("System Default", ""),
        ("Inter", "Inter, system-ui, sans-serif"),
        ("Roboto", "Roboto, sans-serif"),
        ("Open Sans", "'Open Sans', sans-serif"),
        ("Lato", "Lato, sans-serif"),
        ("Poppins", "Poppins, sans-serif"),
        ("Montserrat", "Montserrat, sans-serif"),
        ("Fira Code", "'Fira Code', monospace"),
        ("JetBrains Mono", "'JetBrains Mono', monospace"),
    ];

    view! {
        <div class="space-y-4">
            // Font Family Selector
            <div class="form-control">
                <label class="label">
                    <span class="label-text">"Font Family"</span>
                </label>
                <select
                    class="select select-bordered"
                    on:change=move |ev| {
                        let value = event_target_value(&ev);
                        config.update(|c| {
                            if c.overrides.font_family.is_none() {
                                c.overrides.font_family = Some(FontFamily::default());
                            }
                            if let Some(ref mut font) = c.overrides.font_family {
                                font.sans = if value.is_empty() { None } else { Some(value) };
                            }
                        });
                    }
                >
                    {font_families.into_iter().map(|(name, value)| {
                        view! {
                            <option value=value>{name}</option>
                        }
                    }).collect_view()}
                </select>
            </div>

            // Base Font Size
            <div class="form-control">
                <label class="label">
                    <span class="label-text">"Base Font Size"</span>
                    <span class="label-text-alt">
                        {move || {
                            config.get()
                                .overrides
                                .font_scale
                                .as_ref()
                                .map(|s| format!("{}px", s.base_size))
                                .unwrap_or_else(|| "16px (default)".to_string())
                        }}
                    </span>
                </label>
                <input
                    type="range"
                    min="12"
                    max="20"
                    value="16"
                    class="range"
                    on:input=move |ev| {
                        let value: u8 = event_target_value(&ev).parse().unwrap_or(16);
                        config.update(|c| {
                            if c.overrides.font_scale.is_none() {
                                c.overrides.font_scale = Some(FontScale {
                                    base_size: 16,
                                    scale_ratio: 1.25,
                                });
                            }
                            if let Some(ref mut scale) = c.overrides.font_scale {
                                scale.base_size = value;
                            }
                        });
                    }
                />
                <div class="flex justify-between text-xs px-2">
                    <span>"12px"</span>
                    <span>"16px"</span>
                    <span>"20px"</span>
                </div>
            </div>

            // Scale Ratio
            <div class="form-control">
                <label class="label">
                    <span class="label-text">"Type Scale Ratio"</span>
                </label>
                <select
                    class="select select-bordered"
                    on:change=move |ev| {
                        let value: f32 = event_target_value(&ev).parse().unwrap_or(1.25);
                        config.update(|c| {
                            if c.overrides.font_scale.is_none() {
                                c.overrides.font_scale = Some(FontScale {
                                    base_size: 16,
                                    scale_ratio: 1.25,
                                });
                            }
                            if let Some(ref mut scale) = c.overrides.font_scale {
                                scale.scale_ratio = value;
                            }
                        });
                    }
                >
                    <option value="1.125">"Minor Second (1.125)"</option>
                    <option value="1.2">"Minor Third (1.2)"</option>
                    <option value="1.25" selected>"Major Third (1.25)"</option>
                    <option value="1.333">"Perfect Fourth (1.333)"</option>
                    <option value="1.5">"Perfect Fifth (1.5)"</option>
                    <option value="1.618">"Golden Ratio (1.618)"</option>
                </select>
            </div>

            // Preview
            <div class="border border-base-300 p-4 rounded">
                <h4 class="text-xs mb-2">"Preview"</h4>
                <div class="space-y-1">
                    <p class="text-xs">"Extra Small Text"</p>
                    <p class="text-sm">"Small Text"</p>
                    <p class="text-base">"Base Text"</p>
                    <p class="text-lg">"Large Text"</p>
                    <p class="text-xl">"Extra Large Text"</p>
                </div>
            </div>
        </div>
    }
}

#[component]
fn ColorCustomizer() -> impl IntoView {
    let theme_ctx = use_context::<ThemeContext>().unwrap();
    let config = theme_ctx.config;

    let color_tokens = vec![
        ("Primary", "primary"),
        ("Secondary", "secondary"),
        ("Accent", "accent"),
        ("Neutral", "neutral"),
        ("Base 100", "base_100"),
        ("Base 200", "base_200"),
        ("Base 300", "base_300"),
        ("Info", "info"),
        ("Success", "success"),
        ("Warning", "warning"),
        ("Error", "error"),
    ];

    view! {
        <div class="space-y-4">
            <p class="text-sm opacity-70">
                "Customize color tokens. Colors use Oklahoma LCH format for perceptual uniformity."
            </p>

            {color_tokens.into_iter().map(|(label, key)| {
                let key_clone = key.to_string();
                view! {
                    <div class="form-control">
                        <label class="label">
                            <span class="label-text">{label}</span>
                        </label>
                        <div class="flex gap-2">
                            <input
                                type="color"
                                class="input input-bordered w-16 p-1"
                                on:change=move |ev| {
                                    let hex = event_target_value(&ev);
                                    let oklch = hex_to_oklch(&hex);
                                    config.update(|c| {
                                        match key_clone.as_str() {
                                            "primary" => c.overrides.colors.primary = Some(oklch.clone()),
                                            "secondary" => c.overrides.colors.secondary = Some(oklch.clone()),
                                            "accent" => c.overrides.colors.accent = Some(oklch.clone()),
                                            _ => {}
                                        }
                                    });
                                }
                            />
                            <input
                                type="text"
                                placeholder="oklch(0.7 0.15 150)"
                                class="input input-bordered flex-1"
                                value=move || {
                                    config.get().overrides.colors.primary.clone().unwrap_or_default()
                                }
                            />
                            <button
                                class="btn btn-ghost btn-sm"
                                on:click=move |_| {
                                    config.update(|c| {
                                        match key_clone.as_str() {
                                            "primary" => c.overrides.colors.primary = None,
                                            _ => {}
                                        }
                                    });
                                }
                            >
                                "Reset"
                            </button>
                        </div>
                    </div>
                }
            }).collect_view()}

            // Color Preview
            <div class="border border-base-300 p-4 rounded">
                <h4 class="text-xs mb-2">"Preview"</h4>
                <div class="flex gap-2">
                    <Button color=ButtonColor::Primary>"Primary"</Button>
                    <Button color=ButtonColor::Secondary>"Secondary"</Button>
                    <Button color=ButtonColor::Accent>"Accent"</Button>
                    <Button color=ButtonColor::Info>"Info"</Button>
                    <Button color=ButtonColor::Success>"Success"</Button>
                    <Button color=ButtonColor::Warning>"Warning"</Button>
                    <Button color=ButtonColor::Error>"Error"</Button>
                </div>
            </div>
        </div>
    }
}

#[component]
fn BorderCustomizer() -> impl IntoView {
    let theme_ctx = use_context::<ThemeContext>().unwrap();
    let config = theme_ctx.config;

    view! {
        <div class="space-y-4">
            // Border Width
            <div class="form-control">
                <label class="label">
                    <span class="label-text">"Default Border Width"</span>
                    <span class="label-text-alt">
                        {move || {
                            config.get()
                                .overrides
                                .borders
                                .default_width
                                .clone()
                                .unwrap_or_else(|| "1px (default)".to_string())
                        }}
                    </span>
                </label>
                <input
                    type="range"
                    min="0"
                    max="8"
                    step="0.5"
                    value="1"
                    class="range"
                    on:input=move |ev| {
                        let value = event_target_value(&ev);
                        config.update(|c| {
                            c.overrides.borders.default_width = Some(format!("{}px", value));
                        });
                    }
                />
                <div class="flex justify-between text-xs px-2">
                    <span>"0px"</span>
                    <span>"4px"</span>
                    <span>"8px"</span>
                </div>
            </div>

            // Border Radius
            <div class="form-control">
                <label class="label">
                    <span class="label-text">"Border Radius"</span>
                </label>
                <select
                    class="select select-bordered"
                    on:change=move |ev| {
                        let value = event_target_value(&ev);
                        config.update(|c| {
                            c.overrides.borders.default_radius = Some(value);
                        });
                    }
                >
                    <option value="0">"None (0)"</option>
                    <option value="0.125rem">"Small (0.125rem)"</option>
                    <option value="0.25rem">"Medium (0.25rem)"</option>
                    <option value="0.5rem" selected>"Large (0.5rem)"</option>
                    <option value="1rem">"XL (1rem)"</option>
                    <option value="9999px">"Full (pill)"</option>
                </select>
            </div>

            // Preview
            <div class="border border-base-300 p-4 rounded">
                <h4 class="text-xs mb-2">"Preview"</h4>
                <div class="grid grid-cols-2 gap-4">
                    <Card style=CardStyle::Border class="bg-base-100">
                        <CardBody>
                            <CardTitle>"Card with Border"</CardTitle>
                            <p>"Preview of border styling"</p>
                        </CardBody>
                    </Card>
                    <div class="space-y-2">
                        <input type="text" class="input input-bordered" placeholder="Input field" />
                        <Button color=ButtonColor::Primary>"Button"</Button>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn ComponentCustomizer() -> impl IntoView {
    let theme_ctx = use_context::<ThemeContext>().unwrap();
    let config = theme_ctx.config;

    view! {
        <div class="space-y-4">
            <p class="text-sm opacity-70">
                "Fine-tune specific component styles. These override the global settings."
            </p>

            // Card Borders
            <div class="form-control">
                <label class="label">
                    <span class="label-text">"Card Border Width"</span>
                </label>
                <select
                    class="select select-bordered"
                    on:change=move |ev| {
                        let value = event_target_value(&ev);
                        config.update(|c| {
                            c.overrides.components.card_border_width =
                                if value.is_empty() { None } else { Some(value) };
                        });
                    }
                >
                    <option value="">"Use Default"</option>
                    <option value="1px">"1px"</option>
                    <option value="2px">"2px"</option>
                    <option value="3px">"3px"</option>
                    <option value="4px">"4px"</option>
                </select>
            </div>

            // Button Border Radius
            <div class="form-control">
                <label class="label">
                    <span class="label-text">"Button Border Radius"</span>
                </label>
                <select
                    class="select select-bordered"
                    on:change=move |ev| {
                        let value = event_target_value(&ev);
                        config.update(|c| {
                            c.overrides.components.button_border_radius =
                                if value.is_empty() { None } else { Some(value) };
                        });
                    }
                >
                    <option value="">"Use Default"</option>
                    <option value="0">"Square"</option>
                    <option value="0.25rem">"Slightly Rounded"</option>
                    <option value="0.5rem">"Rounded"</option>
                    <option value="9999px">"Pill"</option>
                </select>
            </div>

            // Input Border Width
            <div class="form-control">
                <label class="label">
                    <span class="label-text">"Input Border Width"</span>
                </label>
                <select
                    class="select select-bordered"
                    on:change=move |ev| {
                        let value = event_target_value(&ev);
                        config.update(|c| {
                            c.overrides.components.input_border_width =
                                if value.is_empty() { None } else { Some(value) };
                        });
                    }
                >
                    <option value="">"Use Default"</option>
                    <option value="1px">"1px"</option>
                    <option value="2px">"2px"</option>
                    <option value="3px">"3px"</option>
                </select>
            </div>
        </div>
    }
}
```

### 4. Persistence Layer

```rust
/// Save theme configuration to localStorage
fn save_theme_config(config: &ThemeConfiguration) {
    if let Some(storage) = web_sys::window()
        .and_then(|w| w.local_storage().ok())
        .flatten()
    {
        if let Ok(json) = serde_json::to_string(config) {
            let _ = storage.set_item("theme_config", &json);
        }
    }
}

/// Load theme configuration from localStorage
fn load_theme_config() -> ThemeConfiguration {
    if let Some(storage) = web_sys::window()
        .and_then(|w| w.local_storage().ok())
        .flatten()
    {
        if let Ok(Some(json)) = storage.get_item("theme_config") {
            if let Ok(config) = serde_json::from_str(&json) {
                return config;
            }
        }
    }
    ThemeConfiguration::default()
}
```

### 5. Utility Functions

```rust
/// Convert hex color to Oklahoma LCH
fn hex_to_oklch(hex: &str) -> String {
    // Implementation would use color conversion library
    // For now, placeholder:
    format!("oklch(0.7 0.15 150)") // Example output
}

/// Trigger file download
fn download_json(json: &str, filename: &str) {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            let blob_parts = js_sys::Array::new();
            blob_parts.push(&wasm_bindgen::JsValue::from_str(json));

            let mut options = web_sys::BlobPropertyBag::new();
            options.type_("application/json");

            if let Ok(blob) = web_sys::Blob::new_with_str_sequence_and_options(
                &blob_parts,
                &options,
            ) {
                let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();

                if let Ok(Some(anchor)) = document.create_element("a") {
                    let anchor = anchor.dyn_into::<web_sys::HtmlAnchorElement>().unwrap();
                    anchor.set_href(&url);
                    anchor.set_download(filename);
                    anchor.click();
                    web_sys::Url::revoke_object_url(&url).unwrap();
                }
            }
        }
    }
}
```

## Implementation Phases

### Phase 1: Foundation (Week 1-2)
- [ ] Create theme data structures
- [ ] Implement ThemeProvider context
- [ ] Add CSS variable injection system
- [ ] Add localStorage persistence
- [ ] Write comprehensive tests

### Phase 2: Base Theme Selection (Week 2-3)
- [ ] Enhance existing ThemeController
- [ ] Add BaseThemeSelector component
- [ ] Implement theme switching with CSS variable preservation
- [ ] Add transition animations

### Phase 3: Typography Customization (Week 3-4)
- [ ] Create TypographyCustomizer component
- [ ] Add font family selection
- [ ] Implement font scale system
- [ ] Add Google Fonts integration (optional)
- [ ] Create typography preview

### Phase 4: Color Customization (Week 4-5)
- [ ] Create ColorCustomizer component
- [ ] Implement color picker integration
- [ ] Add hex to Oklahoma LCH conversion
- [ ] Create color palette preview
- [ ] Add color accessibility checker

### Phase 5: Border & Spacing (Week 5-6)
- [ ] Create BorderCustomizer component
- [ ] Add border width/radius controls
- [ ] Implement spacing scale
- [ ] Add preview cards

### Phase 6: Component Overrides (Week 6-7)
- [ ] Create ComponentCustomizer
- [ ] Add per-component customization
- [ ] Update library components to use CSS variables
- [ ] Create component showcase with live updates

### Phase 7: Import/Export & Polish (Week 7-8)
- [ ] Implement JSON export
- [ ] Add JSON import with validation
- [ ] Create preset themes gallery
- [ ] Add theme sharing functionality
- [ ] Write documentation
- [ ] Create tutorial videos

## Integration with Existing Components

### Update Component Template

All existing components should be updated to use CSS variables:

```rust
#[component]
pub fn Card(...) -> impl IntoView {
    view! {
        <div
            class=move || {
                merge_classes!(
                    "card",
                    style.get().as_str(),
                    size.get().as_str(),
                    class
                )
            }
            // Add CSS variable support
            style="
                border-width: var(--card-border-width, var(--border-width, 1px));
                border-radius: var(--card-border-radius, var(--border-radius, 0.5rem));
            "
            class:card-side=side
            class:image-full=image_full
        >
            {children()}
        </div>
    }
}
```

## CSS Requirements

Add to `input.css`:

```css
:root {
  /* Typography */
  --font-sans: system-ui, sans-serif;
  --font-serif: Georgia, serif;
  --font-mono: 'Courier New', monospace;
  --font-size-base: 16px;
  --font-size-xs: 0.75rem;
  --font-size-sm: 0.875rem;
  --font-size-lg: 1.125rem;
  --font-size-xl: 1.25rem;

  /* Borders */
  --border-width: 1px;
  --border-color: oklch(var(--b2));
  --border-radius: 0.5rem;

  /* Component-specific */
  --card-border-width: var(--border-width);
  --card-border-radius: var(--border-radius);
  --button-border-radius: var(--border-radius);
  --input-border-width: var(--border-width);

  /* Spacing */
  --spacing-scale: 1;
}

/* Apply font family */
body {
  font-family: var(--font-sans);
  font-size: var(--font-size-base);
}

/* Update component styles */
.card-border {
  border-width: var(--card-border-width);
  border-color: var(--border-color);
}

.btn {
  border-radius: var(--button-border-radius);
}

.input, .textarea, .select {
  border-width: var(--input-border-width);
}
```

## Benefits

1. **User Experience**
   - Intuitive visual customization
   - Real-time preview
   - No code changes required
   - Portable theme configurations

2. **Developer Experience**
   - Type-safe theme configuration
   - Easy integration
   - Backward compatible
   - Extensible architecture

3. **Brand Consistency**
   - Easy to match brand guidelines
   - Consistent across entire application
   - Maintainable design system

4. **Performance**
   - CSS variables for instant updates
   - Minimal runtime overhead
   - Efficient storage (JSON)

## Migration Path

### For Existing Apps

1. Wrap app in ThemeProvider
2. Existing ThemeController continues to work
3. Gradually adopt new customization features
4. No breaking changes

```rust
// Before
view! {
    <App />
}

// After
view! {
    <ThemeProvider>
        <App />
    </ThemeProvider>
}
```

## Future Enhancements

- [ ] AI-powered theme generation
- [ ] A11y contrast checker
- [ ] Theme marketplace
- [ ] Design token export (Figma, Sketch)
- [ ] Animation timing customization
- [ ] Dark/light mode auto-switching
- [ ] Per-component theme inheritance
- [ ] Theme versioning and history

## Conclusion

This strategy provides a comprehensive, production-ready theming system that:
- Leverages daisyUI's existing theme infrastructure
- Provides granular runtime customization
- Maintains type safety and performance
- Offers excellent DX and UX
- Scales from simple to complex use cases

The phased implementation allows incremental adoption and testing, while the modular architecture ensures maintainability and extensibility.
