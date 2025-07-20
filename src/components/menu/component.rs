//! Menu component implementations for navigation structures.

use super::style::{MenuDirection, MenuSize};
use crate::merge_classes;
use leptos::{
    html::{H2, Li, Ul},
    prelude::*,
};

/// A navigation menu container component.
///
/// Provides a structured navigation menu with configurable direction, size, and selection state.
/// Can operate in automatic selection mode (tracks selected items) or manual mode for custom behavior.
///
/// # Props
///
/// - `manual` - If `true`, disables automatic selection tracking (default: `false`)
/// - `selected` - Signal for tracking the currently selected menu item value
/// - `direction` - Layout direction: vertical (default) or horizontal
/// - `size` - Size variant: xs, sm, md (default), lg, xl
/// - `class` - Additional CSS classes to apply
/// - `node_ref` - Reference to the underlying `<ul>` element
/// - `children` - Menu content (typically `MenuItem`, `MenuTitle`, `SubMenu` components)
///
/// # Context
///
/// Provides `MenuManager` context to child components for selection state management.
///
/// # Examples
///
/// ## Basic Usage
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::{Menu, MenuItem};
///
/// #[component]
/// fn Navigation() -> impl IntoView {
///     let selected = RwSignal::new(Some("home".to_string()));
///
///     view! {
///         <Menu selected=selected>
///             <MenuItem value="home".to_string()>"Home"</MenuItem>
///             <MenuItem value="about".to_string()>"About"</MenuItem>
///             <MenuItem value="contact".to_string()>"Contact"</MenuItem>
///         </Menu>
///     }
/// }
/// ```
///
/// ## Horizontal Menu
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::{Menu, MenuItem, MenuDirection};
///
/// view! {
///     <Menu direction=MenuDirection::Horizontal>
///         <MenuItem value="nav1".to_string()>"Item 1"</MenuItem>
///         <MenuItem value="nav2".to_string()>"Item 2"</MenuItem>
///     </Menu>
/// }
/// ```
///
/// ## Manual Selection Mode
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::{Menu, MenuItem};
///
/// view! {
///     <Menu manual=true>
///         <MenuItem active=Signal::derive(|| true)>"Always Active"</MenuItem>
///         <MenuItem>"Normal Item"</MenuItem>
///     </Menu>
/// }
/// ```
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

/// An individual menu item component.
///
/// Represents a single selectable item within a menu. Supports navigation links,
/// selection state tracking, and disabled state. Can be used in regular menus or submenus.
///
/// # Props
///
/// - `href` - Optional URL for navigation when clicked
/// - `value` - Unique identifier for this menu item (used for selection tracking)
/// - `active` - Manual active state (only used when parent Menu has `manual=true`)
/// - `disabled` - Whether the item is disabled and non-interactive
/// - `class` - Additional CSS classes to apply
/// - `node_ref` - Reference to the underlying `<li>` element
/// - `is_submenu` - If `true`, renders without anchor wrapper for submenu containers
/// - `children` - Item content (text, icons, etc.)
///
/// # Behavior
///
/// - Automatically updates parent Menu's selected state when clicked (unless disabled)
/// - Active state determined by parent Menu's selection (unless in manual mode)
/// - Disabled items ignore click events
/// - Submenu items render content directly without anchor wrapper
///
/// # Examples
///
/// ## Basic Menu Item
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::MenuItem;
///
/// view! {
///     <MenuItem value="profile".to_string() href="/profile".to_string()>
///         "Profile Settings"
///     </MenuItem>
/// }
/// ```
///
/// ## Disabled Item
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::MenuItem;
///
/// view! {
///     <MenuItem value="admin".to_string() disabled=Signal::derive(|| true)>
///         "Admin Panel (Disabled)"
///     </MenuItem>
/// }
/// ```
///
/// ## Submenu Container
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::{MenuItem, SubMenu};
///
/// view! {
///     <MenuItem is_submenu=true>
///         "Settings"
///         <SubMenu>
///             <MenuItem value="profile".to_string()>"Profile"</MenuItem>
///             <MenuItem value="account".to_string()>"Account"</MenuItem>
///         </SubMenu>
///     </MenuItem>
/// }
/// ```
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

