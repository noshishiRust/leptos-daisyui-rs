use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

/// Tag component demo
#[component]
pub fn TagDemo() -> impl IntoView {
    let (tags, set_tags) = signal(vec![
        "JavaScript".to_string(),
        "Rust".to_string(),
        "TypeScript".to_string(),
        "Python".to_string(),
    ]);

    let remove_tag = move |tag_to_remove: String| {
        set_tags.update(|tags| {
            tags.retain(|t| t != &tag_to_remove);
        });
    };

    view! {
        <div class="space-y-8">
            <div>
                <h2 class="text-2xl font-bold mb-4">"Tag Component"</h2>
                <p class="mb-4 opacity-70">
                    "Tags are used for labels, filters, and categories. They can be closable, have different colors and sizes."
                </p>
            </div>

            // Color Variants
            <div class="card bg-base-200">
                <div class="card-body">
                    <h3 class="card-title">"Color Variants"</h3>
                    <div class="flex gap-2 flex-wrap">
                        <Tag color=TagColor::Neutral>"Neutral"</Tag>
                        <Tag color=TagColor::Primary>"Primary"</Tag>
                        <Tag color=TagColor::Secondary>"Secondary"</Tag>
                        <Tag color=TagColor::Accent>"Accent"</Tag>
                        <Tag color=TagColor::Success>"Success"</Tag>
                        <Tag color=TagColor::Warning>"Warning"</Tag>
                        <Tag color=TagColor::Error>"Error"</Tag>
                        <Tag color=TagColor::Info>"Info"</Tag>
                    </div>
                </div>
            </div>

            // Size Variants
            <div class="card bg-base-200">
                <div class="card-body">
                    <h3 class="card-title">"Size Variants"</h3>
                    <div class="flex gap-2 items-center flex-wrap">
                        <Tag size=TagSize::Small color=TagColor::Primary>"Small"</Tag>
                        <Tag size=TagSize::Medium color=TagColor::Primary>"Medium"</Tag>
                        <Tag size=TagSize::Large color=TagColor::Primary>"Large"</Tag>
                    </div>
                </div>
            </div>

            // Outline Style
            <div class="card bg-base-200">
                <div class="card-body">
                    <h3 class="card-title">"Outline Style"</h3>
                    <div class="flex gap-2 flex-wrap">
                        <Tag color=TagColor::Primary outline=true>"Primary"</Tag>
                        <Tag color=TagColor::Secondary outline=true>"Secondary"</Tag>
                        <Tag color=TagColor::Accent outline=true>"Accent"</Tag>
                        <Tag color=TagColor::Success outline=true>"Success"</Tag>
                        <Tag color=TagColor::Warning outline=true>"Warning"</Tag>
                        <Tag color=TagColor::Error outline=true>"Error"</Tag>
                    </div>
                </div>
            </div>

            // Closable Tags
            <div class="card bg-base-200">
                <div class="card-body">
                    <h3 class="card-title">"Closable Tags"</h3>
                    <p class="text-sm opacity-70 mb-4">
                        "Click the X button to remove tags from the list"
                    </p>
                    <div class="flex gap-2 flex-wrap">
                        {move || tags.get().into_iter().map(|tag| {
                            let tag_clone = tag.clone();
                            let tag_clone2 = tag.clone();
                            view! {
                                <Tag
                                    color=TagColor::Primary
                                    closable=true
                                    on_close=Callback::new(move |_| remove_tag(tag_clone.clone()))
                                >
                                    {tag_clone2}
                                </Tag>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                    {move || if tags.get().is_empty() {
                        view! {
                            <p class="text-sm opacity-50 mt-4">"All tags removed!"</p>
                        }.into_any()
                    } else {
                        ().into_any()
                    }}
                </div>
            </div>

            // Use Cases
            <div class="card bg-base-200">
                <div class="card-body">
                    <h3 class="card-title">"Use Cases"</h3>

                    <div class="space-y-4">
                        // Category Labels
                        <div>
                            <h4 class="font-semibold mb-2">"Category Labels"</h4>
                            <div class="flex gap-2 flex-wrap">
                                <Tag color=TagColor::Info>"Documentation"</Tag>
                                <Tag color=TagColor::Success>"Bug Fix"</Tag>
                                <Tag color=TagColor::Warning>"Enhancement"</Tag>
                                <Tag color=TagColor::Primary>"Feature"</Tag>
                            </div>
                        </div>

                        // Status Tags
                        <div>
                            <h4 class="font-semibold mb-2">"Status Tags"</h4>
                            <div class="flex gap-2 flex-wrap">
                                <Tag color=TagColor::Success>"Active"</Tag>
                                <Tag color=TagColor::Warning>"Pending"</Tag>
                                <Tag color=TagColor::Error>"Rejected"</Tag>
                                <Tag color=TagColor::Neutral>"Draft"</Tag>
                            </div>
                        </div>

                        // Technology Stack
                        <div>
                            <h4 class="font-semibold mb-2">"Technology Stack"</h4>
                            <div class="flex gap-2 flex-wrap">
                                <Tag color=TagColor::Accent outline=true>"Rust"</Tag>
                                <Tag color=TagColor::Accent outline=true>"Leptos"</Tag>
                                <Tag color=TagColor::Accent outline=true>"daisyUI"</Tag>
                                <Tag color=TagColor::Accent outline=true>"Tailwind CSS"</Tag>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
