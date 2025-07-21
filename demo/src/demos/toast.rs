use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn ToastDemo() -> impl IntoView {
    view! {
        <ContentLayout
            title="Toast"
            description="Toast notifications are used to show brief messages to users"
        >

            <Section title="Toast Examples">
                <div class="w-full h-64 relative border border-base-300">
                    <Toast class="absolute">
                        <div class="alert alert-info">
                            <span>New message arrived.</span>
                        </div>
                    </Toast>
                </div>
            </Section>
        </ContentLayout>
    }
}
