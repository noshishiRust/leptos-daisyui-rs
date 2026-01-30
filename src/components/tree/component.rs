use leptos::{attr::Attribute, html, prelude::*};

/// A node in the tree structure
#[derive(Clone, Debug)]
pub struct TreeNode {
    /// Unique identifier for the node
    pub id: String,
    /// Display label for the node
    pub label: String,
    /// Child nodes
    pub children: Vec<TreeNode>,
    /// Whether the node is expanded
    pub is_expanded: bool,
}

/// A hierarchical tree view component.
///
/// # Example
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     let nodes = vec![
///         TreeNode {
///             id: "1".to_string(),
///             label: "Root".to_string(),
///             is_expanded: true,
///             children: vec![
///                 TreeNode {
///                     id: "1.1".to_string(),
///                     label: "Child 1".to_string(),
///                     is_expanded: false,
///                     children: vec![],
///                 },
///             ],
///         },
///     ];
///
///     view! {
///         <Tree
///             nodes=nodes
///             on_select=move |id| { leptos::logging::log!("Selected: {}", id); }
///         />
///     }
/// }
/// ```
///
/// # CSS Classes
/// Add to your `input.css`:
/// ```css
/// @source inline("pl-4 cursor-pointer hover:bg-base-200 p-2 rounded");
/// ```
#[component]
pub fn Tree(
    /// Tree nodes
    #[prop(into)]
    nodes: Signal<Vec<TreeNode>>,
    /// Callback when a node is selected
    #[prop(optional)]
    on_select: Option<Callback<String>>,
    /// Reference to the underlying DOM node. See [MDN docs](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/ul)
    #[prop(optional)]
    node_ref: NodeRef<html::Ul>,
    /// Spread additional attributes onto the `<ul>` element
    #[prop(attrs)]
    attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <ul
            node_ref=node_ref
            class="tree"
            {..attributes}
        >
            <For
                each=move || nodes.get()
                key=|node| node.id.clone()
                children=move |node| {
                    view! {
                        <TreeNodeView node=node on_select=on_select />
                    }
                }
            />
        </ul>
    }
}

#[component]
fn TreeNodeView(
    node: TreeNode,
    #[prop(optional)]
    on_select: Option<Callback<String>>,
) -> impl IntoView {
    let (is_expanded, set_is_expanded) = signal(node.is_expanded);
    let has_children = !node.children.is_empty();
    let node_id = node.id.clone();
    let node_id_for_click = node.id.clone();
    let children = node.children.clone();

    view! {
        <li>
            <div
                class="cursor-pointer hover:bg-base-200 p-2 rounded flex items-center gap-2"
                on:click=move |_| {
                    if has_children {
                        set_is_expanded.update(|v| *v = !*v);
                    }
                    if let Some(callback) = on_select {
                        callback.run(node_id_for_click.clone());
                    }
                }
            >
                <Show when=move || has_children>
                    <span class="text-xs">
                        {move || if is_expanded.get() { "▼" } else { "▶" }}
                    </span>
                </Show>
                <span>{node.label.clone()}</span>
            </div>
            <Show when=move || has_children && is_expanded.get()>
                <ul class="pl-4">
                    <For
                        each=move || children.clone()
                        key=|child| child.id.clone()
                        children=move |child| {
                            view! {
                                <TreeNodeView node=child on_select=on_select />
                            }
                        }
                    />
                </ul>
            </Show>
        </li>
    }
}
