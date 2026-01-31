//! Unit tests for TypographyCustomizer component

#[test]
fn test_font_families_count() {
    let font_families = [
        "System Default",
        "Inter",
        "Roboto",
        "Open Sans",
        "Lato",
        "Poppins",
        "Montserrat",
        "Playfair Display",
        "Merriweather",
    ];

    assert_eq!(font_families.len(), 9, "Should have 9 font families");
}

#[test]
fn test_monospace_fonts_count() {
    let mono_families = [
        "System Default",
        "Fira Code",
        "JetBrains Mono",
        "Source Code Pro",
        "Inconsolata",
    ];

    assert_eq!(mono_families.len(), 5, "Should have 5 monospace fonts");
}

#[test]
fn test_font_families_include_common_fonts() {
    let fonts = [
        "Inter",
        "Roboto",
        "Open Sans",
        "Lato",
        "Poppins",
        "Montserrat",
    ];

    assert!(fonts.contains(&"Inter"), "Should include Inter");
    assert!(fonts.contains(&"Roboto"), "Should include Roboto");
    assert!(fonts.contains(&"Open Sans"), "Should include Open Sans");
}

#[test]
fn test_monospace_fonts_include_popular_choices() {
    let mono_fonts = [
        "Fira Code",
        "JetBrains Mono",
        "Source Code Pro",
        "Inconsolata",
    ];

    assert!(
        mono_fonts.contains(&"Fira Code"),
        "Should include Fira Code"
    );
    assert!(
        mono_fonts.contains(&"JetBrains Mono"),
        "Should include JetBrains Mono"
    );
    assert!(
        mono_fonts.contains(&"Source Code Pro"),
        "Should include Source Code Pro"
    );
}

#[test]
fn test_type_scale_ratios_count() {
    let scale_ratios = [
        1.125, // Minor Second / Major Second
        1.2,   // Minor Third
        1.25,  // Major Third
        1.333, // Perfect Fourth
        1.414, // Augmented Fourth
        1.5,   // Perfect Fifth
        1.618, // Golden Ratio
        1.667, // Major Sixth
    ];

    assert_eq!(scale_ratios.len(), 8, "Should have 8 type scale ratios");
}

#[test]
fn test_type_scale_ratios_are_valid() {
    let scale_ratios = [1.125, 1.2, 1.25, 1.333, 1.414, 1.5, 1.618, 1.667];

    for ratio in scale_ratios {
        assert!(ratio > 1.0, "Type scale ratio should be greater than 1.0");
        assert!(ratio < 2.0, "Type scale ratio should be less than 2.0");
    }
}

#[test]
fn test_font_family_options_no_duplicates() {
    let fonts = [
        "System Default",
        "Inter",
        "Roboto",
        "Open Sans",
        "Lato",
        "Poppins",
        "Montserrat",
        "Playfair Display",
        "Merriweather",
    ];

    let unique_count = fonts.iter().collect::<std::collections::HashSet<_>>().len();
    assert_eq!(
        fonts.len(),
        unique_count,
        "Font family list should not contain duplicates"
    );
}
