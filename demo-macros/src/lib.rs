mod markdown;

use heck::AsUpperCamelCase;
use quote::quote;
use std::fs::read_to_string;
use syn::{LitStr, parse_macro_input};

#[proc_macro]
pub fn create_demo_component(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(stream as LitStr);
    let component = input.value();

    let file = format!("../doc/components/{component}.md");
    let markdown = read_to_string(file).expect("Not found component file");
    let view_stream = markdown::markdown_to_token_stream(&markdown);
    let component_page = format!("{}Page", AsUpperCamelCase(component).to_string());

    quote! {
        #[component]
        pub fn #component_page() -> impl IntoView {
            #view_stream
        }
    }
    .into()
}
