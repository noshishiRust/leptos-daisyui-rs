//! # Demo Macros
//!
//! This crate provides procedural macros for generating Leptos demo pages from markdown documentation.
//!
//! ## Overview
//!
//! The `create_demo_component!` macro reads a markdown file from `doc/components/{name}.md`,
//! parses it using the `comrak` CommonMark parser, and generates a Leptos component that
//! renders the parsed content as interactive UI elements.
//!
//! ## Supported Markdown Elements
//!
//! - **Headings** (h1-h3): Rendered with Tailwind CSS classes and auto-generated IDs
//! - **Paragraphs**: Rendered as `<p class="text-base-content/70">` elements
//! - **Code Blocks**: Parsed as Rust code and rendered directly (for live component examples)
//! - **Tables**: Converted to daisyUI Table components with zebra striping
//!
//! ## Unsupported Elements
//!
//! The following CommonMark elements are currently not supported and will be ignored:
//!
//! - **TODO**: Lists (ordered/unordered) - Need to add List/Item node handling
//! - **TODO**: Blockquotes - Need to add BlockQuote node handling
//! - **TODO**: Images - Need to add Image node handling with proper asset path resolution
//! - **TODO**: Links - Need to add Link node handling
//! - **TODO**: h4-h6 headings - Currently panics, should either render or skip gracefully
//! - **TODO**: Horizontal rules - Need to add Rule node handling
//!
//! ## Usage Example
//!
//! ```rust
//! use demo_macros::create_demo_component;
//!
//! create_demo_component!("accordion");
//! // Generates: pub fn AccordionPage() -> impl IntoView { ... }
//! ```
//!
//! ## Architecture
//!
//! 1. **Macro Entry Point**: `create_demo_component!` receives component name as string literal
//! 2. **File Reading**: Uses `CARGO_MANIFEST_DIR` to resolve an absolute path to `doc/components/{name}.md`
//! 3. **Parsing**: Uses `comrak` to parse markdown into an AST
//! 4. **Code Generation**: Each AST node type is converted to Leptos view code
//! 5. **Component Generation**: Wraps everything in a Leptos component function
//!
//! ## Limitations
//!
//! - **TODO**: Component naming assumes ASCII input - may break with non-ASCII component names
//! - **TODO**: No caching mechanism - re-parses markdown on every build

mod markdown;

use heck::AsUpperCamelCase;
use quote::{format_ident, quote};
use std::path::Path;
use syn::{LitStr, parse_macro_input};

/// Procedural macro that generates a Leptos component page from markdown documentation.
///
/// This macro reads a markdown file and generates a complete Leptos component that renders
/// the parsed content as interactive UI elements.
///
/// # Arguments
///
/// * `component_name` - A string literal specifying the component name (e.g., "accordion")
///
/// # Output
///
/// Generates a public Leptos component function named `{ComponentName}Page()` that returns `impl IntoView`.
///
/// # Example
///
/// ```rust
/// create_demo_component!("accordion");
/// // Expands to:
/// #[component]
/// pub fn AccordionPage() -> impl IntoView {
///     view! {
///         <!-- parsed markdown content -->
///     }
/// }
/// ```
///
/// # Errors
///
/// - Returns a compile error if the markdown file cannot be found at the resolved path
/// - Panics if the markdown contains unsupported syntax that causes parsing to fail
/// - Panics if code blocks contain invalid Rust syntax
///
/// # File Location
///
/// The file path is resolved using `CARGO_MANIFEST_DIR` environment variable.
/// Markdown files are expected at `{CARGO_MANIFEST_DIR}/../doc/components/{name}.md`.
#[proc_macro]
pub fn create_demo_component(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the macro input as a string literal (e.g., "accordion")
    let input = parse_macro_input!(stream as LitStr);
    let component = input.value();

    // Resolve path deterministically using CARGO_MANIFEST_DIR
    // demo-macros/ is at workspace root level, doc/ is one directory up
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR not set");
    let file = Path::new(&manifest_dir)
        .join("..")
        .join("doc")
        .join("components")
        .join(format!("{component}.md"));

    let markdown = match std::fs::read_to_string(&file) {
        Ok(content) => content,
        Err(e) => {
            let msg = format!(
                "failed to read component documentation file `{}`: {e}",
                file.display()
            );
            return syn::Error::new(input.span(), msg)
                .to_compile_error()
                .into();
        }
    };

    // Parse markdown and convert to Leptos view tokens
    let view_stream = markdown::markdown_to_token_stream(&markdown);

    // Generate component name in UpperCamelCase (e.g., "accordion" -> "AccordionPage")
    // **TODO**: This may not work correctly with non-ASCII characters or special cases.
    let component_page = format_ident!("{}Page", AsUpperCamelCase(component).to_string());

    // Generate the complete Leptos component
    quote! {
        #[component]
        pub fn #component_page() -> impl IntoView {
            view! {
                #view_stream
            }
        }
    }
    .into()
}
