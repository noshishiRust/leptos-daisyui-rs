use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn ButtonDemo() -> impl IntoView {
    let (counter, set_counter) = signal(0);
    let (loading, set_loading) = signal(false);
    let (active_color, set_active_color) = signal(ButtonColor::Primary);

    view! {
        <ContentLayout
            title="Button"
            description="Buttons allow users to take actions and make choices"
        >
            <Section title="Colors" row=true>
                <Button>"Default"</Button>
                <Button color=ButtonColor::Neutral>"Neutral"</Button>
                <Button color=ButtonColor::Primary>"Primary"</Button>
                <Button color=ButtonColor::Secondary>"Secondary"</Button>
                <Button color=ButtonColor::Accent>"Accent"</Button>
                <Button color=ButtonColor::Info>"Info"</Button>
                <Button color=ButtonColor::Success>"Success"</Button>
                <Button color=ButtonColor::Warning>"Warning"</Button>
                <Button color=ButtonColor::Error>"Error"</Button>
            </Section>

            <Section title="Sizes" row=true>
                <Button size=ButtonSize::Xs>"XS"</Button>
                <Button size=ButtonSize::Sm>"SM"</Button>
                <Button size=ButtonSize::Md>"MD"</Button>
                <Button size=ButtonSize::Lg>"LG"</Button>
                <Button size=ButtonSize::Xl>"XL"</Button>
            </Section>

            <Section title="Styles" row=true>
                <Button style=ButtonStyle::Default>"Default"</Button>
                <Button style=ButtonStyle::Outline>"Outline"</Button>
                <Button style=ButtonStyle::Ghost>"Ghost"</Button>
                <Button style=ButtonStyle::Link>"Link"</Button>
                <Button style=ButtonStyle::Soft>"Soft"</Button>
                <Button style=ButtonStyle::Dash>"Dash"</Button>
            </Section>

            <Section title="States" row=true>
                <Button color=ButtonColor::Primary>"Normal"</Button>
                <Button color=ButtonColor::Primary active=true>
                    "Active"
                </Button>
                <Button color=ButtonColor::Primary disabled=true>
                    "Disabled"
                </Button>
                <Button color=ButtonColor::Primary class="loading">
                    "Loading"
                </Button>
            </Section>

            <Section title="Shapes" row=true>
                <Button color=ButtonColor::Primary shape=ButtonShape::Wide>
                    "Wide"
                </Button>
                <Button color=ButtonColor::Primary shape=ButtonShape::Square>
                    "□"
                </Button>
                <Button color=ButtonColor::Primary shape=ButtonShape::Circle>
                    "○"
                </Button>
                <Button color=ButtonColor::Primary shape=ButtonShape::Block>
                    "Block"
                </Button>
            </Section>

            <Section title="Reactive Examples">
                <div class="flex flex-col gap-4">
                    <div class="flex items-center gap-2">
                        <Button
                            color=ButtonColor::Primary
                            on:click=move |_| set_counter.update(|c| *c += 1)
                        >
                            "Clicked "
                            {counter}
                            " times"
                        </Button>
                        <Button
                            color=ButtonColor::Error
                            style=ButtonStyle::Outline
                            on:click=move |_| set_counter.set(0)
                        >
                            "Reset"
                        </Button>
                    </div>

                    <div class="flex items-center gap-2">
                        <Button
                            color=ButtonColor::Info
                            class:loading=loading
                            on:click=move |_| {
                                set_loading.set(true);
                                set_timeout(
                                    move || set_loading.set(false),
                                    std::time::Duration::from_millis(2000),
                                );
                            }
                        >
                            "Simulate Loading"
                        </Button>
                    </div>

                    <div class="flex items-center gap-2">
                        <Button
                            color=active_color
                            on:click=move |_| {
                                set_active_color
                                    .update(|color| {
                                        *color = match color {
                                            ButtonColor::Primary => ButtonColor::Secondary,
                                            ButtonColor::Secondary => ButtonColor::Accent,
                                            ButtonColor::Accent => ButtonColor::Success,
                                            ButtonColor::Success => ButtonColor::Warning,
                                            ButtonColor::Warning => ButtonColor::Error,
                                            ButtonColor::Error => ButtonColor::Primary,
                                            _ => ButtonColor::Primary,
                                        }
                                    });
                            }
                        >
                            "Cycle Color: "
                            {move || format!("{:?}", active_color.get())}
                        </Button>
                    </div>
                </div>
            </Section>

            <Section title="Link Button">
                <div class="flex items-center gap-2">
                    <LinkButton
                        href="javascript:void(0)"
                        color=ButtonColor::Primary
                        on:click=move |e| {
                            e.prevent_default();
                        }
                    >
                        "Primary Link"
                    </LinkButton>
                    <LinkButton
                        href="javascript:void(0)"
                        style=ButtonStyle::Outline
                        on:click=move |e| {
                            e.prevent_default();
                        }
                    >
                        "Outline Link"
                    </LinkButton>
                    <LinkButton
                        href="javascript:void(0)"
                        style=ButtonStyle::Ghost
                        on:click=move |e| {
                            e.prevent_default();
                        }
                    >
                        "Ghost Link"
                    </LinkButton>
                </div>
            </Section>
        </ContentLayout>
    }
}
