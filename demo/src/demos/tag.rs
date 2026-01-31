use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn TagDemo() -> impl IntoView {
    let (tags, set_tags) = signal(vec![
        "Design".to_string(),
        "Development".to_string(),
        "Testing".to_string(),
    ]);

    let remove_tag = move |tag: String| {
        set_tags.update(|t| t.retain(|item| item != &tag));
    };

    view! {
        <ContentLayout
            title="Tag"
            description="Label components for categorization, status indication, and closable tags"
        >
            <Section title="Basic Tags">
                <div class="flex gap-2 flex-wrap">
                    <Tag color=TagColor::Neutral size=TagSize::Medium outline=false closable=false >
                        "Default"
                    </Tag>
                    <Tag color=TagColor::Primary size=TagSize::Medium outline=false closable=false >
                        "Primary"
                    </Tag>
                    <Tag color=TagColor::Success size=TagSize::Medium outline=false closable=false >
                        "Success"
                    </Tag>
                    <Tag color=TagColor::Warning size=TagSize::Medium outline=false closable=false >
                        "Warning"
                    </Tag>
                    <Tag color=TagColor::Error size=TagSize::Medium outline=false closable=false >
                        "Error"
                    </Tag>
                </div>
            </Section>

            <Section title="Tag Sizes">
                <div class="flex gap-2 items-center flex-wrap">
                    <Tag color=TagColor::Primary size=TagSize::Small outline=false closable=false >
                        "Small"
                    </Tag>
                    <Tag color=TagColor::Primary size=TagSize::Medium outline=false closable=false >
                        "Medium"
                    </Tag>
                    <Tag color=TagColor::Primary size=TagSize::Large outline=false closable=false >
                        "Large"
                    </Tag>
                </div>
            </Section>

            <Section title="Closable Tags">
                <div class="flex gap-2 flex-wrap">
                    <Tag
                        color=TagColor::Primary
                        size=TagSize::Medium
                        outline=false
                        closable=true
                        on_close=Callback::new(move |_| {
                            leptos::logging::log!("Tag closed");
                        })
                    >
                        "Click X to close"
                    </Tag>
                </div>
            </Section>

            <Section title="Dynamic Tag List">
                <div class="space-y-4">
                    <div class="flex gap-2 flex-wrap">
                        {move || {
                            tags.get()
                                .into_iter()
                                .map(|tag| {
                                    let tag_clone = tag.clone();
                                    view! {
                                        <Tag
                                            color=TagColor::Accent
                                            size=TagSize::Medium
                                            outline=false
                                            closable=true
                                            on_close=Callback::new(move |_| {
                                                remove_tag(tag_clone.clone());
                                            })
                                        >
                                            {tag}
                                        </Tag>
                                    }
                                })
                                .collect_view()
                        }}
                    </div>
                    <p class="text-sm opacity-70">
                        "Click X to remove tags. Count: "
                        {move || tags.get().len().to_string()}
                    </p>
                </div>
            </Section>
        </ContentLayout>
    }
}
