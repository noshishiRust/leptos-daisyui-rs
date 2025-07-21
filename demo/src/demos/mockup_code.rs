use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn MockupCodeDemo() -> impl IntoView {
    view! {
        <ContentLayout
            title="Mockup Code"
            description="Terminal or code editor frame for displaying code content"
        >
            <Section title="Basic Terminal">
                <MockupCode>
                    <MockupCodeLine prefix="$">"npm install leptos"</MockupCodeLine>
                    <MockupCodeLine prefix=">">"+ leptos@0.6.0"</MockupCodeLine>
                    <MockupCodeLine prefix=">">"added 1 package"</MockupCodeLine>
                    <MockupCodeLine prefix="$">"cargo leptos serve"</MockupCodeLine>
                    <MockupCodeLine prefix=">">
                        "Server running at http://localhost:3000"
                    </MockupCodeLine>
                </MockupCode>
            </Section>

            <Section title="Git Commands">
                <MockupCode>
                    <MockupCodeLine prefix="$">"git status"</MockupCodeLine>
                    <MockupCodeLine prefix=">">"On branch main"</MockupCodeLine>
                    <MockupCodeLine prefix=">">"Changes to be committed:"</MockupCodeLine>
                    <MockupCodeLine prefix=">">"  modified:   src/main.rs"</MockupCodeLine>
                    <MockupCodeLine prefix="$">"git commit -m \"Add new feature\""</MockupCodeLine>
                    <MockupCodeLine prefix=">">"[main 1234567] Add new feature"</MockupCodeLine>
                    <MockupCodeLine prefix=">">"1 file changed, 10 insertions(+)"</MockupCodeLine>
                </MockupCode>
            </Section>

            <Section title="Custom Prefix">
                <MockupCode>
                    <MockupCodeLine prefix="❯">"ls -la"</MockupCodeLine>
                    <MockupCodeLine prefix=" ">"total 48"</MockupCodeLine>
                    <MockupCodeLine prefix=" ">
                        "drwxr-xr-x  8 user  staff   256 Oct 20 10:30 ."
                    </MockupCodeLine>
                    <MockupCodeLine prefix=" ">
                        "drwxr-xr-x  3 user  staff    96 Oct 20 09:15 .."
                    </MockupCodeLine>
                    <MockupCodeLine prefix=" ">
                        "-rw-r--r--  1 user  staff  1234 Oct 20 10:30 Cargo.toml"
                    </MockupCodeLine>
                    <MockupCodeLine prefix=" ">
                        "-rw-r--r--  1 user  staff  5678 Oct 20 10:29 src/main.rs"
                    </MockupCodeLine>
                    <MockupCodeLine prefix="❯">"cargo run"</MockupCodeLine>
                    <MockupCodeLine prefix=" ">"   Compiling leptos-app v0.1.0"</MockupCodeLine>
                    <MockupCodeLine prefix=" ">
                        "    Finished dev [unoptimized + debuginfo] target(s)"
                    </MockupCodeLine>
                    <MockupCodeLine prefix=" ">
                        "     Running `target/debug/leptos-app`"
                    </MockupCodeLine>
                </MockupCode>
            </Section>

            <Section title="Code Compilation">
                <MockupCode>
                    <MockupCodeLine prefix="$">"cargo build --release"</MockupCodeLine>
                    <MockupCodeLine prefix=">">"   Compiling proc-macro2 v1.0.69"</MockupCodeLine>
                    <MockupCodeLine prefix=">">"   Compiling unicode-ident v1.0.12"</MockupCodeLine>
                    <MockupCodeLine prefix=">">"   Compiling syn v2.0.38"</MockupCodeLine>
                    <MockupCodeLine prefix=">">"   Compiling leptos v0.6.0"</MockupCodeLine>
                    <MockupCodeLine prefix=">">"   Compiling leptos-daisyui v0.1.0"</MockupCodeLine>
                    <MockupCodeLine prefix=">">
                        "    Finished release [optimized] target(s) in 2m 34s"
                    </MockupCodeLine>
                    <MockupCodeLine prefix="$">"./target/release/app"</MockupCodeLine>
                    <MockupCodeLine prefix=">">"🚀 Server started successfully!"</MockupCodeLine>
                </MockupCode>
            </Section>

            <Section title="Error Output">
                <MockupCode>
                    <MockupCodeLine prefix="$">"cargo check"</MockupCodeLine>
                    <MockupCodeLine prefix=">">"    Checking leptos-app v0.1.0"</MockupCodeLine>
                    <MockupCodeLine prefix="❌">"error[E0308]: mismatched types"</MockupCodeLine>
                    <MockupCodeLine prefix=" ">"  --> src/main.rs:42:5"</MockupCodeLine>
                    <MockupCodeLine prefix=" ">"   |"</MockupCodeLine>
                    <MockupCodeLine prefix=" ">"42 |     \"Hello, World!\""</MockupCodeLine>
                    <MockupCodeLine prefix=" ">
                        "   |     ^^^^^^^^^^^^^^^ expected `()`, found `&str`"
                    </MockupCodeLine>
                    <MockupCodeLine prefix="$">"# Fix the error and try again"</MockupCodeLine>
                </MockupCode>
            </Section>
        </ContentLayout>
    }
}