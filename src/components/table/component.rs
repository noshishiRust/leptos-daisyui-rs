use super::style::TableSize;
use crate::merge_classes;
use leptos::{html::{Table as HtmlTable, Thead, Tbody, Tfoot, Tr, Th, Td}, prelude::*};

#[component]
pub fn Table(
    #[prop(optional, into)] size: Signal<TableSize>,
    #[prop(optional, into)] zebra: Signal<bool>,
    #[prop(optional, into)] pin_rows: Signal<bool>,
    #[prop(optional, into)] pin_cols: Signal<bool>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<HtmlTable>,
    children: Children,
) -> impl IntoView {
    view! {
        <table
            node_ref=node_ref
            class=merge_classes!(
                "table",
                size.get().as_str(),
                class
            )
            class:table-zebra=zebra
            class:table-pin-rows=pin_rows
            class:table-pin-cols=pin_cols
        >
            {children()}
        </table>
    }
}

#[component]
pub fn TableHead(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Thead>,
    children: Children,
) -> impl IntoView {
    view! {
        <thead node_ref=node_ref class=merge_classes!("", class)>
            {children()}
        </thead>
    }
}

#[component]
pub fn TableBody(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Tbody>,
    children: Children,
) -> impl IntoView {
    view! {
        <tbody node_ref=node_ref class=merge_classes!("", class)>
            {children()}
        </tbody>
    }
}

#[component]
pub fn TableFoot(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Tfoot>,
    children: Children,
) -> impl IntoView {
    view! {
        <tfoot node_ref=node_ref class=merge_classes!("", class)>
            {children()}
        </tfoot>
    }
}

#[component]
pub fn TableRow(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Tr>,
    children: Children,
) -> impl IntoView {
    view! {
        <tr node_ref=node_ref class=merge_classes!("", class)>
            {children()}
        </tr>
    }
}

#[component]
pub fn TableHeader(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Th>,
    children: Children,
) -> impl IntoView {
    view! {
        <th node_ref=node_ref class=merge_classes!("", class)>
            {children()}
        </th>
    }
}

#[component]
pub fn TableCell(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Td>,
    children: Children,
) -> impl IntoView {
    view! {
        <td node_ref=node_ref class=merge_classes!("", class)>
            {children()}
        </td>
    }
}