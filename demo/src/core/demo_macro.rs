use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ExprLit, Lit};

/// Macro to render a demo section with preview and code
///
/// Usage:
/// ```rust
/// demo_section! {
///     title: "Colors",
///     id: "colors",
///     row: true,
///     code: r#"
///         <Button>"Default"</Button>
///         <Button color=ButtonColor::Primary>"Primary"</Button>
///     "#,
///     preview: view! {
///         <Button>"Default"</Button>
///         <Button color=ButtonColor::Primary>"Primary"</Button>
///     }
/// }
/// ```
#[proc_macro]
pub fn demo_section(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();

    // Parse the input - this is a simplified version
    // In production, you'd want to use syn to properly parse this
    let (title, id, row, col, code, preview) = parse_demo_section(&input_str);

    let expanded = quote! {
        {
            use leptos::prelude::*;
            use crate::core::CodeBlock;

            let (code_visible, set_code_visible) = ::leptos::prelude::signal(false);

            view! {
                <div id=#id class="demo-section scroll-mt-24 mb-8">
                    <div class="flex items-center gap-2 mb-3">
                        <h2 class="text-xl font-semibold">#title</h2>
                        <button
                            on:click=move |_| set_code_visible.update(|v| *v = !v)
                            class="btn btn-ghost btn-xs ml-auto gap-1"
                        >
                            <span class="text-sm">{move || if code_visible.get() { "Hide Code" } else { "Show Code" }}</span>
                        </button>
                    </div>

                    // Preview container
                    <div class="demo-preview bg-base-200 border border-base-300 rounded-lg p-4 mb-2">
                        <div class="flex gap-2" class:flex-col=#col class:flex-row=#row>
                            #preview
                        </div>
                    </div>

                    // Code block (collapsible)
                    <div class=move || if code_visible.get() { "" } else { "hidden" }>
                        <CodeBlock code=#code language="rust" title=format!("{} Code", #title).to_string() />
                    </div>
                </div>
            }
        }
    };

    TokenStream::from(expanded)
}

fn parse_demo_section(input: &str) -> (proc_macro2::Ident, proc_macro2::Ident, bool, bool, proc_macro2::Literal, proc_macro2::TokenStream) {
    // Simplified parsing - in production, use proper syn parsing
    // For now, return placeholder values
    let title = proc_macro2::Ident::new("Demo", proc_macro2::Span::call_site());
    let id = proc_macro2::Ident::new("demo", proc_macro2::Span::call_site());
    let row = true;
    let col = false;
    let code = proc_macro2::Literal::string("");
    let preview = proc_macro2::TokenStream::new();

    (title, id, row, col, code, preview)
}
