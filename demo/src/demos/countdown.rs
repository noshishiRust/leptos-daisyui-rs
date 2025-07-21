use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn CountdownDemo() -> impl IntoView {
    let (days, set_days) = signal(15);
    let (hours, set_hours) = signal(10);
    let (minutes, set_minutes) = signal(24);
    let (seconds, set_seconds) = signal(47);

    view! {
        <ContentLayout
            title="Countdown"
            description="Countdown component shows a timer that counts down to zero"
        >
            <Section title="Basic Countdown" col=true>
                <div class="grid grid-flow-col gap-5 text-center auto-cols-max">
                    <div class="flex flex-col">
                        <span class="countdown font-mono text-5xl">
                            <span style=move || format!("--value:{};", days.get())></span>
                        </span>
                        "days"
                    </div>
                    <div class="flex flex-col">
                        <span class="countdown font-mono text-5xl">
                            <span style=move || format!("--value:{};", hours.get())></span>
                        </span>
                        "hours"
                    </div>
                    <div class="flex flex-col">
                        <span class="countdown font-mono text-5xl">
                            <span style=move || format!("--value:{};", minutes.get())></span>
                        </span>
                        "min"
                    </div>
                    <div class="flex flex-col">
                        <span class="countdown font-mono text-5xl">
                            <span style=move || format!("--value:{};", seconds.get())></span>
                        </span>
                        "sec"
                    </div>
                </div>

                <div class="flex gap-2 justify-center mt-4">
                    <Button
                        on:click=move |_| {
                            set_days.update(|d| *d = (*d + 1) % 100);
                            set_hours.update(|h| *h = (*h + 1) % 24);
                            set_minutes.update(|m| *m = (*m + 1) % 60);
                            set_seconds.update(|s| *s = (*s + 1) % 60);
                        }
                        size=ButtonSize::Sm
                    >
                        "Update Values"
                    </Button>
                    <Button
                        on:click=move |_| {
                            set_days.set(0);
                            set_hours.set(0);
                            set_minutes.set(0);
                            set_seconds.set(0);
                        }
                        style=ButtonStyle::Outline
                        size=ButtonSize::Sm
                    >
                        "Reset"
                    </Button>
                </div>
            </Section>

            <Section title="Size Variations" row=true>
                <div class="grid grid-flow-col gap-2 text-center auto-cols-max">
                    <div class="flex flex-col">
                        <span class="countdown font-mono text-2xl">
                            <span style="--value:7;"></span>
                        </span>
                        <span class="text-xs">"Small"</span>
                    </div>
                    <div class="flex flex-col">
                        <span class="countdown font-mono text-4xl">
                            <span style="--value:18;"></span>
                        </span>
                        <span class="text-sm">"Medium"</span>
                    </div>
                    <div class="flex flex-col">
                        <span class="countdown font-mono text-6xl">
                            <span style="--value:32;"></span>
                        </span>
                        <span class="text-base">"Large"</span>
                    </div>
                </div>
            </Section>

            <Section title="Colored Variations" row=true>
                <div class="flex flex-col p-2 bg-primary rounded-box text-primary-content">
                    <span class="countdown font-mono text-3xl">
                        <span style="--value:30;"></span>
                    </span>
                    "Primary"
                </div>
                <div class="flex flex-col p-2 bg-secondary rounded-box text-secondary-content">
                    <span class="countdown font-mono text-3xl">
                        <span style="--value:12;"></span>
                    </span>
                    "Secondary"
                </div>
                <div class="flex flex-col p-2 bg-accent rounded-box text-accent-content">
                    <span class="countdown font-mono text-3xl">
                        <span style="--value:45;"></span>
                    </span>
                    "Accent"
                </div>
            </Section>
        </ContentLayout>
    }
}
