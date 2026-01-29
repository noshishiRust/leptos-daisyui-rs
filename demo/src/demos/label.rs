use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;
use leptos_icons::Icon;

#[component]
pub fn LabelDemo() -> impl IntoView {
    view! {
        <ContentLayout
            title="Label"
            description="Label is used to add a caption for form elements"
        >
            <Section title="Basic Label">
                <div class="form-control w-full max-w-xs">
                    <Label class="label">
                        <LabelText>"What is your name?"</LabelText>
                    </Label>
                    <Input attr:placeholder="Type here" class="input input-bordered w-full max-w-xs" />
                </div>
            </Section>

            <Section title="Label with Helper Text">
                <div class="form-control w-full max-w-xs">
                    <Label class="label">
                        <LabelText>"What is your name?"</LabelText>
                        <LabelTextAlt>"Top Right label"</LabelTextAlt>
                    </Label>
                    <Input attr:placeholder="Type here" class="input input-bordered w-full max-w-xs" />
                    <Label class="label">
                        <LabelTextAlt>"Bottom Left label"</LabelTextAlt>
                        <LabelTextAlt>"Bottom Right label"</LabelTextAlt>
                    </Label>
                </div>
            </Section>

            <Section title="Required Field">
                <div class="form-control w-full max-w-xs">
                    <Label class="label">
                        <LabelText>
                            "Email Address "
                            <span class="text-error">"*"</span>
                        </LabelText>
                    </Label>
                    <Input
                        attr:placeholder="email@example.com"
                        attr:r#type="email"
                        class="input input-bordered w-full max-w-xs"
                    />
                    <Label class="label">
                        <LabelTextAlt class="text-error">"Required field"</LabelTextAlt>
                    </Label>
                </div>
            </Section>

            <Section title="Label Colors">
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <div class="form-control w-full max-w-xs">
                        <Label class="label">
                            <LabelText class="text-primary">"Primary Label"</LabelText>
                        </Label>
                        <Input attr:placeholder="Type here" class="input input-bordered w-full max-w-xs" />
                    </div>
                    <div class="form-control w-full max-w-xs">
                        <Label class="label">
                            <LabelText class="text-secondary">"Secondary Label"</LabelText>
                        </Label>
                        <Input attr:placeholder="Type here" class="input input-bordered w-full max-w-xs" />
                    </div>
                    <div class="form-control w-full max-w-xs">
                        <Label class="label">
                            <LabelText class="text-accent">"Accent Label"</LabelText>
                        </Label>
                        <Input attr:placeholder="Type here" class="input input-bordered w-full max-w-xs" />
                    </div>
                    <div class="form-control w-full max-w-xs">
                        <Label class="label">
                            <LabelText class="text-info">"Info Label"</LabelText>
                        </Label>
                        <Input attr:placeholder="Type here" class="input input-bordered w-full max-w-xs" />
                    </div>
                    <div class="form-control w-full max-w-xs">
                        <Label class="label">
                            <LabelText class="text-success">"Success Label"</LabelText>
                        </Label>
                        <Input attr:placeholder="Type here" class="input input-bordered w-full max-w-xs" />
                    </div>
                    <div class="form-control w-full max-w-xs">
                        <Label class="label">
                            <LabelText class="text-warning">"Warning Label"</LabelText>
                        </Label>
                        <Input attr:placeholder="Type here" class="input input-bordered w-full max-w-xs" />
                    </div>
                    <div class="form-control w-full max-w-xs">
                        <Label class="label">
                            <LabelText class="text-error">"Error Label"</LabelText>
                        </Label>
                        <Input attr:placeholder="Type here" class="input input-bordered w-full max-w-xs" />
                    </div>
                </div>
            </Section>

            <Section title="Label with Icons">
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <div class="form-control w-full max-w-xs">
                        <Label class="label">
                            <LabelText>
                                <span class="inline-flex items-center gap-2">
                                    <Icon icon=icondata::AiUserOutlined width="1rem" height="1rem" />
                                    "Username"
                                </span>
                            </LabelText>
                        </Label>
                        <Input attr:placeholder="Enter username" class="input input-bordered w-full max-w-xs" />
                    </div>
                    <div class="form-control w-full max-w-xs">
                        <Label class="label">
                            <LabelText>
                                <span class="inline-flex items-center gap-2">
                                    <Icon icon=icondata::AiLockOutlined width="1rem" height="1rem" />
                                    "Password"
                                </span>
                            </LabelText>
                        </Label>
                        <Input
                            attr:placeholder="Enter password"
                            attr:r#type="password"
                            class="input input-bordered w-full max-w-xs"
                        />
                    </div>
                </div>
            </Section>

            <Section title="Different Form Elements">
                <div class="space-y-4">
                    <div class="form-control w-full max-w-xs">
                        <Label class="label">
                            <LabelText>"Select an option"</LabelText>
                        </Label>
                        <Select class="select select-bordered w-full max-w-xs">
                            <option disabled selected>"Pick one"</option>
                            <option>"Option 1"</option>
                            <option>"Option 2"</option>
                            <option>"Option 3"</option>
                        </Select>
                    </div>

                    <div class="form-control w-full max-w-xs">
                        <Label class="label">
                            <LabelText>"Your message"</LabelText>
                        </Label>
                        <Textarea
                            attr:placeholder="Write your message here..."
                            class="textarea textarea-bordered w-full max-w-xs"
                        />
                        <Label class="label">
                            <LabelTextAlt>"Maximum 500 characters"</LabelTextAlt>
                        </Label>
                    </div>

                    <div class="form-control">
                        <Label class="label cursor-pointer justify-start gap-2 max-w-xs">
                            <Checkbox />
                            <LabelText>"Remember me"</LabelText>
                        </Label>
                    </div>

                    <div class="form-control">
                        <Label class="label cursor-pointer justify-start gap-2 max-w-xs">
                            <Radio attr:name="radio-demo" />
                            <LabelText>"I agree to the terms and conditions"</LabelText>
                        </Label>
                    </div>
                </div>
            </Section>

            <Section title="Validation States">
                <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                    <div class="form-control w-full max-w-xs">
                        <Label class="label">
                            <LabelText class="text-success">"Valid Input"</LabelText>
                        </Label>
                        <Input
                            attr:placeholder="john@example.com"
                            class="input input-bordered input-success w-full max-w-xs"
                        />
                        <Label class="label">
                            <LabelTextAlt class="text-success">
                                <span class="inline-flex items-center gap-1">
                                    <Icon icon=icondata::AiCheckCircleOutlined width="1rem" height="1rem" />
                                    "Email is valid"
                                </span>
                            </LabelTextAlt>
                        </Label>
                    </div>

                    <div class="form-control w-full max-w-xs">
                        <Label class="label">
                            <LabelText class="text-warning">"Warning Input"</LabelText>
                        </Label>
                        <Input
                            attr:placeholder="john@"
                            class="input input-bordered input-warning w-full max-w-xs"
                        />
                        <Label class="label">
                            <LabelTextAlt class="text-warning">
                                <span class="inline-flex items-center gap-1">
                                    <Icon icon=icondata::AiExclamationCircleOutlined width="1rem" height="1rem" />
                                    "Email seems incomplete"
                                </span>
                            </LabelTextAlt>
                        </Label>
                    </div>

                    <div class="form-control w-full max-w-xs">
                        <Label class="label">
                            <LabelText class="text-error">"Invalid Input"</LabelText>
                        </Label>
                        <Input
                            attr:placeholder="invalid-email"
                            class="input input-bordered input-error w-full max-w-xs"
                        />
                        <Label class="label">
                            <LabelTextAlt class="text-error">
                                <span class="inline-flex items-center gap-1">
                                    <Icon icon=icondata::AiCloseCircleOutlined width="1rem" height="1rem" />
                                    "Invalid email format"
                                </span>
                            </LabelTextAlt>
                        </Label>
                    </div>
                </div>
            </Section>

            <Section title="Complex Form Example">
                <Card class="bg-base-100 shadow-xl max-w-md">
                    <CardBody>
                        <h2 class="card-title">"Sign Up Form"</h2>
                        <div class="space-y-4">
                            <div class="form-control w-full">
                                <Label class="label">
                                    <LabelText>
                                        "Full Name "
                                        <span class="text-error">"*"</span>
                                    </LabelText>
                                </Label>
                                <Input
                                    attr:placeholder="John Doe"
                                    class="input input-bordered w-full"
                                />
                            </div>

                            <div class="form-control w-full">
                                <Label class="label">
                                    <LabelText>
                                        "Email Address "
                                        <span class="text-error">"*"</span>
                                    </LabelText>
                                    <LabelTextAlt>"We'll never share your email"</LabelTextAlt>
                                </Label>
                                <Input
                                    attr:placeholder="john@example.com"
                                    attr:r#type="email"
                                    class="input input-bordered w-full"
                                />
                            </div>

                            <div class="form-control w-full">
                                <Label class="label">
                                    <LabelText>"Password"</LabelText>
                                    <LabelTextAlt>"Must be at least 8 characters"</LabelTextAlt>
                                </Label>
                                <Input
                                    attr:placeholder="••••••••"
                                    attr:r#type="password"
                                    class="input input-bordered w-full"
                                />
                            </div>

                            <div class="form-control">
                                <Label class="label cursor-pointer justify-start gap-2">
                                    <Checkbox />
                                    <LabelText class="text-sm">
                                        "I agree to the "
                                        <a href="#" class="link link-primary">"Terms of Service"</a>
                                        " and "
                                        <a href="#" class="link link-primary">"Privacy Policy"</a>
                                    </LabelText>
                                </Label>
                            </div>

                            <div class="form-control">
                                <Label class="label cursor-pointer justify-start gap-2">
                                    <Checkbox />
                                    <LabelText class="text-sm">
                                        "Subscribe to our newsletter for updates"
                                    </LabelText>
                                </Label>
                            </div>

                            <Button color=ButtonColor::Primary class="w-full">
                                "Create Account"
                            </Button>
                        </div>
                    </CardBody>
                </Card>
            </Section>
        </ContentLayout>
    }
}
