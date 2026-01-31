use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    // CheckboxColor tests
    #[test]
    fn test_checkbox_color_default() {
        let color = CheckboxColor::default();
        assert_eq!(color.as_str(), "");
    }

    #[test]
    fn test_checkbox_color_primary() {
        let color = CheckboxColor::Primary;
        assert_eq!(color.as_str(), "checkbox-primary");
    }

    #[test]
    fn test_checkbox_color_secondary() {
        let color = CheckboxColor::Secondary;
        assert_eq!(color.as_str(), "checkbox-secondary");
    }

    #[test]
    fn test_checkbox_color_accent() {
        let color = CheckboxColor::Accent;
        assert_eq!(color.as_str(), "checkbox-accent");
    }

    #[test]
    fn test_checkbox_color_neutral() {
        let color = CheckboxColor::Neutral;
        assert_eq!(color.as_str(), "checkbox-neutral");
    }

    #[test]
    fn test_checkbox_color_success() {
        let color = CheckboxColor::Success;
        assert_eq!(color.as_str(), "checkbox-success");
    }

    #[test]
    fn test_checkbox_color_warning() {
        let color = CheckboxColor::Warning;
        assert_eq!(color.as_str(), "checkbox-warning");
    }

    #[test]
    fn test_checkbox_color_info() {
        let color = CheckboxColor::Info;
        assert_eq!(color.as_str(), "checkbox-info");
    }

    #[test]
    fn test_checkbox_color_error() {
        let color = CheckboxColor::Error;
        assert_eq!(color.as_str(), "checkbox-error");
    }

    #[test]
    fn test_checkbox_color_clone() {
        let color1 = CheckboxColor::Primary;
        let color2 = color1.clone();
        assert_eq!(color1.as_str(), color2.as_str());
    }

    #[test]
    fn test_checkbox_color_debug() {
        let color = CheckboxColor::Success;
        assert!(format!("{:?}", color).contains("Success"));
    }

    // CheckboxSize tests
    #[test]
    fn test_checkbox_size_default() {
        let size = CheckboxSize::default();
        assert_eq!(size.as_str(), "checkbox-md");
    }

    #[test]
    fn test_checkbox_size_xs() {
        let size = CheckboxSize::Xs;
        assert_eq!(size.as_str(), "checkbox-xs");
    }

    #[test]
    fn test_checkbox_size_sm() {
        let size = CheckboxSize::Sm;
        assert_eq!(size.as_str(), "checkbox-sm");
    }

    #[test]
    fn test_checkbox_size_md() {
        let size = CheckboxSize::Md;
        assert_eq!(size.as_str(), "checkbox-md");
    }

    #[test]
    fn test_checkbox_size_lg() {
        let size = CheckboxSize::Lg;
        assert_eq!(size.as_str(), "checkbox-lg");
    }

    #[test]
    fn test_checkbox_size_xl() {
        let size = CheckboxSize::Xl;
        assert_eq!(size.as_str(), "checkbox-xl");
    }

    #[test]
    fn test_checkbox_size_clone() {
        let size1 = CheckboxSize::Lg;
        let size2 = size1.clone();
        assert_eq!(size1.as_str(), size2.as_str());
    }

    #[test]
    fn test_checkbox_size_debug() {
        let size = CheckboxSize::Xl;
        assert!(format!("{:?}", size).contains("Xl"));
    }

    // Comprehensive enum variant coverage tests
    #[test]
    fn test_all_checkbox_colors_return_valid_classes() {
        let colors = vec![
            (CheckboxColor::Default, ""),
            (CheckboxColor::Primary, "checkbox-primary"),
            (CheckboxColor::Secondary, "checkbox-secondary"),
            (CheckboxColor::Accent, "checkbox-accent"),
            (CheckboxColor::Neutral, "checkbox-neutral"),
            (CheckboxColor::Success, "checkbox-success"),
            (CheckboxColor::Warning, "checkbox-warning"),
            (CheckboxColor::Info, "checkbox-info"),
            (CheckboxColor::Error, "checkbox-error"),
        ];

        for (color, expected) in colors {
            assert_eq!(color.as_str(), expected);
        }
    }

    #[test]
    fn test_all_checkbox_sizes_return_valid_classes() {
        let sizes = vec![
            (CheckboxSize::Xs, "checkbox-xs"),
            (CheckboxSize::Sm, "checkbox-sm"),
            (CheckboxSize::Md, "checkbox-md"),
            (CheckboxSize::Lg, "checkbox-lg"),
            (CheckboxSize::Xl, "checkbox-xl"),
        ];

        for (size, expected) in sizes {
            assert_eq!(size.as_str(), expected);
        }
    }
}
