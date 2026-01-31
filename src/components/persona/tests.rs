//! Unit tests for Persona component

use super::style::PersonaSize;

#[test]
fn test_persona_size_count() {
    let sizes = [
        PersonaSize::XSmall,
        PersonaSize::Small,
        PersonaSize::Medium,
        PersonaSize::Large,
        PersonaSize::XLarge,
    ];

    assert_eq!(sizes.len(), 5, "Should have 5 persona sizes");
}

#[test]
fn test_persona_avatar_classes() {
    assert_eq!(
        PersonaSize::XSmall.avatar_class(),
        "w-8",
        "XSmall avatar should use w-8"
    );
    assert_eq!(
        PersonaSize::Small.avatar_class(),
        "w-10",
        "Small avatar should use w-10"
    );
    assert_eq!(
        PersonaSize::Medium.avatar_class(),
        "w-12",
        "Medium avatar should use w-12"
    );
    assert_eq!(
        PersonaSize::Large.avatar_class(),
        "w-16",
        "Large avatar should use w-16"
    );
    assert_eq!(
        PersonaSize::XLarge.avatar_class(),
        "w-24",
        "XLarge avatar should use w-24"
    );
}

#[test]
fn test_persona_name_classes() {
    assert_eq!(
        PersonaSize::XSmall.name_class(),
        "text-xs",
        "XSmall name should use text-xs"
    );
    assert_eq!(
        PersonaSize::Small.name_class(),
        "text-sm",
        "Small name should use text-sm"
    );
    assert_eq!(
        PersonaSize::Medium.name_class(),
        "text-base",
        "Medium name should use text-base"
    );
    assert_eq!(
        PersonaSize::Large.name_class(),
        "text-lg",
        "Large name should use text-lg"
    );
    assert_eq!(
        PersonaSize::XLarge.name_class(),
        "text-xl",
        "XLarge name should use text-xl"
    );
}

#[test]
fn test_persona_secondary_classes() {
    assert_eq!(
        PersonaSize::XSmall.secondary_class(),
        "text-xs",
        "XSmall secondary should use text-xs"
    );
    assert_eq!(
        PersonaSize::Small.secondary_class(),
        "text-xs",
        "Small secondary should use text-xs"
    );
    assert_eq!(
        PersonaSize::Medium.secondary_class(),
        "text-sm",
        "Medium secondary should use text-sm"
    );
    assert_eq!(
        PersonaSize::Large.secondary_class(),
        "text-base",
        "Large secondary should use text-base"
    );
    assert_eq!(
        PersonaSize::XLarge.secondary_class(),
        "text-lg",
        "XLarge secondary should use text-lg"
    );
}

#[test]
fn test_persona_size_default() {
    assert_eq!(
        PersonaSize::default(),
        PersonaSize::Medium,
        "Default size should be Medium"
    );
}

#[test]
fn test_persona_size_clone() {
    let size = PersonaSize::Large;
    let cloned = size.clone();
    assert_eq!(size, cloned, "Cloned size should equal original");
}

#[test]
fn test_persona_size_debug() {
    let size = PersonaSize::Small;
    let debug_str = format!("{:?}", size);
    assert!(
        debug_str.contains("Small"),
        "Debug output should contain size name"
    );
}

#[test]
fn test_avatar_sizes_are_ascending() {
    let extract_width = |class: &str| -> u32 { class.trim_start_matches("w-").parse().unwrap() };

    assert!(
        extract_width(PersonaSize::XSmall.avatar_class())
            < extract_width(PersonaSize::Small.avatar_class())
    );
    assert!(
        extract_width(PersonaSize::Small.avatar_class())
            < extract_width(PersonaSize::Medium.avatar_class())
    );
    assert!(
        extract_width(PersonaSize::Medium.avatar_class())
            < extract_width(PersonaSize::Large.avatar_class())
    );
    assert!(
        extract_width(PersonaSize::Large.avatar_class())
            < extract_width(PersonaSize::XLarge.avatar_class())
    );
}

#[test]
fn test_name_text_sizes_are_ascending() {
    let sizes = ["text-xs", "text-sm", "text-base", "text-lg", "text-xl"];

    assert_eq!(PersonaSize::XSmall.name_class(), sizes[0]);
    assert_eq!(PersonaSize::Small.name_class(), sizes[1]);
    assert_eq!(PersonaSize::Medium.name_class(), sizes[2]);
    assert_eq!(PersonaSize::Large.name_class(), sizes[3]);
    assert_eq!(PersonaSize::XLarge.name_class(), sizes[4]);
}

#[test]
fn test_secondary_text_smaller_than_name() {
    // For Medium and larger, secondary text should be smaller than name text
    let medium_name = PersonaSize::Medium.name_class();
    let medium_secondary = PersonaSize::Medium.secondary_class();
    assert_ne!(
        medium_name, medium_secondary,
        "Medium secondary should be different from name"
    );

    let large_name = PersonaSize::Large.name_class();
    let large_secondary = PersonaSize::Large.secondary_class();
    assert_ne!(
        large_name, large_secondary,
        "Large secondary should be different from name"
    );
}
