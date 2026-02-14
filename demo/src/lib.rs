pub mod core;
pub mod demos;

use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

// create components by macro for demos
demo_macros::create_demo_component! { "accordion" }
