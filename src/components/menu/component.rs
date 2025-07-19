use super::style::{MenuDirection, MenuSize};
use crate::merge_classes;
use leptos::{
    html::{H2, Li, Ul},
    prelude::*,
};

#[component]
pub fn Menu(
    #[prop(optional)] manual: bool,
    #[prop(optional)] selected: RwSignal<Option<String>>,
    #[prop(optional, into)] direction: Signal<MenuDirection>,
    #[prop(optional, into)] size: Signal<MenuSize>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Ul>,
    children: Children,
) -> impl IntoView {
    let manager = MenuManager { manual, selected };
    provide_context(manager);

    view! {
        <ul
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "menu",
                    direction.get().as_str(),
                    size.get().as_str(),
                    class
                )
            }
        >
            {children()}
        </ul>
    }
}

#[component]
pub fn MenuItem(
    #[prop(optional, into)] href: Signal<String>,
    #[prop(optional, into)] value: Signal<String>,
    #[prop(optional, into)] active: Signal<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Li>,
    #[prop(optional, into)] is_submenu: bool,
    children: Children,
) -> impl IntoView {
    let MenuManager { manual, selected } = MenuManager::expect_context();

    let on_click = move |_| {
        if disabled.get_untracked() {
            return;
        }

        if value.get_untracked().is_empty() {
            return;
        }

        let mut selected = selected.write();
        *selected = Some(value.get_untracked());
    };

    let is_active = move || {
        if manual {
            return active.get();
        }

        selected
            .get()
            .as_ref()
            .is_some_and(|s| s == &value.get_untracked())
    };

    view! {
        <li node_ref=node_ref on:click=on_click class=class>
            {if !is_submenu {
                view! {
                    <a href=href class:menu-active=is_active>
                        {children()}
                    </a>
                }
                    .into_any()
            } else {
                view! { {children()} }.into_any()
            }}

        </li>
    }
}

#[component]
pub fn MenuTitle(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<H2>,
    children: Children,
) -> impl IntoView {
    view! {
        <h2 node_ref=node_ref class=move || merge_classes!("menu-title", class)>
            {children()}
        </h2>
    }
}

#[component]
pub fn SubMenu(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Ul>,
    children: Children,
) -> impl IntoView {
    view! {
        <ul node_ref=node_ref class=class>
            {children()}
        </ul>
    }
}

#[derive(Clone)]
pub(crate) struct MenuManager {
    manual: bool,
    selected: RwSignal<Option<String>>,
}

impl MenuManager {
    pub fn expect_context() -> Self {
        expect_context()
    }
}
