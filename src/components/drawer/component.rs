use super::style::DrawerPlacement;
use crate::merge_classes;
use leptos::{
    html::{Div, Input},
    prelude::*,
};

#[component]
pub fn Drawer(
    #[prop(optional, into)] placement: Signal<DrawerPlacement>,
    #[prop(optional, into)] open: Signal<bool>,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "drawer",
                    placement.get().as_str(),
                    class
                )
            }
            class:drawer-open=open
        >
            {children()}
        </div>
    }
}

#[component]
pub fn DrawerToggle(
    id: &'static str,
    #[prop(optional, into)] checked: Signal<bool>,
    #[prop(optional)] node_ref: NodeRef<Input>,
) -> impl IntoView {
    view! {
        <input node_ref=node_ref id=id type="checkbox" class="drawer-toggle" prop:checked=checked />
    }
}

#[component]
pub fn DrawerContent(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("drawer-content", class)>
            {children()}
        </div>
    }
}

#[component]
pub fn DrawerSide(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("drawer-side", class)>
            {children()}
        </div>
    }
}

#[component]
pub fn DrawerOverlay(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
) -> impl IntoView {
    view! { <div node_ref=node_ref class=move || merge_classes!("drawer-overlay", class)></div> }
}
