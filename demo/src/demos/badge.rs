use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn BadgeDemo() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <h1 class="text-3xl font-bold">"Badge"</h1>
            <p class="text-base-content/70">
                "Badges are used to inform the user of the status of specific data"
            </p>

            <div class="space-y-4">
                <h2 class="text-xl font-semibold">"Colors"</h2>
                <div class="flex flex-wrap gap-2">
                    <Badge>"Default"</Badge>
                    <Badge color=BadgeColor::Neutral>"Neutral"</Badge>
                    <Badge color=BadgeColor::Primary>"Primary"</Badge>
                    <Badge color=BadgeColor::Secondary>"Secondary"</Badge>
                    <Badge color=BadgeColor::Accent>"Accent"</Badge>
                    <Badge color=BadgeColor::Info>"Info"</Badge>
                    <Badge color=BadgeColor::Success>"Success"</Badge>
                    <Badge color=BadgeColor::Warning>"Warning"</Badge>
                    <Badge color=BadgeColor::Error>"Error"</Badge>
                </div>

                <h2 class="text-xl font-semibold">"Sizes"</h2>
                <div class="flex items-center gap-2">
                    <Badge size=BadgeSize::Xs>"XS"</Badge>
                    <Badge size=BadgeSize::Sm>"SM"</Badge>
                    <Badge size=BadgeSize::Md>"MD"</Badge>
                    <Badge size=BadgeSize::Lg>"LG"</Badge>
                    <Badge size=BadgeSize::Xl>"XL"</Badge>
                </div>

                <h2 class="text-xl font-semibold">"Styles"</h2>
                <div class="flex gap-2">
                    <Badge color=BadgeColor::Primary>"Default"</Badge>
                    <Badge
                        style=BadgeStyle::Outline
                        color=BadgeColor::Primary
                    >
                        "Outline"
                    </Badge>
                    <Badge
                        style=BadgeStyle::Ghost
                        color=BadgeColor::Primary
                    >
                        "Ghost"
                    </Badge>
                    <Badge
                        style=BadgeStyle::Soft
                        color=BadgeColor::Primary
                    >
                        "Soft"
                    </Badge>
                </div>

                <h2 class="text-xl font-semibold">"In Text"</h2>
                <div class="text-lg">
                    "Inbox " <Badge color=BadgeColor::Secondary>"3"</Badge>
                </div>

                <h2 class="text-xl font-semibold">"Empty Badge"</h2>
                <div class="flex items-center gap-2">
                    "Notifications "
                    <Badge color=BadgeColor::Error class="w-3 h-3 p-0">
                        "Notifications"
                    </Badge>
                </div>
            </div>
        </div>
    }
}