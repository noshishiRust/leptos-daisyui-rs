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
                    <Badge color=Signal::derive(move || BadgeColor::Neutral)>"Neutral"</Badge>
                    <Badge color=Signal::derive(move || BadgeColor::Primary)>"Primary"</Badge>
                    <Badge color=Signal::derive(move || BadgeColor::Secondary)>"Secondary"</Badge>
                    <Badge color=Signal::derive(move || BadgeColor::Accent)>"Accent"</Badge>
                    <Badge color=Signal::derive(move || BadgeColor::Info)>"Info"</Badge>
                    <Badge color=Signal::derive(move || BadgeColor::Success)>"Success"</Badge>
                    <Badge color=Signal::derive(move || BadgeColor::Warning)>"Warning"</Badge>
                    <Badge color=Signal::derive(move || BadgeColor::Error)>"Error"</Badge>
                </div>

                <h2 class="text-xl font-semibold">"Sizes"</h2>
                <div class="flex items-center gap-2">
                    <Badge size=Signal::derive(move || BadgeSize::Xs)>"XS"</Badge>
                    <Badge size=Signal::derive(move || BadgeSize::Sm)>"SM"</Badge>
                    <Badge size=Signal::derive(move || BadgeSize::Md)>"MD"</Badge>
                    <Badge size=Signal::derive(move || BadgeSize::Lg)>"LG"</Badge>
                    <Badge size=Signal::derive(move || BadgeSize::Xl)>"XL"</Badge>
                </div>

                <h2 class="text-xl font-semibold">"Styles"</h2>
                <div class="flex gap-2">
                    <Badge color=Signal::derive(move || BadgeColor::Primary)>"Default"</Badge>
                    <Badge
                        style=Signal::derive(move || BadgeStyle::Outline)
                        color=Signal::derive(move || BadgeColor::Primary)
                    >
                        "Outline"
                    </Badge>
                    <Badge
                        style=Signal::derive(move || BadgeStyle::Ghost)
                        color=Signal::derive(move || BadgeColor::Primary)
                    >
                        "Ghost"
                    </Badge>
                    <Badge
                        style=Signal::derive(move || BadgeStyle::Soft)
                        color=Signal::derive(move || BadgeColor::Primary)
                    >
                        "Soft"
                    </Badge>
                </div>

                <h2 class="text-xl font-semibold">"In Text"</h2>
                <div class="text-lg">
                    "Inbox " <Badge color=Signal::derive(move || BadgeColor::Secondary)>"3"</Badge>
                </div>

                <h2 class="text-xl font-semibold">"Empty Badge"</h2>
                <div class="flex items-center gap-2">
                    "Notifications "
                    <Badge color=Signal::derive(move || BadgeColor::Error) class="w-3 h-3 p-0">
                        "Notifications"
                    </Badge>
                </div>
            </div>
        </div>
    }
}