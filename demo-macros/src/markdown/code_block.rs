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
//! 1. Extract code block literal content
//! 2. **TODO**: Check for language identifier (e.g., \`\`\`rust)
//! 3. Parse as Rust TokenStream
//! 4. Wrap in `<div>` container
//! 5. Include in generated view
//!
//! ## Limitations
//!
//! - **TODO**: Language identifier (`info`) is parsed but not used or validated
//! - **TODO**: No syntax highlighting or code display mode (always executes)
//! - **TODO**: No error context if parsing fails - just panics
//! - **TODO**: Should support non-Rust code blocks with proper escaping/display

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
/// - **TODO**: Panics if code block contains invalid Rust syntax
/// - Should provide better error messages showing which line/column failed
///
/// # Language Identifier
///
/// **TODO**: The `info` field contains the language identifier (e.g., "rust", "javascript").
/// This is currently extracted but not validated or used. Future improvements:
///
/// ```ignore
/// let info = code_block_node.info.clone();
/// if !info.is_empty() && info != "rust" {
///     // Either escape for display or emit compile-time warning
///     compile_error!("Only Rust code blocks are supported");
/// }
/// ```
pub fn token_stream_for_view<'a>(
    _node: &'a AstNode<'a>,
    code_block_node: &NodeCodeBlock,
) -> TokenStream {
    // Extract language identifier (e.g., "rust", "js", "toml")
    // **TODO**: This is currently extracted but not used. Should:
    // 1. Validate that it's "rust" (or empty/default)
    // 2. Emit compile warning if non-rust code is found
    // 3. Consider supporting other languages with proper escaping
    let _info = code_block_node.info.clone();

    // Extract the actual code content
    let code = code_block_node.literal.clone();

    // **TODO**: Add better error handling for parse failures
    // Current panic message is not helpful:
    //   "Failed to convert stream from string"
    // Better:
    //   "Failed to parse Rust code in markdown: {code}\nError: {error}"
    let code_raw_stream = code.parse::<TokenStream>().expect("Failed to convert stream from string");

    // Wrap code in a div for container
    // **TODO**: Consider adding classes for styling:
    // - class="demo-code-block" for styling hooks
    // - Add line numbers option
    // - Add copy button functionality
    quote! {
        <div>
            #code_raw_stream
        </div>
    }
}
