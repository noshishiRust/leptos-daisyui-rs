use leptos::prelude::*;
use leptos_daisyui_rs::components::*;
use crate::core::{ContentLayout, Section};

#[component]
pub fn DiffDemo() -> impl IntoView {
    let (left_text, set_left_text) = signal("Original text content");
    let (right_text, set_right_text) = signal("Updated text content");
    let (show_before, set_show_before) = signal(true);

    view! {
        <ContentLayout
            title="Diff"
            description="Diff component shows the difference between two pieces of content"
        >
            <Section title="Basic Text Diff" col=true>
                <Diff class="aspect-[16/9]">
                    <DiffItem1>
                        <div class="bg-warning text-warning-content p-8 text-center h-full flex flex-col justify-center">
                            <h2 class="text-2xl font-bold mb-4">"Before"</h2>
                            <p>{move || left_text.get()}</p>
                            <p class="mt-2">"Contact us at: info@example.com"</p>
                            <p class="mt-2">"Phone: (555) 123-4567"</p>
                        </div>
                    </DiffItem1>
                    <DiffItem2>
                        <div class="bg-success text-success-content p-8 text-center h-full flex flex-col justify-center">
                            <h2 class="text-2xl font-bold mb-4">"After"</h2>
                            <p>{move || right_text.get()}</p>
                            <p class="mt-2">"Contact us at: hello@newdomain.com"</p>
                            <p class="mt-2">"Phone: (555) 987-6543"</p>
                            <p class="mt-2">"Live Chat Available 24/7"</p>
                        </div>
                    </DiffItem2>
                    <DiffResizer />
                </Diff>

                <div class="flex gap-2 mt-4">
                    <Button
                        on:click=move |_| {
                            set_left_text.set("Welcome to our website! We offer great services.")
                        }
                        size=ButtonSize::Sm
                    >
                        "Update Left"
                    </Button>
                    <Button
                        on:click=move |_| {
                            set_right_text
                                .set(
                                    "Welcome to our improved website! We offer amazing services and support.",
                                )
                        }
                        size=ButtonSize::Sm
                        color=ButtonColor::Secondary
                    >
                        "Update Right"
                    </Button>
                </div>
            </Section>

            <Section title="Basic Brand Diff" col=true>
                <Diff class="aspect-[16/9]">
                    <DiffItem1>
                        <div class="bg-primary text-primary-content text-9xl font-black grid place-content-center h-full">
                            "DAISY"
                        </div>
                    </DiffItem1>
                    <DiffItem2>
                        <div class="bg-base-200 text-9xl font-black grid place-content-center h-full">
                            "DAISY"
                        </div>
                    </DiffItem2>
                    <DiffResizer />
                </Diff>
            </Section>

            <Section title="Interactive Theme Comparison" col=true>
                <div class="flex gap-2 mb-4">
                    <Button
                        on:click=move |_| set_show_before.update(|b| *b = !*b)
                        size=ButtonSize::Sm
                        color=move || if show_before.get() {
                            ButtonColor::Primary
                        } else {
                            ButtonColor::Secondary
                        }
                    >
                        "Toggle Theme: "
                        {move || if show_before.get() { "Dark" } else { "Light" }}
                    </Button>
                </div>

                <Diff class="aspect-[16/9]">
                    <DiffItem1>
                        <div class=move || {
                            if show_before.get() {
                                "bg-gradient-to-br from-gray-800 to-gray-900 p-8 h-full"
                            } else {
                                "bg-gradient-to-br from-blue-50 to-indigo-100 p-8 h-full"
                            }
                        }>
                            <h3 class=move || {
                                if show_before.get() {
                                    "text-white font-bold mb-6"
                                } else {
                                    "text-gray-800 font-bold mb-6"
                                }
                            }>
                                {move || {
                                    if show_before.get() { "Dark Theme" } else { "Light Theme" }
                                }}
                            </h3>
                            <div class="space-y-4">
                                <div class=move || {
                                    if show_before.get() {
                                        "bg-gray-700 p-4 rounded-lg"
                                    } else {
                                        "bg-white p-4 rounded-lg shadow-sm"
                                    }
                                }>
                                    <h4 class=move || {
                                        if show_before.get() {
                                            "text-white font-semibold"
                                        } else {
                                            "text-gray-800 font-semibold"
                                        }
                                    }>"Navigation"</h4>
                                    <div class="flex gap-2 mt-2">
                                        <span class=move || {
                                            if show_before.get() {
                                                "px-3 py-1 bg-gray-600 text-white rounded text-sm"
                                            } else {
                                                "px-3 py-1 bg-gray-200 text-gray-700 rounded text-sm"
                                            }
                                        }>"Home"</span>
                                        <span class=move || {
                                            if show_before.get() {
                                                "px-3 py-1 bg-blue-600 text-white rounded text-sm"
                                            } else {
                                                "px-3 py-1 bg-blue-500 text-white rounded text-sm"
                                            }
                                        }>"About"</span>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </DiffItem1>
                    <DiffItem2>
                        <div class="bg-gradient-to-br from-purple-500 to-blue-500 p-8 h-full">
                            <h3 class="text-lg font-bold mb-4 text-white">"Modern Design"</h3>
                            <div class="space-y-4">
                                <Button color=ButtonColor::Primary size=ButtonSize::Lg>
                                    "Enhanced Button"
                                </Button>
                                <Alert color=AlertColor::Success>
                                    "Improved alert with better styling"
                                </Alert>
                                <Card class="bg-white/90 backdrop-blur shadow-xl">
                                    <CardBody>
                                        <h2 class="card-title">"Modern Card"</h2>
                                        <p>"Enhanced card with better typography"</p>
                                        <div class="card-actions justify-end">
                                            <Button color=ButtonColor::Primary size=ButtonSize::Sm>
                                                "Action"
                                            </Button>
                                        </div>
                                    </CardBody>
                                </Card>
                            </div>
                        </div>
                    </DiffItem2>
                    <DiffResizer />
                </Diff>
            </Section>

            <Section title="Code Comparison" col=true>
                <Diff class="aspect-[16/9]">
                    <DiffItem1>
                        <div class="bg-base-300 p-6 h-full overflow-auto">
                            <h3 class="font-bold mb-4 text-error">"❌ Before (HTML)"</h3>
                            <div class="mockup-code text-sm">
                                <pre data-prefix="1">
                                    <code>"<div class=\"container\">"</code>
                                </pre>
                                <pre data-prefix="2">
                                    <code>"  <h1>Title</h1>"</code>
                                </pre>
                                <pre data-prefix="3">
                                    <code>"  <p>Content</p>"</code>
                                </pre>
                                <pre data-prefix="4">
                                    <code>"  <button onclick=\"alert('Hi')\">"</code>
                                </pre>
                                <pre data-prefix="5">
                                    <code>"    Click me"</code>
                                </pre>
                                <pre data-prefix="6">
                                    <code>"  </button>"</code>
                                </pre>
                                <pre data-prefix="7">
                                    <code>"</div>"</code>
                                </pre>
                            </div>
                        </div>
                    </DiffItem1>
                    <DiffItem2>
                        <div class="bg-base-100 p-6 h-full overflow-auto">
                            <h3 class="font-bold mb-4 text-success">
                                "✅ After (Leptos + daisyUI)"
                            </h3>
                            <div class="mockup-code text-sm">
                                <pre data-prefix="1">
                                    <code>"<Card class=\"shadow-xl\">"</code>
                                </pre>
                                <pre data-prefix="2">
                                    <code>"  <CardBody>"</code>
                                </pre>
                                <pre data-prefix="3">
                                    <code>"    <h1 class=\"card-title\">"</code>
                                </pre>
                                <pre data-prefix="4">
                                    <code>"      \"Title\""</code>
                                </pre>
                                <pre data-prefix="5">
                                    <code>"    </h1>"</code>
                                </pre>
                                <pre data-prefix="6">
                                    <code>"    <p>\"Content\"</p>"</code>
                                </pre>
                                <pre data-prefix="7">
                                    <code>"    <Button color=Primary>"</code>
                                </pre>
                                <pre data-prefix="8">
                                    <code>"      \"Click me\""</code>
                                </pre>
                                <pre data-prefix="9">
                                    <code>"    </Button>"</code>
                                </pre>
                                <pre data-prefix="10">
                                    <code>"  </CardBody>"</code>
                                </pre>
                                <pre data-prefix="11">
                                    <code>"</Card>"</code>
                                </pre>
                            </div>
                        </div>
                    </DiffItem2>
                    <DiffResizer />
                </Diff>
            </Section>
        </ContentLayout>
    }
}
