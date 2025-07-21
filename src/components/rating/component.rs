use super::style::RatingSize;
use crate::merge_classes;
use leptos::{
    html::{Div, Input},
    prelude::*,
};

/// # Rating Component
///
/// A container for star rating inputs that allows users to rate items.
/// Supports radio button groups for single ratings.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("rating rating-hidden rating-xs rating-sm rating-md rating-lg rating-xl");
/// ```
///
/// ## Node References
/// - `node_ref` - References the rating `<div>` element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn Rating(
    /// Size of the rating display
    #[prop(optional, into)] 
    size: Signal<RatingSize>,

    /// Additional CSS classes to apply to the rating container
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the rating `<div>` element
    #[prop(optional)] 
    node_ref: NodeRef<Div>,

    /// Child [`RatingItem`] and optionally [`RatingHidden`] components
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=move || {
                merge_classes!("rating",
                size.get().as_str(),
                class)
            }
        >
            {children()}
        </div>
    }
}

/// # Rating Item Component
///
/// An individual star in the rating system. Uses radio inputs to ensure
/// only one rating can be selected at a time.
///
/// ## Node References
/// - `node_ref` - References the item `<input>` element ([HTMLInputElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement))
#[component]
pub fn RatingItem(
    /// Whether this rating star is selected
    #[prop(optional, into)] 
    checked: Signal<bool>,

    /// Radio group name for the rating (all items should share the same name)
    #[prop(optional)] 
    name: Option<&'static str>,

    /// Value for this rating level
    #[prop(optional)] 
    value: Option<&'static str>,

    /// Additional CSS classes to apply to the rating item
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the item `<input>` element
    #[prop(optional)] 
    node_ref: NodeRef<Input>,

    /// Change handler when this rating is selected
    #[prop(optional)] 
    on_change: Option<Box<dyn Fn(bool)>>,
) -> impl IntoView {
    view! {
        <input
            node_ref=node_ref
            type="radio"
            name=name
            value=value
            class=move || merge_classes!("mask", "mask-star-2", "bg-orange-400", class)
            prop:checked=checked
            on:change=move |ev| {
                if let Some(handler) = &on_change {
                    handler(event_target_checked(&ev));
                }
            }
        />
    }
}

/// # Rating Hidden Component
///
/// A hidden radio input that allows users to clear/reset the rating.
/// Should be the first item in the rating group.
///
/// ## Node References
/// - `node_ref` - References the hidden `<input>` element ([HTMLInputElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement))
#[component]
pub fn RatingHidden(
    /// Radio group name that matches the rating items
    #[prop(optional)] 
    name: Option<&'static str>,

    /// Additional CSS classes to apply to the hidden input
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the hidden `<input>` element
    #[prop(optional)] 
    node_ref: NodeRef<Input>,
) -> impl IntoView {
    view! {
        <input
            node_ref=node_ref
            type="radio"
            name=name
            class=move || merge_classes!("rating-hidden", class)
        />
    }
}
