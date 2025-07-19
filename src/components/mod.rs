mod accordion;
mod alert;
mod avatar;
mod badge;
mod breadcrumbs;
mod button;
mod calendar;
mod card;
mod carousel;
mod chat;
mod checkbox;
mod collapse;
mod countdown;
mod diff;
mod divider;
mod dock;
mod drawer;
mod dropdown;
mod fieldset;
mod file_input;
mod filter;
mod footer;
mod hero;
mod indicator;
mod input;
mod join;
mod kbd;
mod label;
mod link;
mod list;
mod loading;
mod mask;
mod menu;
mod mockup_browser;
mod mockup_code;
mod mockup_phone;
mod mockup_window;
mod modal;
mod navbar;
mod pagination;
mod progress;
mod radial_progress;
mod radio;
mod range;
mod rating;
mod select;
mod skeleton;
mod stack;
mod stats;
mod status;
mod steps;
mod swap;
mod tab;
mod table;
mod textarea;
mod theme_controller;
mod timeline;
mod toast;
mod toggle;
mod validator;

pub use accordion::*;
pub use alert::*;
pub use avatar::*;
pub use badge::*;
pub use breadcrumbs::*;
pub use button::*;
pub use calendar::*;
pub use card::*;
pub use carousel::*;
pub use chat::*;
pub use checkbox::*;
pub use collapse::*;
pub use countdown::*;
pub use diff::*;
pub use divider::*;
pub use dock::*;
pub use drawer::*;
pub use dropdown::*;
pub use fieldset::*;
pub use file_input::*;
pub use filter::*;
pub use footer::*;
pub use hero::*;
pub use indicator::*;
pub use input::*;
pub use join::*;
pub use kbd::*;
pub use label::*;
pub use link::*;
pub use list::*;
pub use loading::*;
pub use mask::*;
pub use menu::*;
pub use mockup_browser::*;
pub use mockup_code::*;
pub use mockup_phone::*;
pub use mockup_window::*;
pub use modal::*;
pub use navbar::*;
pub use pagination::*;
pub use progress::*;
pub use radial_progress::*;
pub use radio::*;
pub use range::*;
pub use rating::*;
pub use select::*;
pub use skeleton::*;
pub use stack::*;
pub use stats::*;
pub use status::*;
pub use steps::*;
pub use swap::*;
pub use tab::*;
pub use table::*;
pub use textarea::*;
pub use theme_controller::*;
pub use timeline::*;
pub use toast::*;
pub use toggle::*;
pub use validator::*;

pub use leptos::{
    prelude::RenderEffect,
    tachys::{
        html::class::IntoClass,
        renderer::{Rndr, types},
    },
};

// Reference Thaw UI
// https://github.com/thaw-ui/thaw/blob/main/thaw_utils/src/class_list.rs
#[derive(Debug, Clone)]
pub struct ClassAttributes {
    values: Vec<ClassAttribute>,
}

impl ClassAttributes {
    pub fn new() -> Self {
        ClassAttributes { values: Vec::new() }
    }

    pub fn add_class(mut self, class: impl Into<ClassAttribute>) -> Self {
        self.values.push(class.into());

        self
    }

    pub fn to_class(&self) -> String {
        self.values
            .iter()
            .filter_map(|attr| match attr {
                ClassAttribute::None => None,
                ClassAttribute::Dynamic(s) => {
                    if !s.is_empty() {
                        Some(s.clone())
                    } else {
                        None
                    }
                }
                ClassAttribute::Static(s) => {
                    if !s.is_empty() {
                        Some(s.to_string())
                    } else {
                        None
                    }
                }
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
}

impl Default for ClassAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoClass for ClassAttributes {
    type AsyncOutput = Self;
    type State = RenderEffect<(types::Element, String)>;
    type Cloneable = Self;
    type CloneableOwned = Self;

    fn html_len(&self) -> usize {
        self.values.len()
    }

    fn to_html(self, _class: &mut String) {
        self.to_class();
    }

    fn hydrate<const FROM_SERVER: bool>(self, el: &types::Element) -> Self::State {
        let el = el.to_owned();
        RenderEffect::new(move |prev| {
            let class = self.to_class();

            if let Some(state) = prev {
                let (el, prev_class) = state;
                if class != prev_class {
                    Rndr::set_attribute(&el, "class", &class);
                    (el, class)
                } else {
                    (el, prev_class)
                }
            } else {
                if !class.is_empty() && !FROM_SERVER {
                    Rndr::set_attribute(&el, "class", &class);
                }

                (el.clone(), class)
            }
        })
    }

    fn build(self, el: &types::Element) -> Self::State {
        let el = el.to_owned();
        RenderEffect::new(move |prev| {
            let class = self.to_class();

            if let Some(state) = prev {
                let (el, prev_class) = state;
                if class != prev_class {
                    Rndr::set_attribute(&el, "class", &class);
                    (el, class)
                } else {
                    (el, prev_class)
                }
            } else {
                if !class.is_empty() {
                    Rndr::set_attribute(&el, "class", &class);
                }
                (el.clone(), class)
            }
        })
    }

    fn rebuild(self, state: &mut Self::State) {
        let prev = state.take_value();
        *state = RenderEffect::new_with_value(
            move |prev| {
                if let Some(state) = prev {
                    let class = self.to_class();
                    let (el, prev_class) = state;
                    if class != *prev_class {
                        Rndr::set_attribute(&el, "class", &class);
                        (el, class)
                    } else {
                        (el, prev_class)
                    }
                } else {
                    unreachable!()
                }
            },
            prev,
        );
    }

    fn into_cloneable(self) -> Self::Cloneable {
        self
    }

    fn into_cloneable_owned(self) -> Self::CloneableOwned {
        self
    }

    fn dry_resolve(&mut self) {}

    async fn resolve(self) -> Self::AsyncOutput {
        self
    }

    fn reset(state: &mut Self::State) {
        *state = RenderEffect::new_with_value(
            move |prev| {
                if let Some(state) = prev {
                    let (el, _) = &state;
                    Rndr::remove_attribute(el, "class");
                    state
                } else {
                    unreachable!()
                }
            },
            state.take_value(),
        );
    }
}

#[derive(Debug, Clone)]
pub enum ClassAttribute {
    None,
    Dynamic(String),
    Static(&'static str),
}

impl From<&'static str> for ClassAttribute {
    fn from(value: &'static str) -> Self {
        ClassAttribute::Static(value)
    }
}

impl From<Option<&'static str>> for ClassAttribute {
    fn from(value: Option<&'static str>) -> Self {
        match value {
            Some(v) => ClassAttribute::Static(v),
            None => ClassAttribute::None,
        }
    }
}

impl From<String> for ClassAttribute {
    fn from(value: String) -> Self {
        ClassAttribute::Dynamic(value)
    }
}

impl From<Option<String>> for ClassAttribute {
    fn from(value: Option<String>) -> Self {
        match value {
            Some(v) => ClassAttribute::Dynamic(v),
            None => ClassAttribute::None,
        }
    }
}

impl<F> From<F> for ClassAttribute
where
    F: Fn() -> &'static str,
{
    fn from(value: F) -> Self {
        ClassAttribute::Dynamic(value().to_string())
    }
}

#[macro_export]
macro_rules! merge_classes {
    ($($name:expr),+) => {
        {
            use $crate::components::ClassAttributes;
            ClassAttributes::new()$(.add_class($name))+
        }
    };
}
