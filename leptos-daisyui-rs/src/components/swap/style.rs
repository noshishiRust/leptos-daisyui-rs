/// # Swap Rotate Variants
///
/// Style enum for daisyUI swap animation classes that control the transition effect
/// when swapping between elements.
#[derive(Clone, Debug, Default)]
pub enum SwapRotate {
    /// No rotation animation (default)
    #[default]
    None,
    /// Rotation animation for swap transition
    Rotate,
    /// Flip animation for swap transition
    Flip,
}

impl SwapRotate {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            SwapRotate::None => "",
            SwapRotate::Rotate => "swap-rotate",
            SwapRotate::Flip => "swap-flip",
        }
    }
}
