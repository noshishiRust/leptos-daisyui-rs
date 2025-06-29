use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn TabDemo() -> impl IntoView {
    let (active_tab, set_active_tab) = signal(0);

    view! {
        <div class="space-y-6">
            <h1 class="text-3xl font-bold">"Tab"</h1>
            <p class="text-base-content/70">
                "Tabs are used to organize content into different sections"
            </p>

            <div class="space-y-4">
                <h2 class="text-xl font-semibold">"Basic Tabs"</h2>
                <div class="tabs">
                    <Tab
                        class="tab-lifted"
                        class:tab-active=Signal::derive(move || active_tab.get() == 0)
                        on:click=move |_| set_active_tab.set(0)
                    >
                        "Tab 1"
                    </Tab>
                    <Tab
                        class="tab-lifted"
                        class:tab-active=Signal::derive(move || active_tab.get() == 1)
                        on:click=move |_| set_active_tab.set(1)
                    >
                        "Tab 2"
                    </Tab>
                    <Tab
                        class="tab-lifted"
                        class:tab-active=Signal::derive(move || active_tab.get() == 2)
                        on:click=move |_| set_active_tab.set(2)
                    >
                        "Tab 3"
                    </Tab>
                </div>

                <div class="bg-base-200 p-4 rounded-box">
                    {move || match active_tab.get() {
                        0 => view! { <p>"Content for Tab 1"</p> }.into_any(),
                        1 => view! { <p>"Content for Tab 2"</p> }.into_any(),
                        2 => view! { <p>"Content for Tab 3"</p> }.into_any(),
                        _ => view! { <p>"Content for Tab 1"</p> }.into_any(),
                    }}
                </div>

                <h2 class="text-xl font-semibold">"Tab Styles"</h2>
                <div class="tabs tabs-bordered">
                    <Tab class="tab tab-active">"Bordered"</Tab>
                    <Tab class="tab">"Tab"</Tab>
                    <Tab class="tab">"Tab"</Tab>
                </div>

                <div class="tabs tabs-boxed">
                    <Tab class="tab tab-active">"Boxed"</Tab>
                    <Tab class="tab">"Tab"</Tab>
                    <Tab class="tab">"Tab"</Tab>
                </div>
            </div>
        </div>
    }
}