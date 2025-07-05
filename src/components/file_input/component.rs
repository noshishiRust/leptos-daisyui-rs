use super::style::{FileInputColor, FileInputSize, FileInputStyle};
use crate::merge_classes;
use leptos::{html::Input, prelude::*};

#[component]
pub fn FileInput(
    #[prop(optional, into)] style: Signal<FileInputStyle>,
    #[prop(optional, into)] color: Signal<FileInputColor>,
    #[prop(optional, into)] size: Signal<FileInputSize>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Input>,
    #[prop(optional)] on_change: Option<Box<dyn Fn(leptos::ev::Event)>>,
) -> impl IntoView {
    view! {
        <input
            node_ref=node_ref
            type="file"
            class=move || {
                merge_classes!(
                    "file-input",
                style.get().as_str(),
                color.get().as_str(),
                size.get().as_str(),
                class
                )
            }
            prop:disabled=disabled
            on:change=move |ev| {
                if let Some(handler) = &on_change {
                    handler(ev);
                }
            }
        />
    }
}
