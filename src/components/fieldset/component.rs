use crate::merge_classes;
use leptos::{
    html::{Fieldset, Legend, P},
    prelude::*,
};

#[component]
pub fn FieldSet(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Fieldset>,
    children: Children,
) -> impl IntoView {
    view! {
        <fieldset node_ref=node_ref class=move || merge_classes!("fieldset", class)>
            {children()}
        </fieldset>
    }
}

#[component]
pub fn FieldsetLegend(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Legend>,
    children: Children,
) -> impl IntoView {
    view! {
        <legend node_ref=node_ref class=move || merge_classes!("fieldset-legend", class)>
            {children()}
        </legend>
    }
}

#[component]
pub fn FieldsetLabel(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<P>,
    children: Children,
) -> impl IntoView {
    view! {
        <p node_ref=node_ref class=move || merge_classes!("label", class)>
            {children()}
        </p>
    }
}
