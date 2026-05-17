//! # Code Block Node Handler
//!
//! This module handles markdown code blocks, parsing them as Rust code and rendering
//! them directly in the generated Leptos view.
//!
//! ## Purpose
//!
//! Code blocks are treated specially - instead of being displayed as text, they are
//! parsed as actual Rust code and rendered as live Leptos components. This enables
//! interactive examples in the demo application.
//!
//! ## Behavior
//!
//! 1. Check language identifier — only `""` (empty) and `"rust"` are accepted
//! 2. Non-Rust code blocks are silently skipped (empty token stream)
//! 3. Parse accepted blocks as Rust TokenStream
//! 4. On parse failure, emit `compile_error!` with the offending code and error details
//! 5. Wrap in `<div>` container
//!
//! ## Limitations
//!
//! - No syntax highlighting or code display mode (always executes)
//! - No line numbers or copy button functionality

use comrak::nodes::{AstNode, NodeCodeBlock};
use proc_macro2::TokenStream;
use quote::quote;

/// Converts a markdown code block AST node into Leptos view tokens.
///
/// Code blocks are treated as live Rust code rather than display text. The content
/// is parsed as a TokenStream and rendered directly, enabling interactive component
/// examples in the demo application.
///
/// # Arguments
///
/// * `_node` - The AST node containing the code block (unused but kept for consistency)
/// * `code_block_node` - The code block data containing literal code and metadata
///
/// # Returns
///
/// A `TokenStream` containing a `<div>` wrapper with the parsed code as children.
///
/// # Example
///
/// Given markdown:
/// ```markdown
/// ```rust
/// <Button>"Click me"</Button>
/// ```
/// ```
///
/// Generates:
/// ```ignore
/// quote! {
///     <div>
///         <Button>"Click me"</Button>
///     </div>
/// }
/// ```
///
/// # Panics
///
/// This function does not panic. Invalid Rust code blocks produce a `compile_error!`
/// at the macro invocation site with the offending code and parse error details.
///
/// # Language Identifier
///
/// The `info` field is validated to ensure only Rust code blocks are processed:
/// - `""` (empty) — accepted (indented or un-annotated fenced blocks)
/// - `"rust"` — accepted explicitly
/// - Any other value (e.g., `"html"`, `"css"`) — silently skipped (empty token stream)
pub fn token_stream_for_view<'a>(
    _node: &'a AstNode<'a>,
    code_block_node: &NodeCodeBlock,
) -> TokenStream {
    let info = code_block_node.info.clone();

    // Skip non-Rust code blocks silently. Accept both empty (indented or
    // un-annotated fenced blocks) and "rust" explicitly.
    if !info.is_empty() && info != "rust" {
        return quote! {};
    }

    let code = code_block_node.literal.clone();

    let code_raw_stream = match code.parse::<TokenStream>() {
        Ok(stream) => stream,
        Err(e) => {
            let msg = format!(
                "failed to parse Rust code block as valid TokenStream:\n--- code ---\n{}\n--- error ---\n{}",
                code, e
            );
            return quote! {
                compile_error!(#msg);
            };
        }
    };

    quote! {
        <div>
            #code_raw_stream
        </div>
    }
}
