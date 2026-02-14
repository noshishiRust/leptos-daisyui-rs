//! # Paragraph Node Handler
//!
//! This module handles markdown paragraph nodes, converting them into styled `<p>` elements.
//!
//! ## Purpose
//!
//! Paragraphs are the most common element in documentation, providing descriptive text
//! for components, examples, and usage guidelines.
//!
//! ## Styling
//!
//! All paragraphs receive the class `text-base-content/70`, which applies:
//! - `text-base-content`: Uses the theme's base content color
//! - `/70`: 70% opacity for reduced visual weight
//!
//! This makes documentation text visually distinct from code and headings.
//!
//! ## Behavior
//!
//! 1. Extract text content from paragraph node
//! 2. **TODO**: Support inline formatting (bold, italic, code, links)
//! 3. Wrap in `<p>` with opacity class
//! 4. Include in generated view
//!
//! ## Limitations
//!
//! - **TODO**: Only plain text is supported (no inline formatting)
//! - **TODO**: Multiple text nodes in a paragraph are not handled
//! - **TODO**: Inline code, emphasis, strong text are not parsed
//! - **TODO**: Line breaks within paragraphs are lost

use comrak::nodes::{AstNode, NodeValue};
use proc_macro2::TokenStream;
use quote::quote;

/// Converts a markdown paragraph AST node into Leptos view tokens.
///
/// Paragraphs are rendered as `<p>` elements with reduced opacity for better
/// visual hierarchy in documentation.
///
/// # Arguments
///
/// * `node` - The AST node containing the paragraph and its children
///
/// # Returns
///
/// A `TokenStream` containing a `<p>` element with:
/// - `class="text-base-content/70"` for styling
/// - Text content from the paragraph
///
/// # Example
///
/// Given markdown:
/// ```markdown
/// A collapsible content component that allows users to expand sections.
/// ```
///
/// Generates:
/// ```ignore
/// quote! {
///     <p class="text-base-content/70">
///         A collapsible content component that allows users to expand sections.
///     </p>
/// }
/// ```
///
/// # Panics
///
/// - Panics if paragraph contains no children
/// - Panics if first child is not a text node
/// - **TODO**: Should handle these cases gracefully with empty/whitespace content
///
/// # Inline Elements
///
/// **TODO**: Currently only supports plain text. Many markdown inline elements
/// are not handled:
///
/// ```ignore
/// // Unsupported inline elements:
/// NodeValue::Code       // `inline code`
/// NodeValue::Emph       // *italic*
/// NodeValue::Strong      // **bold**
/// NodeValue::Link        // [text](url)
/// NodeValue::SoftBreak   // Soft line breaks
/// NodeValue::HardBreak   // Hard line breaks (trailing backslash)
/// ```
///
/// To support these, would need to iterate all children and compose them:
/// ```ignore
/// let inline_stream = node.children()
///     .map(|child| inline_token_stream(child))
///     .collect::<TokenStream>();
/// ```
pub fn token_stream_for_view<'a>(node: &'a AstNode<'a>) -> TokenStream {
    // Get first child node (should be text)
    // **TODO**: Handle multiple children and different node types
    let text_node = node.children().next().expect("Not Found Text in paragraph");

    // Extract the text content from the node
    let value = &text_node.data.borrow().value;
    let text = match value {
        NodeValue::Text(t) => t.clone(),
        // **TODO**: Handle other inline elements:
        // - NodeValue::Code: Render as <code class="...">
        // - NodeValue::Emph: Render as <em> or <span class="italic">
        // - NodeValue::Strong: Render as <strong> or <span class="font-bold">
        // - NodeValue::Link: Render as <a href="...">
        // - NodeValue::SoftBreak/HardBreak: Render as <br>
        _ => unimplemented!("Not Found Text"),
    };

    // **TODO**: Consider adding configurable paragraph classes via macro attributes:
    // #[demo_component(class = "custom-paragraph-class")]
    quote! {
        <p class="text-base-content/70">
            #text
        </p>
    }
}
