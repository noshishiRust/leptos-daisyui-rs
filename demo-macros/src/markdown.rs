//! # Markdown to TokenStream Converter
//!
//! This module provides the core parsing functionality that converts markdown AST nodes
//! into Leptos view code tokens.
//!
//! ## Architecture
//!
//! 1. **Parsing**: Uses `comrak` to parse markdown text into an AST
//! 2. **Traversal**: Iterates over top-level AST nodes
//! 3. **Conversion**: Each node type is dispatched to a specialized handler module
//! 4. **Composition**: Results are composed into a single TokenStream
//!
//! ## Supported Node Types
//!
//! - `Heading` → [`heading`] module → `<h1>`/`<h2>`/`<h3>` with classes and IDs
//! - `Paragraph` → [`paragraph`] module → `<p>` with opacity styling
//! - `CodeBlock` → [`code_block`] module → Raw Rust code wrapped in `<div>`
//! - `Table` → [`table`] module → daisyUI `<Table>` component
//!
//! ## Unsupported Node Types
//!
//! The following CommonMark node types are intentionally not supported:
//!
//! - **TODO**: `List` (ordered/unordered) - High priority for FAQ/features lists
//! - **TODO**: `Item` (list items) - Required for list support
//! - **TODO**: `BlockQuote` - Useful for callouts and notes
//! - **TODO**: `Image` - Requires asset path resolution strategy
//! - **TODO**: `Link` - Important for cross-references
//! - **TODO**: `Rule` (horizontal rules) - Simple to add, low priority
//! - **TODO**: `HtmlBlock` - Security implications, needs sanitization
//! - **TODO**: h4-h6 `Heading` levels - Currently panics, needs graceful handling
//!
//! ## Extension Points
//!
//! To add support for a new node type:
//!
//! 1. Create a new module in `markdown/` (e.g., `list.rs`)
//! 2. Implement `token_stream_for_view()` function
//! 3. Add match arm in `markdown_to_token_stream()`
//! 4. Export module in `mod.rs`
//!
//! # Example
//!
//! ```ignore
//! // Adding list support:
//! match &node.data.borrow().value {
//!     NodeValue::List(list) => list::token_stream_for_view(node, list),
//!     // ... other nodes
//! }
//! ```

mod code_block;
mod heading;
mod paragraph;
mod table;

use comrak::nodes::NodeValue;
use comrak::{Arena, Options, parse_document};
use proc_macro2::TokenStream;
use quote::quote;

/// Converts markdown text into Leptos view code tokens.
///
/// This is the main entry point for markdown parsing. It takes a string containing
/// markdown, parses it into an AST using `comrak`, and converts each supported
/// node type into corresponding Leptos view code.
///
/// # Arguments
///
/// * `markdown` - A string slice containing markdown source code
///
/// # Returns
///
/// A `TokenStream` containing Leptos view macros that will render the markdown content.
///
/// # Example
///
/// ```ignore
/// let markdown = "# Title\n\nSome text";
/// let tokens = markdown_to_token_stream(markdown);
/// // tokens contains: <h1 class="text-3xl font-bold" id="title">Title</h1> ...
/// ```
///
/// # Panics
///
/// - **TODO**: Panics with `unimplemented!()` if encountering h4-h6 headings
/// - **TODO**: Should return `Result<TokenStream, Error>` instead of panicking
/// - Silently ignores unsupported node types (generates empty TokenStream)
///
/// # Supported Extensions
///
/// Currently enables `comrak`'s table extension. Other extensions could be added:
///
/// ```ignore
/// options.extension.strikethrough = true;
/// options.extension.tasklist = true;
/// ```
///
/// **TODO**: Make extension support configurable via macro attributes.
pub fn markdown_to_token_stream(markdown: &str) -> TokenStream {
    let mut view_token_stream = TokenStream::new();

    // Create an Arena for AST node lifetime management.
    // comrak uses arenas for memory efficiency - all nodes live for the same lifetime.
    let arena = Arena::new();

    // Configure comrak parser options
    let mut options = Options::default();
    options.extension.table = true; // Enable GitHub-style tables

    // **TODO**: Consider adding more extensions:
    // - options.extension.strikethrough = true;  // ~~text~~
    // - options.extension.tasklist = true;        // - [x] tasks
    // - options.extension.superscript = true;      // ^superscript^

    // Parse markdown into AST
    let root_node = parse_document(&arena, markdown, &options);

    // Iterate over top-level nodes and convert each to view tokens
    root_node.children().into_iter().for_each(|node| {
        let token = match &node.data.borrow().value {
            // Dispatch to specialized handlers based on node type
            NodeValue::Heading(h) => heading::token_stream_for_view(node, h),
            NodeValue::Paragraph => paragraph::token_stream_for_view(node),
            NodeValue::Table(_) => table::token_stream_for_view(node),
            NodeValue::CodeBlock(c) => code_block::token_stream_for_view(node, c),

            // **TODO**: Add handlers for unsupported node types:
            // NodeValue::List(l) => list::token_stream_for_view(node, l),
            // NodeValue::Item => list::item_token_stream_for_view(node),
            // NodeValue::BlockQuote => blockquote::token_stream_for_view(node),
            // NodeValue::Image(image) => image::token_stream_for_view(node, image),
            // NodeValue::Link(link) => link::token_stream_for_view(node, link),
            // NodeValue::Rule => hr::token_stream_for_view(node),

            // Silently ignore unsupported nodes by generating empty tokens
            // **TODO**: This should probably log a warning at compile time
            _ => quote! {}
        };

        view_token_stream.extend(token);
    });

    view_token_stream
}
