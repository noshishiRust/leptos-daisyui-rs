use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;
use leptos_icons::Icon;

#[component]
pub fn AlertDemo() -> impl IntoView {
    view! {
        <ContentLayout
            title="Alert"
            description="Alerts are used to display important messages to users"
        >
            <Section title="Colors" col=true>
                <Alert color=AlertColor::Info>
                    <Icon icon=icondata::AiInfoCircleFilled />
                    <span>"Info alert - Something noteworthy"</span>
                </Alert>
                <Alert color=AlertColor::Success>
                    <Icon icon=icondata::AiCheckCircleFilled />
                    <span>"Success! Task completed"</span>
                </Alert>
                <Alert color=AlertColor::Warning>
                    <Icon icon=icondata::AiWarningFilled />
                    <span>"Warning - Please review"</span>
                </Alert>
                <Alert color=AlertColor::Error>
                    <Icon icon=icondata::AiCloseCircleFilled />
                    <span>"Error - Something went wrong"</span>
                </Alert>
            </Section>

            <Section title="Style Variants" col=true>
                <Alert style=AlertStyle::Default color=AlertColor::Info>
                    <span>"Default filled style"</span>
                </Alert>
                <Alert style=AlertStyle::Outline color=AlertColor::Success>
                    <span>"Outline style with border"</span>
                </Alert>
                <Alert style=AlertStyle::Dash color=AlertColor::Warning>
                    <span>"Dashed border style"</span>
                </Alert>
                <Alert style=AlertStyle::Soft color=AlertColor::Error>
                    <span>"Soft subtle background"</span>
                </Alert>
            </Section>

            <Section title="With Actions">
                <Alert color=AlertColor::Warning>
                    <Icon icon=icondata::AiWarningFilled />
                    <span>"New software update available"</span>
                    <div>
                        <button class="btn btn-sm">"Deny"</button>
                        <button class="btn btn-sm btn-primary">"Apply"</button>
                    </div>
                </Alert>
            </Section>
        </ContentLayout>
    }
}
