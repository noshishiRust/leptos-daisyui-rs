use crate::merge_classes;
use leptos::{html::Div, prelude::*};

/// Tree node data structure
#[derive(Clone, Debug)]
pub struct TreeNode {
    pub id: String,
    pub label: String,
    pub children: Vec<TreeNode>,
    pub is_expanded: bool,
}

/// # Tree Component
///
/// Hierarchical tree view for file explorers, org charts, and navigation.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("tree collapse collapse-arrow join join-vertical");
/// ```
#[component]
pub fn Tree(
    #[prop(into)] nodes: Signal<Vec<TreeNode>>,
    #[prop(optional, into)] on_select: Option<Callback<String>>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("tree", class)>
            <ul class="menu bg-base-200 rounded-box">
                {move || {
                    nodes
                        .get()
                        .into_iter()
                        .map(|node| {
                            view! { <TreeNodeView node=node on_select=on_select /> }
                        })
                        .collect_view()
                }}

            </ul>
        </div>
    }
}

#[component]
fn TreeNodeView(
    node: TreeNode,
    #[prop(optional, into)] on_select: Option<Callback<String>>,
) -> impl IntoView {
    let (is_expanded, set_is_expanded) = signal(node.is_expanded);
    let has_children = !node.children.is_empty();
    let node_id = node.id.clone();
    let label = node.label.clone();

    view! {
        <li>
            {if has_children {
                let children = node.children.clone();
                view! {
                    <details open=is_expanded>
                        <summary
                            on:click=move |_| {
                                set_is_expanded.update(|v| *v = !*v);
                                if let Some(ref callback) = on_select {
                                    callback.run(node_id.clone());
                                }
                            }
                        >

                            {label.clone()}
                        </summary>
                        <ul>
                            {children
                                .into_iter()
                                .map(|child| {
                                    view! { <TreeNodeView node=child on_select=on_select /> }
                                })
                                .collect_view()}

                        </ul>
                    </details>
                }
                    .into_any()
            } else {
                view! {
                    <a
                        on:click=move |_| {
                            if let Some(ref callback) = on_select {
                                callback.run(node_id.clone());
                            }
                        }
                    >

                        {label}
                    </a>
                }
                    .into_any()
            }}

        </li>
    }
}
