use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn ValidatorDemo() -> impl IntoView {
    let (email_value, set_email_value) = signal("".to_string());
    let (password_value, set_password_value) = signal("".to_string());
    let (username_value, set_username_value) = signal("".to_string());

    let email_valid = move || {
        let email = email_value.get();
        if email.is_empty() {
            return false;
        }
        email.contains("@") && email.contains(".")
    };

    let password_valid = move || {
        let password = password_value.get();
        if password.is_empty() {
            return false;
        }
        password.len() >= 8
            && password.chars().any(|c| c.is_uppercase())
            && password.chars().any(|c| c.is_numeric())
    };

    let username_valid = move || {
        let username = username_value.get();
        if username.is_empty() {
            return false;
        }
        username.len() >= 3 && username.chars().all(|c| c.is_alphanumeric() || c == '_')
    };

    view! {
        <ContentLayout
            title="Validator"
            description="Form validation with visual feedback and error messages"
        >
            <Section title="Basic Form Validation">
                <div class="max-w-2xl space-y-4">
                    <div class="form-control">
                        <Label class="label">
                            <LabelText>"Username"</LabelText>
                        </Label>
                        <Validator>
                            <Input
                                attr:placeholder="Enter username (min 3 chars)"
                                attr:value=username_value
                                on:input=move |ev| {
                                    set_username_value.set(event_target_value(&ev));
                                }
                            />
                        </Validator>
                        <ValidatorHint>
                            {move || {
                                if !username_value.get().is_empty() && !username_valid() {
                                    "Username must be at least 3 characters and contain only letters, numbers, or underscores"
                                } else {
                                    ""
                                }
                            }}

                        </ValidatorHint>
                    </div>

                    <div class="form-control">
                        <Label class="label">
                            <LabelText>"Email"</LabelText>
                        </Label>
                        <Validator>
                            <Input
                                attr:r#type="email"
                                attr:placeholder="Enter email address"
                                attr:value=email_value
                                on:input=move |ev| {
                                    set_email_value.set(event_target_value(&ev));
                                }
                            />
                        </Validator>
                        <ValidatorHint>
                            {move || {
                                if !email_value.get().is_empty() && !email_valid() {
                                    "Please enter a valid email address"
                                } else {
                                    ""
                                }
                            }}

                        </ValidatorHint>
                    </div>

                    <div class="form-control">
                        <Label class="label">
                            <LabelText>"Password"</LabelText>
                        </Label>
                        <Validator>
                            <Input
                                attr:r#type="password"
                                attr:placeholder="Enter password"
                                attr:value=password_value
                                on:input=move |ev| {
                                    set_password_value.set(event_target_value(&ev));
                                }
                            />
                        </Validator>
                        <ValidatorHint>
                            {move || {
                                if !password_value.get().is_empty() && !password_valid() {
                                    "Password must be at least 8 characters with uppercase letter and number"
                                } else {
                                    ""
                                }
                            }}

                        </ValidatorHint>
                    </div>

                    <div>
                        <Button color=ButtonColor::Primary>
                            "Submit"
                        </Button>
                        <div class="text-sm text-base-content/60 mt-2">
                            {move || {
                                if email_valid() && password_valid() && username_valid() {
                                    "All fields valid! Ready to submit."
                                } else {
                                    "Please fill in all fields correctly to submit."
                                }
                            }}

                        </div>
                    </div>
                </div>
            </Section>

            <Section title="Static Validation States">
                <div class="grid grid-cols-1 md:grid-cols-2 gap-6 max-w-2xl">
                    <div class="form-control">
                        <Label class="label">
                            <LabelText>"Valid Input"</LabelText>
                        </Label>
                        <Validator>
                            <Input attr:value="user@example.com" />
                        </Validator>
                        <ValidatorHint>"This is a valid email address"</ValidatorHint>
                    </div>

                    <div class="form-control">
                        <Label class="label">
                            <LabelText>"Invalid Input"</LabelText>
                        </Label>
                        <Validator>
                            <Input attr:value="invalid-email" />
                        </Validator>
                        <ValidatorHint>"Please enter a valid email address"</ValidatorHint>
                    </div>
                </div>
            </Section>

            <Section title="Usage Example">
                <div class="mockup-code text-sm max-w-3xl">
                    <pre data-prefix="1">
                        <code>"<Validator>"</code>
                    </pre>
                    <pre data-prefix="2">
                        <code>
                            {"  <Input attr:value=signal on:input=handler />"}
                        </code>
                    </pre>
                    <pre data-prefix="3">
                        <code>"</Validator>"</code>
                    </pre>
                    <pre data-prefix="4">
                        <code>
                            {"<ValidatorHint>\"Error message\"</ValidatorHint>"}
                        </code>
                    </pre>
                </div>
            </Section>
        </ContentLayout>
    }
}
