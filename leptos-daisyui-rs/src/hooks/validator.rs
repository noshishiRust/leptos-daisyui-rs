//! # use_field Hook
//!
//! Form field validation with error state management.

use leptos::prelude::*;

/// A validation rule: takes an input string, returns `Ok(())` if valid,
/// or `Err(message)` with the error description.
pub type ValidationFn = Box<dyn Fn(&str) -> Result<(), String>>;

/// State for a single validated form field.
///
/// Tracks the field value, validation error, and touched state.
/// Use with the existing [`Validator`](crate::components::Validator) and
/// [`ValidatorHint`](crate::components::ValidatorHint) components.
///
/// ## Example
///
/// ```rust,ignore
/// let email = use_field(String::new());
/// let rules: Vec<ValidationFn> = vec![
///     Box::new(|v| if v.is_empty() { Err("Required".into()) } else { Ok(()) }),
///     Box::new(|v| if v.contains('@') { Ok(()) } else { Err("Invalid email".into()) }),
/// ];
///
/// view! {
///     <Validator>
///         <Input
///             on:input:target=move |ev| email.value.set(event_target_value(&ev))
///             on:blur=move |_| { email.touch(); email.validate(&rules); }
///         />
///     </Validator>
///     {move || email.error.get().map(|e| view! { <ValidatorHint>{e}</ValidatorHint> })}
/// }
/// ```
#[derive(Clone)]
pub struct FieldState {
    /// Current value of the field.
    pub value: RwSignal<String>,
    /// Current validation error, if any. `None` means valid or not yet validated.
    pub error: RwSignal<Option<String>>,
    /// Whether the user has interacted with this field.
    pub touched: RwSignal<bool>,
}

impl FieldState {
    /// Validate the field using the provided rules.
    ///
    /// Returns `true` if all rules pass, `false` otherwise.
    /// Sets the `error` signal to the first failing rule's message.
    pub fn validate(&self, rules: &[ValidationFn]) -> bool {
        let val = self.value.get();
        self.touched.set(true);
        for rule in rules {
            if let Err(msg) = rule(&val) {
                self.error.set(Some(msg));
                return false;
            }
        }
        self.error.set(None);
        true
    }

    /// Whether this field is currently valid (no error).
    pub fn is_valid(&self) -> bool {
        self.error.get().is_none()
    }

    /// Mark the field as touched (user has interacted).
    pub fn touch(&self) {
        self.touched.set(true);
    }

    /// Reset the field to its initial state.
    pub fn reset(&self) {
        self.value.set(String::new());
        self.error.set(None);
        self.touched.set(false);
    }
}

/// Create a new validated field with an initial value.
///
/// Returns a [`FieldState`] for managing form field validation.
///
/// ## Example
///
/// ```rust,ignore
/// let name = use_field(String::new());
/// let rules: Vec<ValidationFn> = vec![
///     Box::new(|v| if v.is_empty() { Err("Name is required".into()) } else { Ok(()) }),
/// ];
///
/// view! {
///     <Validator>
///         <Input
///             on:input:target=move |ev| name.value.set(event_target_value(&ev))
///             on:blur=move |_| name.validate(&rules)
///         />
///     </Validator>
///     {move || name.error.get().map(|e| view! { <ValidatorHint>{e}</ValidatorHint> })}
/// }
/// ```
pub fn use_field(initial_value: String) -> FieldState {
    FieldState {
        value: RwSignal::new(initial_value),
        error: RwSignal::new(None),
        touched: RwSignal::new(false),
    }
}
