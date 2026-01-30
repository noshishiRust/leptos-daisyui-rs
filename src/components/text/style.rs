#[derive(Clone, Debug, Default, PartialEq)]
pub enum TextSize { ExtraSmall, Small, #[default] Base, Large, ExtraLarge, TwoXL, ThreeXL }
impl TextSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            TextSize::ExtraSmall => "text-xs", TextSize::Small => "text-sm", TextSize::Base => "text-base",
            TextSize::Large => "text-lg", TextSize::ExtraLarge => "text-xl", TextSize::TwoXL => "text-2xl",
            TextSize::ThreeXL => "text-3xl"
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum TextWeight { #[default] Normal, Medium, Semibold, Bold }
impl TextWeight {
    pub fn as_str(&self) -> &'static str {
        match self {
            TextWeight::Normal => "font-normal", TextWeight::Medium => "font-medium",
            TextWeight::Semibold => "font-semibold", TextWeight::Bold => "font-bold"
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum TextColor { #[default] Default, Primary, Secondary, Accent, Error, Success, Warning, Info }
impl TextColor {
    pub fn as_str(&self) -> &'static str {
        match self {
            TextColor::Default => "", TextColor::Primary => "text-primary", TextColor::Secondary => "text-secondary",
            TextColor::Accent => "text-accent", TextColor::Error => "text-error", TextColor::Success => "text-success",
            TextColor::Warning => "text-warning", TextColor::Info => "text-info"
        }
    }
}
