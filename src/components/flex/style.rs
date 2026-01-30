/// Flex direction options
#[derive(Clone, Debug, Default, PartialEq)]
pub enum FlexDirection {
    /// Horizontal layout (default)
    #[default]
    Row,
    /// Vertical layout
    Column,
}

impl FlexDirection {
    /// Returns the CSS class for the flex direction
    pub fn as_str(&self) -> &'static str {
        match self {
            FlexDirection::Row => "flex-row",
            FlexDirection::Column => "flex-col",
        }
    }
}

/// Flex wrap options
#[derive(Clone, Debug, Default, PartialEq)]
pub enum FlexWrap {
    /// No wrapping (default)
    #[default]
    NoWrap,
    /// Wrap items
    Wrap,
}

impl FlexWrap {
    /// Returns the CSS class for the flex wrap
    pub fn as_str(&self) -> &'static str {
        match self {
            FlexWrap::NoWrap => "flex-nowrap",
            FlexWrap::Wrap => "flex-wrap",
        }
    }
}

/// Justify content alignment options
#[derive(Clone, Debug, Default, PartialEq)]
pub enum FlexJustify {
    /// Align to start (default)
    #[default]
    Start,
    /// Center alignment
    Center,
    /// Align to end
    End,
    /// Space between items
    Between,
    /// Space around items
    Around,
}

impl FlexJustify {
    /// Returns the CSS class for justify content
    pub fn as_str(&self) -> &'static str {
        match self {
            FlexJustify::Start => "justify-start",
            FlexJustify::Center => "justify-center",
            FlexJustify::End => "justify-end",
            FlexJustify::Between => "justify-between",
            FlexJustify::Around => "justify-around",
        }
    }
}

/// Align items alignment options
#[derive(Clone, Debug, Default, PartialEq)]
pub enum FlexAlign {
    /// Align to start (default)
    #[default]
    Start,
    /// Center alignment
    Center,
    /// Align to end
    End,
    /// Stretch items
    Stretch,
}

impl FlexAlign {
    /// Returns the CSS class for align items
    pub fn as_str(&self) -> &'static str {
        match self {
            FlexAlign::Start => "items-start",
            FlexAlign::Center => "items-center",
            FlexAlign::End => "items-end",
            FlexAlign::Stretch => "items-stretch",
        }
    }
}
