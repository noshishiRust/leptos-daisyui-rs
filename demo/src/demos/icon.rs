use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn IconDemo() -> impl IntoView {
    view! {
        <ContentLayout
            title="Icon"
            description="Icon component with Lucide icon support for displaying SVG icons"
        >
            <Section title="Basic Icons">
                <div class="flex gap-4 items-center">
                    <Icon name="heart".to_string() size=IconSize::Medium />
                    <Icon name="star".to_string() size=IconSize::Medium />
                    <Icon name="user".to_string() size=IconSize::Medium />
                    <Icon name="home".to_string() size=IconSize::Medium />
                    <Icon name="settings".to_string() size=IconSize::Medium />
                </div>
            </Section>

            <Section title="Icon Sizes">
                <div class="flex gap-6 items-center">
                    <div class="flex flex-col items-center gap-2">
                        <Icon name="star".to_string() size=IconSize::XSmall />
                        <span class="text-xs">"XSmall"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Icon name="star".to_string() size=IconSize::Small />
                        <span class="text-xs">"Small"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Icon name="star".to_string() size=IconSize::Medium />
                        <span class="text-xs">"Medium"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Icon name="star".to_string() size=IconSize::Large />
                        <span class="text-xs">"Large"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Icon name="star".to_string() size=IconSize::XLarge />
                        <span class="text-xs">"XLarge"</span>
                    </div>
                </div>
            </Section>

            <Section title="Colored Icons">
                <div class="flex gap-4 items-center">
                    <Icon name="heart".to_string() size=IconSize::Large color="text-error".to_string() />
                    <Icon name="star".to_string() size=IconSize::Large color="text-warning".to_string() />
                    <Icon name="check-circle".to_string() size=IconSize::Large color="text-success".to_string() />
                    <Icon name="info".to_string() size=IconSize::Large color="text-info".to_string() />
                </div>
            </Section>

            <Section title="Setup Instructions" col=true>
                <div class="alert alert-info">
                    <div>
                        <h4 class="font-bold">"Lucide Icons Setup Required"</h4>
                        <p class="text-sm">
                            "Add the following to your "
                            <code class="bg-base-300 px-1 rounded">"index.html"</code>
                            " file:"
                        </p>
                        <pre class="bg-base-300 p-2 rounded mt-2 text-xs overflow-x-auto">
                            {"<script src=\"https://unpkg.com/lucide@latest\"></script>\n<script>\n  lucide.createIcons();\n</script>"}
                        </pre>
                    </div>
                </div>
            </Section>
        </ContentLayout>
    }
}
