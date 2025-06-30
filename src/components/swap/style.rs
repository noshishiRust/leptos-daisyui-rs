#[derive(Clone, Debug, Default)]
pub enum SwapRotate {
    #[default]
    None,
    Rotate,
    Flip,
}

impl SwapRotate {
    pub fn as_str(&self) -> &'static str {
        match self {
            SwapRotate::None => "",
            SwapRotate::Rotate => "swap-rotate",
            SwapRotate::Flip => "swap-flip",
        }
    }
}
