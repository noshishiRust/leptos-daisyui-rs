use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    // ChatPlacement tests
    #[test]
    fn test_chat_placement_default() {
        let placement = ChatPlacement::default();
        assert_eq!(placement.as_str(), "chat-start");
    }

    #[test]
    fn test_chat_placement_start() {
        let placement = ChatPlacement::Start;
        assert_eq!(placement.as_str(), "chat-start");
    }

    #[test]
    fn test_chat_placement_end() {
        let placement = ChatPlacement::End;
        assert_eq!(placement.as_str(), "chat-end");
    }

    #[test]
    fn test_chat_placement_clone() {
        let placement1 = ChatPlacement::End;
        let placement2 = placement1.clone();
        assert_eq!(placement1.as_str(), placement2.as_str());
    }

    #[test]
    fn test_chat_placement_debug() {
        let placement = ChatPlacement::Start;
        assert!(format!("{:?}", placement).contains("Start"));
    }

    // ChatBubbleColor tests
    #[test]
    fn test_chat_bubble_color_default() {
        let color = ChatBubbleColor::default();
        assert_eq!(color.as_str(), "");
    }

    #[test]
    fn test_chat_bubble_color_neutral() {
        let color = ChatBubbleColor::Neutral;
        assert_eq!(color.as_str(), "chat-bubble-neutral");
    }

    #[test]
    fn test_chat_bubble_color_primary() {
        let color = ChatBubbleColor::Primary;
        assert_eq!(color.as_str(), "chat-bubble-primary");
    }

    #[test]
    fn test_chat_bubble_color_secondary() {
        let color = ChatBubbleColor::Secondary;
        assert_eq!(color.as_str(), "chat-bubble-secondary");
    }

    #[test]
    fn test_chat_bubble_color_accent() {
        let color = ChatBubbleColor::Accent;
        assert_eq!(color.as_str(), "chat-bubble-accent");
    }

    #[test]
    fn test_chat_bubble_color_info() {
        let color = ChatBubbleColor::Info;
        assert_eq!(color.as_str(), "chat-bubble-info");
    }

    #[test]
    fn test_chat_bubble_color_success() {
        let color = ChatBubbleColor::Success;
        assert_eq!(color.as_str(), "chat-bubble-success");
    }

    #[test]
    fn test_chat_bubble_color_warning() {
        let color = ChatBubbleColor::Warning;
        assert_eq!(color.as_str(), "chat-bubble-warning");
    }

    #[test]
    fn test_chat_bubble_color_error() {
        let color = ChatBubbleColor::Error;
        assert_eq!(color.as_str(), "chat-bubble-error");
    }

    #[test]
    fn test_chat_bubble_color_clone() {
        let color1 = ChatBubbleColor::Success;
        let color2 = color1.clone();
        assert_eq!(color1.as_str(), color2.as_str());
    }

    #[test]
    fn test_chat_bubble_color_debug() {
        let color = ChatBubbleColor::Warning;
        assert!(format!("{:?}", color).contains("Warning"));
    }

    // Comprehensive enum variant coverage tests
    #[test]
    fn test_all_chat_placements_return_valid_classes() {
        let placements = vec![
            (ChatPlacement::Start, "chat-start"),
            (ChatPlacement::End, "chat-end"),
        ];

        for (placement, expected) in placements {
            assert_eq!(placement.as_str(), expected);
        }
    }

    #[test]
    fn test_all_chat_bubble_colors_return_valid_classes() {
        let colors = vec![
            (ChatBubbleColor::Default, ""),
            (ChatBubbleColor::Neutral, "chat-bubble-neutral"),
            (ChatBubbleColor::Primary, "chat-bubble-primary"),
            (ChatBubbleColor::Secondary, "chat-bubble-secondary"),
            (ChatBubbleColor::Accent, "chat-bubble-accent"),
            (ChatBubbleColor::Info, "chat-bubble-info"),
            (ChatBubbleColor::Success, "chat-bubble-success"),
            (ChatBubbleColor::Warning, "chat-bubble-warning"),
            (ChatBubbleColor::Error, "chat-bubble-error"),
        ];

        for (color, expected) in colors {
            assert_eq!(color.as_str(), expected);
        }
    }
}
