//! # Heading Node Handler
//!
//! This module handles markdown heading nodes (h1, h2, h3), converting them
//! into HTML heading elements with Tailwind CSS classes and auto-generated IDs.
//!
//! ## Supported Levels
//!
//! - **h1** (`# Title`): Largest heading, used for component title
//! - **h2** (`## Section`): Medium heading, used for main sections
//! - **h3** (`### Subsection`): Smallest supported heading, used for examples/subsections
//! - **h4-h6**: **TODO**: Not supported, will panic with `unimplemented!()`
//!
//! ## ID Generation
//!
//! Heading IDs are automatically generated from the heading text for anchor links:
//!
//! 1. Convert to lowercase
//! 2. Replace spaces and special characters with hyphens
//! 3. Remove consecutive hyphens
//! 4. Truncate to 50 characters
//!
//! Examples:
//! - "Basic Usage" → `basic-usage`
//! - "Props Table" → `props-table`
//! - "What's the difference?" → `whats-the-difference`
//!
//! ## Styling Classes
//!
//! Each heading level receives specific Tailwind classes:
//!
//! - h1: `text-3xl font-bold`
//! - h2: `text-xl font-semibold`
//! - h3: `text-lg font-medium`
//!
//! **TODO**: Make classes configurable or support custom class additions
//!
//! ## Limitations
//!
//! - Only ASCII headings are supported (non-ASCII is filtered)
//! - ID generation is simplistic (no collision detection)
//! - h4-h6 will cause panic
//! - No support for heading attributes or custom IDs

use comrak::nodes::{AstNode, NodeHeading, NodeValue};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

/// Converts a markdown heading AST node into Leptos view tokens.
///
/// Headings are rendered as HTML `<h1>`, `<h2>`, or `<h3>` elements with
/// Tailwind CSS classes and auto-generated IDs for anchor linking.
///
/// # Arguments
///
/// * `node` - The AST node containing the heading and its children
/// * `heading_node` - The heading data containing level (1-6) and metadata
///
/// # Returns
///
/// A `TokenStream` containing a heading element with:
/// - Appropriate tag (`h1`, `h2`, or `h3`)
/// - Tailwind CSS class for styling
/// - Auto-generated ID for anchor links
/// - Heading text content
///
/// # Example
///
/// Given markdown:
/// ```markdown
/// ### Basic Usage
/// ```
///
/// Generates:
/// ```ignore
/// quote! {
///     <h3 class="text-lg font-medium" id="basic-usage">
///         Basic Usage
///     </h3>
/// }
/// ```
///
/// # Panics
///
/// - **TODO**: Panics with `unimplemented!()` for h4-h6 headings
/// - Should either:
///   1. Support h4-h6 with appropriate styling
///   2. Skip deeper levels with a compile warning
///   3. Fall back to a generic heading style
pub fn token_stream_for_view<'a>(node: &'a AstNode<'a>, heading_node: &NodeHeading) -> TokenStream {
    // Extract heading level (1-6)
    let level = heading_node.level;

    // **TODO**: Add heading level validation and graceful handling:
    // if level > 3 {
    //     eprintln!("Warning: h{level} is not supported, falling back to h3 styling");
    //     let level = 3;
    // }

    // Get first child node (should be text)
    let text_node = node.children().next().expect("Not Found Text in heading");

    // Extract the text content from the node
    let value = &text_node.data.borrow().value;
    let text = match value {
        NodeValue::Text(t) => t.clone(),
        // **TODO**: Handle other inline elements in headings:
        // - NodeValue::Code (inline code in headings)
        // - NodeValue::Emph (italicized text)
        // - NodeValue::Strong (bold text)
        _ => unimplemented!("Not Found Text"),
    };

    // Generate ID from heading text for anchor links
    let h_id = generate_id(&text);

    // Create HTML tag identifier (h1, h2, h3)
    let h_tag = format_ident!("h{level}");

    // Get CSS class for this heading level
    let h_class = heading_class(level);

    quote! {
        <#h_tag class=#h_class id=#h_id>
            #text
        </#h_tag>
    }
}

/// Generates a URL-safe ID from heading text.
///
/// The ID generation process:
///
/// 1. Filter each character:
///    - Alphanumeric: keep as lowercase
///    - Whitespace/hyphen: convert to hyphen
///    - Everything else: remove
/// 2. Split on hyphens and filter empty segments
/// 3. Join with single hyphens
/// 4. Limit to 50 characters
///
/// # Arguments
///
/// * `text` - The heading text to convert
///
/// # Returns
///
/// A URL-safe ID string.
///
/// # Examples
///
/// ```ignore
/// assert_eq!(generate_id("Basic Usage"), "basic-usage");
/// assert_eq!(generate_id("What's the difference?"), "whats-the-difference");
/// assert_eq!(generate_id("Hello---World"), "hello-world");
/// assert_eq!(generate_id("A"*50), "a".repeat(50)); // Truncates at 50
/// ```
///
/// # Limitations
///
/// - **TODO**: No collision detection (duplicate IDs will be generated)
/// - **TODO**: Non-ASCII characters are silently dropped
/// - **TODO**: Numbers-only IDs are technically valid but might be problematic
fn generate_id(text: &str) -> String {
    text.chars()
        .filter_map(|c| {
            if c.is_alphanumeric() {
                // Keep alphanumeric characters as lowercase
                Some(c.to_ascii_lowercase())
            } else if c.is_whitespace() || c == '-' {
                // Convert whitespace and hyphens to hyphens
                Some('-')
            } else {
                // Remove everything else (punctuation, symbols, etc.)
                None
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join("-")
        .chars()
        .take(50) // **TODO**: This truncation might split words mid-character
        .collect()
}

/// Returns the Tailwind CSS class string for a given heading level.
///
/// # Arguments
///
/// * `level` - The heading level (1-6)
///
/// # Returns
///
/// A CSS class string for the heading.
///
/// # Panics
///
/// - **TODO**: Panics with `unimplemented!()` for levels 4, 5, 6
/// - Should either:
///   1. Add h4: `text-base font-normal` etc.
///   2. Fall back to h3 styling
///   3. Return empty string for deeper levels
fn heading_class(level: u8) -> String {
    match level {
        1 => "text-3xl font-bold".into(),
        2 => "text-xl font-semibold".into(),
        3 => "text-lg font-medium".into(),
        // **TODO**: Add support for h4-h6:
        // 4 => "text-base font-normal".into(),
        // 5 => "text-sm font-normal".into(),
        // 6 => "text-xs font-normal".into(),
        _ => {
            // This is hit for h4, h5, h6
            // **TODO**: Instead of panicking, should:
            // - eprintln!("Warning: h{level} not supported, using h3 styling");
            // - return "text-lg font-medium".into();
            unimplemented!()
        }
    }
}
