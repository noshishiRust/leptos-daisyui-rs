use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn TabDemo() -> impl IntoView {
    let (active_tab, set_active_tab) = signal(0);
    let (bordered_tab, set_bordered_tab) = signal(0);
    let (boxed_tab, set_boxed_tab) = signal(0);

    view! {
        <ContentLayout
            title="Tab"
            description="Tabs are used to organize content into different sections"
        >
            <Section title="Basic Tabs">
                <Tabs variant=TabVariant::Lift>
                    <Tab
                        active=Signal::derive(move || active_tab.get() == 0)
                        on:click=move |_| set_active_tab.set(0)
                    >
                        "Tab 1"
                    </Tab>
                    <Tab
                        active=Signal::derive(move || active_tab.get() == 1)
                        on:click=move |_| set_active_tab.set(1)
                    >
                        "Tab 2"
                    </Tab>
                    <Tab
                        active=Signal::derive(move || active_tab.get() == 2)
                        on:click=move |_| set_active_tab.set(2)
                    >
                        "Tab 3"
                    </Tab>
                </Tabs>

                <div class="bg-base-200 p-4 rounded-box mt-4">
                    {move || match active_tab.get() {
                        0 => view! { <p>"Content for Tab 1"</p> }.into_any(),
                        1 => view! { <p>"Content for Tab 2"</p> }.into_any(),
                        2 => view! { <p>"Content for Tab 3"</p> }.into_any(),
                        _ => view! { <p>"Content for Tab 1"</p> }.into_any(),
                    }}
                </div>
            </Section>

            <Section title="Tab Variants">

                <div>
                    <h3 class="text-sm font-medium mb-2">"Bordered Tabs"</h3>
                    <Tabs variant=TabVariant::Border>
                        <Tab
                            active=Signal::derive(move || bordered_tab.get() == 0)
                            on:click=move |_| set_bordered_tab.set(0)
                        >
                            "First"
                        </Tab>
                        <Tab
                            active=Signal::derive(move || bordered_tab.get() == 1)
                            on:click=move |_| set_bordered_tab.set(1)
                        >
                            "Second"
                        </Tab>
                        <Tab
                            active=Signal::derive(move || bordered_tab.get() == 2)
                            on:click=move |_| set_bordered_tab.set(2)
                        >
                            "Third"
                        </Tab>
                    </Tabs>
                </div>

                <div>
                    <h3 class="text-sm font-medium mb-2">"Boxed Tabs"</h3>
                    <Tabs variant=TabVariant::Boxed>
                        <Tab
                            active=Signal::derive(move || boxed_tab.get() == 0)
                            on:click=move |_| set_boxed_tab.set(0)
                        >
                            "Home"
                        </Tab>
                        <Tab
                            active=Signal::derive(move || boxed_tab.get() == 1)
                            on:click=move |_| set_boxed_tab.set(1)
                        >
                            "About"
                        </Tab>
                        <Tab
                            active=Signal::derive(move || boxed_tab.get() == 2)
                            on:click=move |_| set_boxed_tab.set(2)
                        >
                            "Contact"
                        </Tab>
                    </Tabs>
                </div>
            </Section>

            <Section title="Tab Sizes">
                <div>
                    <h3 class="text-sm font-medium mb-2">"Extra Small"</h3>
                    <Tabs size=TabSize::Xs>
                        <Tab active=RwSignal::new(true)>"XS Tab 1"</Tab>
                        <Tab>"XS Tab 2"</Tab>
                    </Tabs>
                </div>
                <div>
                    <h3 class="text-sm font-medium mb-2">"Small"</h3>
                    <Tabs size=TabSize::Sm>
                        <Tab active=RwSignal::new(true)>"SM Tab 1"</Tab>
                        <Tab>"SM Tab 2"</Tab>
                    </Tabs>
                </div>
                <div>
                    <h3 class="text-sm font-medium mb-2">"Large"</h3>
                    <Tabs size=TabSize::Lg>
                        <Tab active=RwSignal::new(true)>"LG Tab 1"</Tab>
                        <Tab>"LG Tab 2"</Tab>
                    </Tabs>
                </div>
            </Section>

            <Section title="Tab with Disabled">
                <Tabs>
                    <Tab active=RwSignal::new(true)>"Active"</Tab>
                    <Tab>"Normal"</Tab>
                    <Tab disabled=RwSignal::new(true)>"Disabled"</Tab>
                </Tabs>
            </Section>
        </ContentLayout>
    }
}
