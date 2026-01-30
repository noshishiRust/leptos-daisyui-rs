/// Direction for FAB flower layout expansion
///
/// Controls which quadrant the speed dial buttons fan out into when using flower layout.
/// This is essential for responsive design to prevent buttons from rendering off-screen.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum FabDirection {
    /// Buttons fan out to bottom-right (default)
    /// Best when FAB is in top-left area
    #[default]
    BottomRight,

    /// Buttons fan out to bottom-left
    /// Best when FAB is in top-right area
    BottomLeft,

    /// Buttons fan out to top-right
    /// Best when FAB is in bottom-left area
    TopRight,

    /// Buttons fan out to top-left
    /// Best when FAB is in bottom-right area
    TopLeft,
}

impl FabDirection {
    /// Returns the CSS class name for this direction
    pub fn as_str(&self) -> &'static str {
        match self {
            FabDirection::BottomRight => "fab-flower-bottom-right",
            FabDirection::BottomLeft => "fab-flower-bottom-left",
            FabDirection::TopRight => "fab-flower-top-right",
            FabDirection::TopLeft => "fab-flower-top-left",
        }
    }
}
