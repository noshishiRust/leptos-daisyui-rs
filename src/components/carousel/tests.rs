use super::*;


// CarouselModifier tests
#[test]
fn test_carousel_modifier_default() {
    let modifier = CarouselModifier::default();
    assert_eq!(modifier.as_str(), "");
}

#[test]
fn test_carousel_modifier_start() {
    let modifier = CarouselModifier::Start;
    assert_eq!(modifier.as_str(), "carousel-start");
}

#[test]
fn test_carousel_modifier_center() {
    let modifier = CarouselModifier::Center;
    assert_eq!(modifier.as_str(), "carousel-center");
}

#[test]
fn test_carousel_modifier_end() {
    let modifier = CarouselModifier::End;
    assert_eq!(modifier.as_str(), "carousel-end");
}

#[test]
fn test_carousel_modifier_clone() {
    let modifier1 = CarouselModifier::Center;
    let modifier2 = modifier1.clone();
    assert_eq!(modifier1.as_str(), modifier2.as_str());
}

#[test]
fn test_carousel_modifier_debug() {
    let modifier = CarouselModifier::Start;
    assert!(format!("{:?}", modifier).contains("Start"));
}

// CarouselDirection tests
#[test]
fn test_carousel_direction_default() {
    let direction = CarouselDirection::default();
    assert_eq!(direction.as_str(), "carousel-horizontal");
}

#[test]
fn test_carousel_direction_horizontal() {
    let direction = CarouselDirection::Horizontal;
    assert_eq!(direction.as_str(), "carousel-horizontal");
}

#[test]
fn test_carousel_direction_vertical() {
    let direction = CarouselDirection::Vertical;
    assert_eq!(direction.as_str(), "carousel-vertical");
}

#[test]
fn test_carousel_direction_clone() {
    let direction1 = CarouselDirection::Vertical;
    let direction2 = direction1.clone();
    assert_eq!(direction1.as_str(), direction2.as_str());
}

#[test]
fn test_carousel_direction_debug() {
    let direction = CarouselDirection::Horizontal;
    assert!(format!("{:?}", direction).contains("Horizontal"));
}

// Comprehensive enum variant coverage tests
#[test]
fn test_all_carousel_modifiers_return_valid_classes() {
    let modifiers = vec![
        (CarouselModifier::Default, ""),
        (CarouselModifier::Start, "carousel-start"),
        (CarouselModifier::Center, "carousel-center"),
        (CarouselModifier::End, "carousel-end"),
    ];

    for (modifier, expected) in modifiers {
        assert_eq!(modifier.as_str(), expected);
    }
}

#[test]
fn test_all_carousel_directions_return_valid_classes() {
    let directions = vec![
        (CarouselDirection::Horizontal, "carousel-horizontal"),
        (CarouselDirection::Vertical, "carousel-vertical"),
    ];

    for (direction, expected) in directions {
        assert_eq!(direction.as_str(), expected);
    }
}
