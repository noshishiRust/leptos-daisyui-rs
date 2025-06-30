#[derive(Clone, Debug, Default)]
pub enum ToastPosition {
    #[default]
    TopEnd,
    TopStart,
    TopCenter,
    MiddleStart,
    MiddleCenter,
    MiddleEnd,
    BottomStart,
    BottomCenter,
    BottomEnd,
}

impl ToastPosition {
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
