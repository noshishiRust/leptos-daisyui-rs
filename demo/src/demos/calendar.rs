use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn CalendarDemo() -> impl IntoView {
    view! {
        <ContentLayout
            title="Calendar"
            description="Styling wrapper for calendar libraries (Cally, Pikaday, React Day Picker)"
        >
            <Section title="Native HTML Date Input">
                <div class="flex justify-center">
                    <Calendar class="cally">
                        <input type="date" class="input input-bordered" />
                    </Calendar>
                </div>
            </Section>

            <Section title="Date Range">
                <div class="flex justify-center gap-4">
                    <Calendar class="cally">
                        <div>
                            <label class="label">
                                <span class="label-text">"Start Date"</span>
                            </label>
                            <input type="date" class="input input-bordered" />
                        </div>
                    </Calendar>
                    <Calendar class="cally">
                        <div>
                            <label class="label">
                                <span class="label-text">"End Date"</span>
                            </label>
                            <input type="date" class="input input-bordered" />
                        </div>
                    </Calendar>
                </div>
            </Section>

            <Section title="With Card">
                <div class="flex justify-center">
                    <Card class="w-96 bg-base-200 shadow-xl">
                        <CardBody>
                            <h2 class="card-title">"Select a Date"</h2>
                            <Calendar class="cally">
                                <input type="date" class="input input-bordered w-full" />
                            </Calendar>
                            <div class="card-actions justify-end mt-4">
                                <Button color=ButtonColor::Primary>"Confirm"</Button>
                            </div>
                        </CardBody>
                    </Card>
                </div>
            </Section>

            <Section title="Datetime Input">
                <div class="flex justify-center">
                    <Calendar class="cally">
                        <div class="space-y-2">
                            <label class="label">
                                <span class="label-text">"Select Date and Time"</span>
                            </label>
                            <input type="datetime-local" class="input input-bordered w-full" />
                        </div>
                    </Calendar>
                </div>
            </Section>

            <Section title="Month Picker">
                <div class="flex justify-center">
                    <Calendar class="cally">
                        <div class="space-y-2">
                            <label class="label">
                                <span class="label-text">"Select Month"</span>
                            </label>
                            <input type="month" class="input input-bordered" />
                        </div>
                    </Calendar>
                </div>
            </Section>

            <Section title="Week Picker">
                <div class="flex justify-center">
                    <Calendar class="cally">
                        <div class="space-y-2">
                            <label class="label">
                                <span class="label-text">"Select Week"</span>
                            </label>
                            <input type="week" class="input input-bordered" />
                        </div>
                    </Calendar>
                </div>
            </Section>

            <Section title="Supported Libraries">
                <div class="alert alert-info">
                    <div class="space-y-2">
                        <p>
                            <strong>"This component provides styling for:"</strong>
                        </p>
                        <ul class="list-disc list-inside space-y-1">
                            <li>
                                <strong>"Cally"</strong>
                                " - Universal web component (recommended)"
                            </li>
                            <li>
                                <strong>"Pikaday"</strong>
                                " - JavaScript date picker library"
                            </li>
                            <li>
                                <strong>"React Day Picker"</strong>
                                " - React date picker component"
                            </li>
                            <li>
                                <strong>"Native HTML"</strong>
                                " - Built-in date/time inputs (shown above)"
                            </li>
                        </ul>
                        <p class="text-sm mt-2">
                            "For JavaScript library integration, use leptos_use or custom JavaScript interop."
                        </p>
                    </div>
                </div>
            </Section>

            <Section title="Integration Example">
                <div class="mockup-code">
                    <pre data-prefix="$"><code>"// Using Cally web component"</code></pre>
                    <pre data-prefix=">"><code>"<Calendar>"</code></pre>
                    <pre data-prefix=">"><code>"  <calendar-date></calendar-date>"</code></pre>
                    <pre data-prefix=">"><code>"</Calendar>"</code></pre>
                    <pre data-prefix=" "><code></code></pre>
                    <pre data-prefix="$"><code>"// Using native HTML"</code></pre>
                    <pre data-prefix=">"><code>"<Calendar class=\"cally\">"</code></pre>
                    <pre data-prefix=">"><code>"  <input type=\"date\" />"</code></pre>
                    <pre data-prefix=">"><code>"</Calendar>"</code></pre>
                </div>
            </Section>
        </ContentLayout>
    }
}
