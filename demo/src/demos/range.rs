use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::{Range, RangeColor, RangeSize};

#[component]
pub fn RangeDemo() -> impl IntoView {
    let (basic_value, set_basic_value) = signal(50);
    let (primary_value, set_primary_value) = signal(25);
    let (secondary_value, set_secondary_value) = signal(40);
    let (accent_value, set_accent_value) = signal(60);
    let (success_value, set_success_value) = signal(70);
    let (warning_value, set_warning_value) = signal(80);
    let (info_value, set_info_value) = signal(90);
    let (error_value, set_error_value) = signal(35);
    let (xs_value, set_xs_value) = signal(20);
    let (sm_value, set_sm_value) = signal(30);
    let (md_value, set_md_value) = signal(50);
    let (lg_value, set_lg_value) = signal(70);
    let (xl_value, set_xl_value) = signal(85);
    let (labeled_value, set_labeled_value) = signal(0);
    let (display_value, set_display_value) = signal(50);

    view! {
        <ContentLayout
            title="Range"
            description="Range slider is used to select a value by sliding a handle"
        >

            <Section title="Basic Range" col=true>
                <div class="w-full max-w-xs">
                    <Range
                        prop:min="0"
                        prop:max="100"
                        prop:value=basic_value
                        on:input=move |ev| {
                            let value = event_target_value(&ev).parse::<i32>().unwrap_or(0);
                            set_basic_value.set(value);
                        }
                    />
                    <div class="text-sm text-base-content/70 mt-2">
                        "Value: " {move || basic_value.get()}
                    </div>
                </div>
            </Section>

            <Section title="Range Colors" col=true>
                <div class="w-full max-w-xs">
                    <label class="text-sm font-medium">"Primary"</label>
                    <Range
                        color=RangeColor::Primary
                        prop:min="0"
                        prop:max="100"
                        prop:value=primary_value
                        on:input=move |ev| {
                            let value = event_target_value(&ev).parse::<i32>().unwrap_or(0);
                            set_primary_value.set(value);
                        }
                    />
                </div>

                <div class="w-full max-w-xs">
                    <label class="text-sm font-medium">"Secondary"</label>
                    <Range
                        color=RangeColor::Secondary
                        prop:min="0"
                        prop:max="100"
                        prop:value=secondary_value
                        on:input=move |ev| {
                            let value = event_target_value(&ev).parse::<i32>().unwrap_or(0);
                            set_secondary_value.set(value);
                        }
                    />
                </div>

                <div class="w-full max-w-xs">
                    <label class="text-sm font-medium">"Accent"</label>
                    <Range
                        color=RangeColor::Accent
                        prop:min="0"
                        prop:max="100"
                        prop:value=accent_value
                        on:input=move |ev| {
                            let value = event_target_value(&ev).parse::<i32>().unwrap_or(0);
                            set_accent_value.set(value);
                        }
                    />
                </div>

                <div class="w-full max-w-xs">
                    <label class="text-sm font-medium">"Success"</label>
                    <Range
                        color=RangeColor::Success
                        prop:min="0"
                        prop:max="100"
                        prop:value=success_value
                        on:input=move |ev| {
                            let value = event_target_value(&ev).parse::<i32>().unwrap_or(0);
                            set_success_value.set(value);
                        }
                    />
                </div>

                <div class="w-full max-w-xs">
                    <label class="text-sm font-medium">"Warning"</label>
                    <Range
                        color=RangeColor::Warning
                        prop:min="0"
                        prop:max="100"
                        prop:value=warning_value
                        on:input=move |ev| {
                            let value = event_target_value(&ev).parse::<i32>().unwrap_or(0);
                            set_warning_value.set(value);
                        }
                    />
                </div>

                <div class="w-full max-w-xs">
                    <label class="text-sm font-medium">"Info"</label>
                    <Range
                        color=RangeColor::Info
                        prop:min="0"
                        prop:max="100"
                        prop:value=info_value
                        on:input=move |ev| {
                            let value = event_target_value(&ev).parse::<i32>().unwrap_or(0);
                            set_info_value.set(value);
                        }
                    />
                </div>

                <div class="w-full max-w-xs">
                    <label class="text-sm font-medium">"Error"</label>
                    <Range
                        color=RangeColor::Error
                        prop:min="0"
                        prop:max="100"
                        prop:value=error_value
                        on:input=move |ev| {
                            let value = event_target_value(&ev).parse::<i32>().unwrap_or(0);
                            set_error_value.set(value);
                        }
                    />
                </div>
            </Section>

            <Section title="Range Sizes" col=true>
                <div class="w-full max-w-xs">
                    <label class="text-sm font-medium">"Extra Small (xs)"</label>
                    <Range
                        size=RangeSize::Xs
                        color=RangeColor::Primary
                        prop:min="0"
                        prop:max="100"
                        prop:value=xs_value
                        on:input=move |ev| {
                            let value = event_target_value(&ev).parse::<i32>().unwrap_or(0);
                            set_xs_value.set(value);
                        }
                    />
                </div>

                <div class="w-full max-w-xs">
                    <label class="text-sm font-medium">"Small (sm)"</label>
                    <Range
                        size=RangeSize::Sm
                        color=RangeColor::Secondary
                        prop:min="0"
                        prop:max="100"
                        prop:value=sm_value
                        on:input=move |ev| {
                            let value = event_target_value(&ev).parse::<i32>().unwrap_or(0);
                            set_sm_value.set(value);
                        }
                    />
                </div>

                <div class="w-full max-w-xs">
                    <label class="text-sm font-medium">"Medium (md) - Default"</label>
                    <Range
                        size=RangeSize::Md
                        color=RangeColor::Accent
                        prop:min="0"
                        prop:max="100"
                        prop:value=md_value
                        on:input=move |ev| {
                            let value = event_target_value(&ev).parse::<i32>().unwrap_or(0);
                            set_md_value.set(value);
                        }
                    />
                </div>

                <div class="w-full max-w-xs">
                    <label class="text-sm font-medium">"Large (lg)"</label>
                    <Range
                        size=RangeSize::Lg
                        color=RangeColor::Success
                        prop:min="0"
                        prop:max="100"
                        prop:value=lg_value
                        on:input=move |ev| {
                            let value = event_target_value(&ev).parse::<i32>().unwrap_or(0);
                            set_lg_value.set(value);
                        }
                    />
                </div>

                <div class="w-full max-w-xs">
                    <label class="text-sm font-medium">"Extra Large (xl)"</label>
                    <Range
                        size=RangeSize::Xl
                        color=RangeColor::Warning
                        prop:min="0"
                        prop:max="100"
                        prop:value=xl_value
                        on:input=move |ev| {
                            let value = event_target_value(&ev).parse::<i32>().unwrap_or(0);
                            set_xl_value.set(value);
                        }
                    />
                </div>
            </Section>

            <Section title="Range with Labels" col=true>
                <div class="w-full max-w-xs">
                    <div class="flex justify-between text-xs px-2 mb-2">
                        <span>"0°C"</span>
                        <span>"25°C"</span>
                        <span>"50°C"</span>
                        <span>"75°C"</span>
                        <span>"100°C"</span>
                    </div>
                    <Range
                        color=RangeColor::Primary
                        prop:min="0"
                        prop:max="100"
                        prop:step="25"
                        prop:value=labeled_value
                        on:input=move |ev| {
                            let value = event_target_value(&ev).parse::<i32>().unwrap_or(0);
                            set_labeled_value.set(value);
                        }
                    />
                    <div class="flex justify-between text-xs px-2 mt-1">
                        <span>"|"</span>
                        <span>"|"</span>
                        <span>"|"</span>
                        <span>"|"</span>
                        <span>"|"</span>
                    </div>
                </div>
            </Section>

            <Section title="Range with Value Display" col=true>
                <div class="w-full max-w-xs">
                    <div class="flex items-center gap-4">
                        <Range
                            color=RangeColor::Accent
                            size=RangeSize::Md
                            prop:min="0"
                            prop:max="100"
                            prop:value=display_value
                            on:input=move |ev| {
                                let value = event_target_value(&ev).parse::<i32>().unwrap_or(0);
                                set_display_value.set(value);
                            }
                            class="flex-1"
                        />
                        <div class="badge badge-accent badge-lg font-bold min-w-16">
                            {move || display_value.get()} "%"
                        </div>
                    </div>
                </div>
            </Section>

            <Section title="Vertical Range" col=true>
                <p class="text-sm text-base-content/70">
                    "Use CSS transforms to create vertical range sliders"
                </p>
                <div class="flex gap-8 items-end h-64">
                    <div class="flex flex-col items-center gap-2">
                        <div class="text-sm font-medium">"Volume"</div>
                        <Range
                            color=RangeColor::Primary
                            size=RangeSize::Md
                            prop:min="0"
                            prop:max="100"
                            prop:value=75
                            class="w-64 h-8"
                            style:transform="rotate(-90deg)"
                            style:transform-origin="center"
                        />
                        <div class="text-sm text-base-content/70">"75%"</div>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <div class="text-sm font-medium">"Bass"</div>
                        <Range
                            color=RangeColor::Secondary
                            size=RangeSize::Md
                            prop:min="0"
                            prop:max="100"
                            prop:value=60
                            class="w-64 h-8"
                            style:transform="rotate(-90deg)"
                            style:transform-origin="center"
                        />
                        <div class="text-sm text-base-content/70">"60%"</div>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <div class="text-sm font-medium">"Treble"</div>
                        <Range
                            color=RangeColor::Accent
                            size=RangeSize::Md
                            prop:min="0"
                            prop:max="100"
                            prop:value=45
                            class="w-64 h-8"
                            style:transform="rotate(-90deg)"
                            style:transform-origin="center"
                        />
                        <div class="text-sm text-base-content/70">"45%"</div>
                    </div>
                </div>
            </Section>

            <Section title="Styled Range Examples" col=true>
                // Volume Control
                <div class="card bg-base-200 shadow-xl p-6 max-w-md">
                    <div class="flex items-center gap-4 mb-4">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="h-6 w-6"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M15.536 8.464a5 5 0 010 7.072m2.828-9.9a9 9 0 010 12.728M5.586 15H4a1 1 0 01-1-1v-4a1 1 0 011-1h1.586l4.707-4.707C10.923 3.663 12 4.109 12 5v14c0 .891-1.077 1.337-1.707.707L5.586 15z"
                            />
                        </svg>
                        <h4 class="text-lg font-semibold">"Volume Control"</h4>
                    </div>
                    <Range
                        color=RangeColor::Primary
                        size=RangeSize::Lg
                        prop:min="0"
                        prop:max="100"
                        prop:value=65
                    />
                    <div class="text-sm text-base-content/70 mt-2 text-right">"65%"</div>
                </div>

                // Brightness Control
                <div class="card bg-base-200 shadow-xl p-6 max-w-md">
                    <div class="flex items-center gap-4 mb-4">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="h-6 w-6"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"
                            />
                        </svg>
                        <h4 class="text-lg font-semibold">"Brightness"</h4>
                    </div>
                    <Range
                        color=RangeColor::Warning
                        size=RangeSize::Lg
                        prop:min="0"
                        prop:max="100"
                        prop:value=80
                    />
                    <div class="text-sm text-base-content/70 mt-2 text-right">"80%"</div>
                </div>

                // Temperature Control
                <div class="card bg-base-200 shadow-xl p-6 max-w-md">
                    <div class="flex items-center gap-4 mb-4">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="h-6 w-6"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"
                            />
                        </svg>
                        <h4 class="text-lg font-semibold">"Temperature"</h4>
                    </div>
                    <div class="flex justify-between text-xs mb-2">
                        <span>"16°C"</span>
                        <span>"30°C"</span>
                    </div>
                    <Range
                        color=RangeColor::Error
                        size=RangeSize::Lg
                        prop:min="16"
                        prop:max="30"
                        prop:value=22
                    />
                    <div class="text-sm text-base-content/70 mt-2 text-right">"22°C"</div>
                </div>
            </Section>

        </ContentLayout>
    }
}
