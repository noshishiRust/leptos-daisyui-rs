use crate::merge_classes;
use leptos::{html::Div, prelude::*};

#[component]
pub fn MockupPhone(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || { merge_classes!("mockup-phone", class) }>
            <div class="camera"></div>
            <div class="display">
                <div class="artboard">{children()}</div>
            </div>
        </div>
    }
}

#[component]
pub fn MockupPhoneCamera(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || { merge_classes!("mockup-phone-camera", class) }>
            <div class="camera"></div>
            <div class="display">
                <div class="artboard">{children()}</div>
            </div>
        </div>
    }
}

#[component]
pub fn MockupPhoneDisplay(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || { merge_classes!("mockup-phone-display", class) }>
            <div class="camera"></div>
            <div class="display">
                <div class="artboard">{children()}</div>
            </div>
        </div>
    }
}
