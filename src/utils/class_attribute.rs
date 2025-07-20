pub use leptos::{
    prelude::RenderEffect,
    tachys::{
        html::class::IntoClass,
        renderer::{Rndr, types},
    },
};

// Reference Thaw UI
// https://github.com/thaw-ui/thaw/blob/main/thaw_utils/src/class_list.rs

/// Struct for building Class
#[derive(Debug, Clone)]
pub struct ClassAttributes {
    values: Vec<ClassAttribute>,
}

impl ClassAttributes {
    /// Creates a new instance of `ClassAttributes`
    pub fn new() -> Self {
        ClassAttributes { values: Vec::new() }
    }

    /// Adds a class to the list of classes
    pub fn add_class(mut self, class: impl Into<ClassAttribute>) -> Self {
        self.values.push(class.into());

        self
    }

    /// Build the class string from the attributes
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

/// ClassAttribute enum to represent different types of class attributes
#[derive(Debug, Clone)]
pub enum ClassAttribute {
    /// No class attribute
    None,

    /// Dynamic class attribute that can change
    Dynamic(String),

    /// Static class attribute that does not change
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

/// Macro to merge multiple class names into a single ClassAttributes instance
#[macro_export]
macro_rules! merge_classes {
    ($($name:expr),+) => {
        {
            use $crate::utils::ClassAttributes;
            ClassAttributes::new()$(.add_class($name))+
        }
    };
}
