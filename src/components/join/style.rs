#[derive(Clone, Debug, Default)]
pub enum JoinDirection {
    #[default]
    Horizontal,
    Vertical,
}

impl JoinDirection {
    pub fn as_str(&self) -> &'static str {
        match self {
            JoinDirection::Horizontal => "join-horizontal",
            JoinDirection::Vertical => "join-vertical",
        }
    }
}