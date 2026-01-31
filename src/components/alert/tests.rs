use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    // AlertStyle tests
    #[test]
    fn test_alert_style_default() {
        let style = AlertStyle::default();
        assert_eq!(style.as_str(), "");
    }

    #[test]
    fn test_alert_style_outline() {
        let style = AlertStyle::Outline;
        assert_eq!(style.as_str(), "alert-outline");
    }

    #[test]
    fn test_alert_style_dash() {
        let style = AlertStyle::Dash;
        assert_eq!(style.as_str(), "alert-dash");
    }

    #[test]
    fn test_alert_style_soft() {
        let style = AlertStyle::Soft;
        assert_eq!(style.as_str(), "alert-soft");
    }

    #[test]
    fn test_alert_style_clone() {
        let style1 = AlertStyle::Outline;
        let style2 = style1.clone();
        assert_eq!(style1.as_str(), style2.as_str());
    }

    #[test]
    fn test_alert_style_debug() {
        let style = AlertStyle::Dash;
        assert!(format!("{:?}", style).contains("Dash"));
    }

    // AlertColor tests
    #[test]
    fn test_alert_color_default() {
        let color = AlertColor::default();
        assert_eq!(color.as_str(), "");
    }

    #[test]
    fn test_alert_color_info() {
        let color = AlertColor::Info;
        assert_eq!(color.as_str(), "alert-info");
    }

    #[test]
    fn test_alert_color_success() {
        let color = AlertColor::Success;
        assert_eq!(color.as_str(), "alert-success");
    }

    #[test]
    fn test_alert_color_warning() {
        let color = AlertColor::Warning;
        assert_eq!(color.as_str(), "alert-warning");
    }

    #[test]
    fn test_alert_color_error() {
        let color = AlertColor::Error;
        assert_eq!(color.as_str(), "alert-error");
    }

    #[test]
    fn test_alert_color_clone() {
        let color1 = AlertColor::Success;
        let color2 = color1.clone();
        assert_eq!(color1.as_str(), color2.as_str());
    }

    #[test]
    fn test_alert_color_debug() {
        let color = AlertColor::Warning;
        assert!(format!("{:?}", color).contains("Warning"));
    }

    // AlertDirection tests
    #[test]
    fn test_alert_direction_default() {
        let direction = AlertDirection::default();
        assert_eq!(direction.as_str(), "");
    }

    #[test]
    fn test_alert_direction_vertical() {
        let direction = AlertDirection::Vertical;
        assert_eq!(direction.as_str(), "alert-vertical");
    }

    #[test]
    fn test_alert_direction_horizontal() {
        let direction = AlertDirection::Horizontal;
        assert_eq!(direction.as_str(), "alert-horizontal");
    }

    #[test]
    fn test_alert_direction_clone() {
        let direction1 = AlertDirection::Horizontal;
        let direction2 = direction1.clone();
        assert_eq!(direction1.as_str(), direction2.as_str());
    }

    #[test]
    fn test_alert_direction_debug() {
        let direction = AlertDirection::Vertical;
        assert!(format!("{:?}", direction).contains("Vertical"));
    }

    // Comprehensive enum variant coverage tests
    #[test]
    fn test_all_alert_styles_return_valid_classes() {
        let styles = vec![
            (AlertStyle::Default, ""),
            (AlertStyle::Outline, "alert-outline"),
            (AlertStyle::Dash, "alert-dash"),
            (AlertStyle::Soft, "alert-soft"),
        ];

        for (style, expected) in styles {
            assert_eq!(style.as_str(), expected);
        }
    }

    #[test]
    fn test_all_alert_colors_return_valid_classes() {
        let colors = vec![
            (AlertColor::Default, ""),
            (AlertColor::Info, "alert-info"),
            (AlertColor::Success, "alert-success"),
            (AlertColor::Warning, "alert-warning"),
            (AlertColor::Error, "alert-error"),
        ];

        for (color, expected) in colors {
            assert_eq!(color.as_str(), expected);
        }
    }

    #[test]
    fn test_all_alert_directions_return_valid_classes() {
        let directions = vec![
            (AlertDirection::Default, ""),
            (AlertDirection::Vertical, "alert-vertical"),
            (AlertDirection::Horizontal, "alert-horizontal"),
        ];

        for (direction, expected) in directions {
            assert_eq!(direction.as_str(), expected);
        }
    }
}
