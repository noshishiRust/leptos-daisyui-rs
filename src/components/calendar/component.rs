use crate::merge_classes;
use leptos::{
    html::{Button, Div, Table, Tbody, Td, Th, Thead, Tr},
    prelude::*,
};

#[component]
pub fn Calendar(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("calendar", class)>
            {children()}
        </div>
    }
}

#[component]
pub fn CalendarHeader(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("calendar-header", class)>
            {children()}
        </div>
    }
}

#[component]
pub fn CalendarBody(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Table>,
    children: Children,
) -> impl IntoView {
    view! {
        <table node_ref=node_ref class=move || merge_classes!("calendar-body", class)>
            {children()}
        </table>
    }
}

#[component]
pub fn CalendarWeekdays(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Thead>,
    children: Children,
) -> impl IntoView {
    view! {
        <thead node_ref=node_ref class=move || merge_classes!("calendar-weekdays", class)>
            {children()}
        </thead>
    }
}

#[component]
pub fn CalendarWeeks(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Tbody>,
    children: Children,
) -> impl IntoView {
    view! {
        <tbody node_ref=node_ref class=move || merge_classes!("calendar-weeks", class)>
            {children()}
        </tbody>
    }
}

#[component]
pub fn CalendarWeek(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Tr>,
    children: Children,
) -> impl IntoView {
    view! {
        <tr node_ref=node_ref class=move || merge_classes!("calendar-week", class)>
            {children()}
        </tr>
    }
}

#[component]
pub fn CalendarWeekday(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Th>,
    children: Children,
) -> impl IntoView {
    view! {
        <th node_ref=node_ref class=move || merge_classes!("calendar-weekday", class)>
            {children()}
        </th>
    }
}

#[component]
pub fn CalendarDate(
    #[prop(optional, into)] selected: Signal<bool>,
    #[prop(optional, into)] today: Signal<bool>,
    #[prop(optional, into)] outside_month: Signal<bool>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Td>,
    #[prop(optional)] on_click: Option<Box<dyn Fn()>>,
    children: Children,
) -> impl IntoView {
    view! {
        <td
            node_ref=node_ref
            class=move || merge_classes!("calendar-date", class)
            class:calendar-date-selected=selected
            class:calendar-date-today=today
            class:calendar-date-outside=outside_month
            on:click=move |_| {
                if let Some(handler) = &on_click {
                    handler();
                }
            }
        >
            {children()}
        </td>
    }
}

#[component]
pub fn CalendarNavigation(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("calendar-nav", class)>
            {children()}
        </div>
    }
}

#[component]
pub fn CalendarButton(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Button>,
    #[prop(optional)] on_click: Option<Box<dyn Fn()>>,
    children: Children,
) -> impl IntoView {
    view! {
        <button
            node_ref=node_ref
            class=move || merge_classes!("btn", "btn-sm", class)
            on:click=move |_| {
                if let Some(handler) = &on_click {
                    handler();
                }
            }
        >
            {children()}
        </button>
    }
}
