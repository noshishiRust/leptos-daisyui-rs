use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;
use leptos_icons::Icon;

#[component]
pub fn BreadcrumbsDemo() -> impl IntoView {
    view! {
        <ContentLayout
            title="Breadcrumbs"
            description="Breadcrumbs help users navigate and understand their current location"
        >
            <Section title="Basic Usage">
                <Breadcrumbs>
                    <BreadcrumbItem href="#".to_string()>"Home"</BreadcrumbItem>
                    <BreadcrumbItem href="#".to_string()>"Documents"</BreadcrumbItem>
                    <BreadcrumbItem class="font-bold">"Current Page"</BreadcrumbItem>
                </Breadcrumbs>
            </Section>

            <Section title="With Icons">
                <Breadcrumbs>
                    <BreadcrumbItem href="#".to_string()>
                        <div class="flex items-center gap-1">
                            <Icon icon=icondata::AiHomeFilled width="16" height="16" />
                            "Home"
                        </div>
                    </BreadcrumbItem>
                    <BreadcrumbItem href="#".to_string()>
                        <div class="flex items-center gap-1">
                            <Icon icon=icondata::AiFolderFilled width="16" height="16" />
                            "Documents"
                        </div>
                    </BreadcrumbItem>
                    <BreadcrumbItem>
                        <div class="flex items-center gap-1">
                            <Icon icon=icondata::AiFileTextFilled width="16" height="16" />
                            "File.pdf"
                        </div>
                    </BreadcrumbItem>
                </Breadcrumbs>
            </Section>
        </ContentLayout>
    }
}
