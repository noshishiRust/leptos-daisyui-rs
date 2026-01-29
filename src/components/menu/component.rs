use super::style::{MenuDirection, MenuSize};
use crate::merge_classes;
use leptos::{
    html::{H2, Li, Ul},
    prelude::*,
};

/// # Menu Component
///
/// A reactive Leptos wrapper for daisyUI's menu component that provides
/// structured navigation with automatic selection tracking and hierarchical organization.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("menu menu-horizontal menu-vertical menu-xs menu-sm menu-md menu-lg menu-xl menu-title menu-active");
/// ```
///
/// ## Node References
/// - `node_ref` - References the ul element ([HTMLUListElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLUListElement))
#[component]
pub fn Menu(
    /// If true, disables automatic selection tracking
    #[prop(optional)]
    manual: bool,

    /// Signal for tracking the currently selected menu item value
    #[prop(optional)]
    selected: RwSignal<Option<String>>,

    /// Layout direction of menu items
    #[prop(optional, into)]
    direction: Signal<MenuDirection>,

    /// Size variant for menu items
    #[prop(optional, into)]
    size: Signal<MenuSize>,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Reference to the ul element
    #[prop(optional)]
    node_ref: NodeRef<Ul>,

    /// Menu content
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

/// # Menu Item Component
///
/// A reactive Leptos wrapper for individual menu items with selection tracking
/// and navigation support.
///
/// ## Node References
/// - `node_ref` - References the li element ([HTMLLIElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLIElement))
#[component]
pub fn MenuItem(
    /// Optional URL for navigation
    #[prop(optional, into)]
    href: Signal<String>,

    /// Unique identifier for selection tracking
    #[prop(optional, into)]
    value: Signal<String>,

    /// Manual active state (manual mode only)
    #[prop(optional, into)]
    active: Signal<bool>,

    /// Whether the item is disabled
    #[prop(optional, into)]
    disabled: Signal<bool>,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Reference to the li element
    #[prop(optional)]
    node_ref: NodeRef<Li>,

    /// If true, renders without anchor wrapper
    #[prop(optional, into)]
    is_submenu: bool,

    /// Item content
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

/// # Menu Title Component
///
/// A reactive Leptos wrapper for menu section titles that provide
/// visual organization of menu items into groups.
///
/// ## Node References
/// - `node_ref` - References the h2 element ([HTMLHeadingElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHeadingElement))
#[component]
pub fn MenuTitle(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Reference to the h2 element
    #[prop(optional)]
    node_ref: NodeRef<H2>,

    /// Title text
    children: Children,
) -> impl IntoView {
    view! {
        <h2 node_ref=node_ref class=move || merge_classes!("menu-title", class)>
            {children()}
        </h2>
    }
}

/// # SubMenu Component
///
/// A reactive Leptos wrapper for nested submenu containers within menu items.
/// Used for hierarchical navigation structures.
///
/// ## Node References
/// - `node_ref` - References the ul element ([HTMLUListElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLUListElement))
#[component]
pub fn SubMenu(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Reference to the ul element
    #[prop(optional)]
    node_ref: NodeRef<Ul>,

    /// Submenu content
    children: Children,
) -> impl IntoView {
    view! {
        <ul node_ref=node_ref class=class>
            {children()}
        </ul>
    }
}

/// Internal context manager for menu selection state.
#[derive(Clone)]
pub(crate) struct MenuManager {
    /// Whether the menu operates in manual mode (disables automatic selection)
    manual: bool,
    /// Signal tracking the currently selected menu item value
    selected: RwSignal<Option<String>>,
}

impl MenuManager {
    /// Retrieves the MenuManager from context.
    ///
    /// # Panics
    /// Panics if MenuItem or MenuDropdown is used outside of a Menu component.
    pub fn expect_context() -> Self {
        use_context::<MenuManager>()
            .expect("MenuItem and MenuDropdown must be used within a Menu component")
    }
}
