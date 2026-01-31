use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    // CardStyle tests
    #[test]
    fn test_card_style_default() {
        let style = CardStyle::default();
        assert_eq!(style.as_str(), "");
    }

    #[test]
    fn test_card_style_border() {
        let style = CardStyle::Border;
        assert_eq!(style.as_str(), "card-border");
    }

    #[test]
    fn test_card_style_dash() {
        let style = CardStyle::Dash;
        assert_eq!(style.as_str(), "card-border border-dashed");
    }

    #[test]
    fn test_card_style_clone() {
        let style1 = CardStyle::Border;
        let style2 = style1.clone();
        assert_eq!(style1.as_str(), style2.as_str());
    }

    #[test]
    fn test_card_style_debug() {
        let style = CardStyle::Dash;
        assert!(format!("{:?}", style).contains("Dash"));
    }

    // CardSize tests
    #[test]
    fn test_card_size_default() {
        let size = CardSize::default();
        assert_eq!(size.as_str(), "card-md");
    }

    #[test]
    fn test_card_size_xs() {
        let size = CardSize::Xs;
        assert_eq!(size.as_str(), "card-xs");
    }

    #[test]
    fn test_card_size_sm() {
        let size = CardSize::Sm;
        assert_eq!(size.as_str(), "card-sm");
    }

    #[test]
    fn test_card_size_md() {
        let size = CardSize::Md;
        assert_eq!(size.as_str(), "card-md");
    }

    #[test]
    fn test_card_size_lg() {
        let size = CardSize::Lg;
        assert_eq!(size.as_str(), "card-lg");
    }

    #[test]
    fn test_card_size_xl() {
        let size = CardSize::Xl;
        assert_eq!(size.as_str(), "card-xl");
    }

    #[test]
    fn test_card_size_clone() {
        let size1 = CardSize::Lg;
        let size2 = size1.clone();
        assert_eq!(size1.as_str(), size2.as_str());
    }

    #[test]
    fn test_card_size_debug() {
        let size = CardSize::Xl;
        assert!(format!("{:?}", size).contains("Xl"));
    }

    // Comprehensive enum variant coverage tests
    #[test]
    fn test_all_card_styles_return_valid_classes() {
        let styles = vec![
            (CardStyle::Default, ""),
            (CardStyle::Border, "card-border"),
            (CardStyle::Dash, "card-border border-dashed"),
        ];

        for (style, expected) in styles {
            assert_eq!(style.as_str(), expected);
        }
    }

    #[test]
    fn test_all_card_sizes_return_valid_classes() {
        let sizes = vec![
            (CardSize::Xs, "card-xs"),
            (CardSize::Sm, "card-sm"),
            (CardSize::Md, "card-md"),
            (CardSize::Lg, "card-lg"),
            (CardSize::Xl, "card-xl"),
        ];

        for (size, expected) in sizes {
            assert_eq!(size.as_str(), expected);
        }
    }
}