/// A section title component for organizing menu items.
///
/// Provides semantic sectioning within menus to group related items.
/// Typically used to create visual and logical separation between menu sections.
///
/// # Props
///
/// - `class` - Additional CSS classes to apply
/// - `node_ref` - Reference to the underlying `<h2>` element
/// - `children` - Title text content
///
/// # Examples
///
/// ## Section Organization
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::{Menu, MenuItem, MenuTitle};
///
/// view! {
///     <Menu>
///         <MenuItem value="home".to_string()>"Home"</MenuItem>
///         <MenuItem value="about".to_string()>"About"</MenuItem>
///         
///         <MenuTitle>"Account"</MenuTitle>
///         <MenuItem value="profile".to_string()>"Profile"</MenuItem>
///         <MenuItem value="settings".to_string()>"Settings"</MenuItem>
///         
///         <MenuTitle>"Admin"</MenuTitle>
///         <MenuItem value="users".to_string()>"Manage Users"</MenuItem>
///         <MenuItem value="reports".to_string()>"Reports"</MenuItem>
///     </Menu>
/// }
/// ```
///
/// ## Custom Styling
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::MenuTitle;
///
/// view! {
///     <MenuTitle class="text-primary font-bold">
///         "Important Section"
///     </MenuTitle>
/// }
/// ```
#[component]
pub fn MenuTitle(
    /// Additional CSS classes
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

/// A nested submenu container component.
///
/// Creates a nested menu structure within a parent MenuItem. Used for hierarchical
/// navigation where items need to be organized in a tree-like structure.
///
/// # Props
///
/// - `class` - Additional CSS classes to apply
/// - `node_ref` - Reference to the underlying `<ul>` element
/// - `children` - Submenu content (typically more MenuItem components)
///
/// # Usage
///
/// Should be used inside a MenuItem with `is_submenu=true` to create proper nesting.
///
/// # Examples
///
/// ## Nested Menu Structure
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::{Menu, MenuItem, SubMenu};
///
/// view! {
///     <Menu>
///         <MenuItem value="home".to_string()>"Home"</MenuItem>
///         
///         <MenuItem is_submenu=true>
///             "Settings"
///             <SubMenu>
///                 <MenuItem value="profile".to_string()>"Profile"</MenuItem>
///                 <MenuItem value="account".to_string()>"Account"</MenuItem>
///                 <MenuItem value="privacy".to_string()>"Privacy"</MenuItem>
///             </SubMenu>
///         </MenuItem>
///         
///         <MenuItem is_submenu=true>
///             "Tools"
///             <SubMenu>
///                 <MenuItem value="export".to_string()>"Export Data"</MenuItem>
///                 <MenuItem value="import".to_string()>"Import Data"</MenuItem>
///             </SubMenu>
///         </MenuItem>
///     </Menu>
/// }
/// ```
///
/// ## Deep Nesting
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::{MenuItem, SubMenu};
///
/// view! {
///     <MenuItem is_submenu=true>
///         "Advanced"
///         <SubMenu>
///             <MenuItem value="config".to_string()>"Configuration"</MenuItem>
///             
///             <MenuItem is_submenu=true>
///                 "Developer Tools"
///                 <SubMenu>
///                     <MenuItem value="debug".to_string()>"Debug Mode"</MenuItem>
///                     <MenuItem value="console".to_string()>"Console"</MenuItem>
///                 </SubMenu>
///             </MenuItem>
///         </SubMenu>
///     </MenuItem>
/// }
/// ```
#[component]
pub fn SubMenu(
    /// Additional CSS classes
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
///
/// Provides centralized state management for menu item selection and behavior modes.
/// This context is automatically provided by the Menu component and consumed by MenuItem components.
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
    ///
    /// Panics if called outside of a Menu component context.
    pub fn expect_context() -> Self {
        expect_context()
    }
}
