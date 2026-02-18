/// # Toast Position Variants
///
/// Style enum for daisyUI toast positioning classes that control where notification
/// toasts appear on the screen. Supports all 9 corners and edges of the viewport.
#[derive(Clone, Debug, Default)]
pub enum ToastPosition {
    /// Top-right corner of the screen
    TopEnd,

    /// Top-left corner of the screen
    TopStart,

    /// Top center edge of the screen
    TopCenter,

    /// Middle-left edge of the screen
    MiddleStart,

    /// Center of the screen
    MiddleCenter,

    /// Middle-right edge of the screen
    MiddleEnd,

    /// Bottom-left corner of the screen
    BottomStart,

    /// Bottom center edge of the screen
    BottomCenter,

    /// Bottom-right corner of the screen (default position)
    #[default]
    BottomEnd,
}

impl ToastPosition {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            ToastPosition::TopEnd => "toast-top toast-end",
            ToastPosition::TopStart => "toast-top toast-start",
            ToastPosition::TopCenter => "toast-top toast-center",
            ToastPosition::MiddleStart => "toast-middle toast-start",
            ToastPosition::MiddleCenter => "toast-middle toast-center",
            ToastPosition::MiddleEnd => "toast-middle toast-end",
            ToastPosition::BottomStart => "toast-bottom toast-start",
            ToastPosition::BottomCenter => "toast-bottom toast-center",
            ToastPosition::BottomEnd => "toast-bottom toast-end",
        }
    }
}
