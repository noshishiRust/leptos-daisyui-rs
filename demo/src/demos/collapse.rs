use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn CollapseDemo() -> impl IntoView {
    let (checkbox_open, set_checkbox_open) = signal(false);

    view! {
        <div class="space-y-6">
            <h1 class="text-3xl font-bold">"Collapse"</h1>
            <p class="text-base-content/70">"Collapse is used for showing and hiding content"</p>

            <div class="space-y-4">
                <h2 class="text-xl font-semibold">"Basic Collapse with Focus"</h2>
                <div
                    tabindex="0"
                    class="collapse collapse-arrow border border-base-300 bg-base-100 rounded-box"
                >
                    <div class="collapse-title text-xl font-medium">"Focus me to see content"</div>
                    <div class="collapse-content">
                        <p>"tabindex=\"0\" attribute is necessary to make the div focusable"</p>
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Collapse with Checkbox"</h2>
                <div class="collapse bg-base-200">
                    <input
                        type="checkbox"
                        checked=checkbox_open.get_untracked()
                        on:change=move |ev| {
                            let checked = event_target_checked(&ev);
                            set_checkbox_open.set(checked);
                        }
                    />
                    <div class="collapse-title text-xl font-medium">
                        "Click me to show/hide content"
                    </div>
                    <div class="collapse-content">
                        <p>"hello"</p>
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Force Open"</h2>
                <div class="collapse collapse-open border border-base-300 bg-base-100 rounded-box">
                    <div class="collapse-title text-xl font-medium">
                        "I have collapse-open class"
                    </div>
                    <div class="collapse-content">
                        <p>"This collapse is always open"</p>
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Force Close"</h2>
                <div class="collapse collapse-close border border-base-300 bg-base-100 rounded-box">
                    <div class="collapse-title text-xl font-medium">
                        "I have collapse-close class"
                    </div>
                    <div class="collapse-content">
                        <p>"This collapse is always closed"</p>
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Collapse with Arrow Icon"</h2>
                <div
                    tabindex="0"
                    class="collapse collapse-arrow border border-base-300 bg-base-100 rounded-box"
                >
                    <div class="collapse-title text-xl font-medium">"I have arrow icon"</div>
                    <div class="collapse-content">
                        <p>"The arrow icon rotates when expanded"</p>
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Collapse with Plus Icon"</h2>
                <div
                    tabindex="0"
                    class="collapse collapse-plus border border-base-300 bg-base-100 rounded-box"
                >
                    <div class="collapse-title text-xl font-medium">"I have plus icon"</div>
                    <div class="collapse-content">
                        <p>"The plus icon changes to minus when expanded"</p>
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Nested Collapse"</h2>
                <div
                    tabindex="0"
                    class="collapse collapse-arrow border border-base-300 bg-base-100 rounded-box"
                >
                    <div class="collapse-title text-xl font-medium">"Parent Level"</div>
                    <div class="collapse-content">
                        <p class="mb-2">"This is the parent collapse content."</p>

                        <div
                            tabindex="0"
                            class="collapse collapse-arrow border border-base-300 bg-base-200 rounded-box"
                        >
                            <div class="collapse-title text-lg font-medium">"Child Level"</div>
                            <div class="collapse-content">
                                <p>"This is nested inside the parent collapse."</p>
                            </div>
                        </div>
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Custom Styled Collapse"</h2>
                <div
                    tabindex="0"
                    class="collapse bg-primary text-primary-content focus:bg-secondary focus:text-secondary-content"
                >
                    <div class="collapse-title">"Custom colors and styling"</div>
                    <div class="collapse-content">
                        <p>"You can customize the appearance using daisyUI color classes"</p>
                        <div class="mt-2">
                            <Button size=ButtonSize::Sm color=ButtonColor::Accent>
                                "Action Button"
                            </Button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}