pub mod core;
pub mod demos;

use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

// create components by macro for demos
demo_macros::create_demo_component! { "accordion" }
demo_macros::create_demo_component! { "alert" }
demo_macros::create_demo_component! { "avatar" }
demo_macros::create_demo_component! { "badge" }
demo_macros::create_demo_component! { "breadcrumbs" }
demo_macros::create_demo_component! { "button" }
demo_macros::create_demo_component! { "card" }
demo_macros::create_demo_component! { "carousel" }
demo_macros::create_demo_component! { "chat" }
demo_macros::create_demo_component! { "checkbox" }
demo_macros::create_demo_component! { "collapse" }
demo_macros::create_demo_component! { "countdown" }
demo_macros::create_demo_component! { "diff" }
demo_macros::create_demo_component! { "divider" }
