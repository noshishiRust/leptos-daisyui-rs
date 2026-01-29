use leptos::prelude::*;

#[component]
pub fn FieldsetDemo() -> impl IntoView {
    view! {
        // <div class="space-y-6">
        //     <h1 class="text-3xl font-bold">"Fieldset"</h1>
        //     <p class="text-base-content/70">
        //         "Fieldset groups related form controls and provides a legend"
        //     </p>

        //     <div class="space-y-4">
        //         <h2 class="text-xl font-semibold">"Basic Fieldset"</h2>
        //         <Fieldset>
        //             <FieldsetLegend>"Personal Information"</FieldsetLegend>
        //             <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        //                 <div class="form-control">
        //                     <Label class="label">
        //                         <LabelText>"First Name"</LabelText>
        //                     </Label>
        //                     <Input placeholder="John" class="input input-bordered" />
        //                 </div>
        //                 <div class="form-control">
        //                     <Label class="label">
        //                         <LabelText>"Last Name"</LabelText>
        //                     </Label>
        //                     <Input placeholder="Doe" class="input input-bordered" />
        //                 </div>
        //                 <div class="form-control md:col-span-2">
        //                     <Label class="label">
        //                         <LabelText>"Email"</LabelText>
        //                     </Label>
        //                     <Input placeholder="john.doe@example.com" input_type="email" class="input input-bordered" />
        //                 </div>
        //             </div>
        //         </Fieldset>

        //         <h2 class="text-xl font-semibold">"Multiple Fieldsets"</h2>
        //         <div class="space-y-6">
        //             <Fieldset>
        //                 <FieldsetLegend>"Account Settings"</FieldsetLegend>
        //                 <div class="space-y-4">
        //                     <div class="form-control">
        //                         <Label class="label">
        //                             <LabelText>"Username"</LabelText>
        //                         </Label>
        //                         <Input placeholder="johndoe" class="input input-bordered" />
        //                     </div>
        //                     <div class="form-control">
        //                         <Label class="label">
        //                             <LabelText>"Password"</LabelText>
        //                         </Label>
        //                         <Input placeholder="••••••••" input_type="password" class="input input-bordered" />
        //                     </div>
        //                     <div class="form-control">
        //                         <Label class="label cursor-pointer justify-start gap-2">
        //                             <Checkbox />
        //                             <LabelText>"Enable two-factor authentication"</LabelText>
        //                         </Label>
        //                     </div>
        //                 </div>
        //             </Fieldset>

        //             <Fieldset>
        //                 <FieldsetLegend>"Preferences"</FieldsetLegend>
        //                 <div class="space-y-4">
        //                     <div class="form-control">
        //                         <Label class="label">
        //                             <LabelText>"Language"</LabelText>
        //                         </Label>
        //                         <Select class="select select-bordered">
        //                             <option>"English"</option>
        //                             <option>"Spanish"</option>
        //                             <option>"French"</option>
        //                             <option>"German"</option>
        //                         </Select>
        //                     </div>
        //                     <div class="form-control">
        //                         <Label class="label">
        //                             <LabelText>"Theme"</LabelText>
        //                         </Label>
        //                         <div class="flex gap-4">
        //                             <Label class="label cursor-pointer justify-start gap-2">
        //                                 <Radio name="theme" />
        //                                 <LabelText>"Light"</LabelText>
        //                             </Label>
        //                             <Label class="label cursor-pointer justify-start gap-2">
        //                                 <Radio name="theme" />
        //                                 <LabelText>"Dark"</LabelText>
        //                             </Label>
        //                             <Label class="label cursor-pointer justify-start gap-2">
        //                                 <Radio name="theme" />
        //                                 <LabelText>"Auto"</LabelText>
        //                             </Label>
        //                         </div>
        //                     </div>
        //                     <div class="form-control">
        //                         <Label class="label cursor-pointer justify-start gap-2">
        //                             <Checkbox />
        //                             <LabelText>"Enable notifications"</LabelText>
        //                         </Label>
        //                     </div>
        //                 </div>
        //             </Fieldset>
        //         </div>

        //         <h2 class="text-xl font-semibold">"Disabled Fieldset"</h2>
        //         <Fieldset disabled=true>
        //             <FieldsetLegend>"Billing Information (Coming Soon)"</FieldsetLegend>
        //             <div class="space-y-4">
        //                 <div class="form-control">
        //                     <Label class="label">
        //                         <LabelText>"Credit Card Number"</LabelText>
        //                     </Label>
        //                     <Input placeholder="**** **** **** ****" class="input input-bordered" disabled=true />
        //                 </div>
        //                 <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        //                     <div class="form-control">
        //                         <Label class="label">
        //                             <LabelText>"Expiry Date"</LabelText>
        //                         </Label>
        //                         <Input placeholder="MM/YY" class="input input-bordered" disabled=true />
        //                     </div>
        //                     <div class="form-control">
        //                         <Label class="label">
        //                             <LabelText>"CVV"</LabelText>
        //                         </Label>
        //                         <Input placeholder="123" class="input input-bordered" disabled=true />
        //                     </div>
        //                 </div>
        //                 <div class="form-control">
        //                     <Label class="label cursor-pointer justify-start gap-2">
        //                         <Checkbox disabled=true />
        //                         <LabelText>"Save payment method"</LabelText>
        //                     </Label>
        //                 </div>
        //             </div>
        //         </Fieldset>

        //         <h2 class="text-xl font-semibold">"Nested Fieldsets"</h2>
        //         <Fieldset>
        //             <FieldsetLegend>"Contact Information"</FieldsetLegend>
        //             <div class="space-y-6">
        //                 <Fieldset>
        //                     <FieldsetLegend>"Primary Address"</FieldsetLegend>
        //                     <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        //                         <div class="form-control md:col-span-2">
        //                             <Label class="label">
        //                                 <LabelText>"Street Address"</LabelText>
        //                             </Label>
        //                             <Input placeholder="123 Main Street" class="input input-bordered" />
        //                         </div>
        //                         <div class="form-control">
        //                             <Label class="label">
        //                                 <LabelText>"City"</LabelText>
        //                             </Label>
        //                             <Input placeholder="New York" class="input input-bordered" />
        //                         </div>
        //                         <div class="form-control">
        //                             <Label class="label">
        //                                 <LabelText>"ZIP Code"</LabelText>
        //                             </Label>
        //                             <Input placeholder="10001" class="input input-bordered" />
        //                         </div>
        //                     </div>
        //                 </Fieldset>

        //                 <Fieldset>
        //                     <FieldsetLegend>"Phone Numbers"</FieldsetLegend>
        //                     <div class="space-y-4">
        //                         <div class="form-control">
        //                             <Label class="label">
        //                                 <LabelText>"Home"</LabelText>
        //                             </Label>
        //                             <Input placeholder="(555) 123-4567" input_type="tel" class="input input-bordered" />
        //                         </div>
        //                         <div class="form-control">
        //                             <Label class="label">
        //                                 <LabelText>"Mobile"</LabelText>
        //                             </Label>
        //                             <Input placeholder="(555) 987-6543" input_type="tel" class="input input-bordered" />
        //                         </div>
        //                         <div class="form-control">
        //                             <Label class="label">
        //                                 <LabelText>"Work"</LabelText>
        //                             </Label>
        //                             <Input placeholder="(555) 555-0123" input_type="tel" class="input input-bordered" />
        //                         </div>
        //                     </div>
        //                 </Fieldset>
        //             </div>
        //         </Fieldset>

        //         <h2 class="text-xl font-semibold">"Form with Fieldsets"</h2>
        //         <Card class="bg-base-100 shadow-xl">
        //             <CardBody>
        //                 <h2 class="card-title">"User Registration"</h2>
        //                 <form class="space-y-6">
        //                     <Fieldset>
        //                         <FieldsetLegend>
        //                             <Icon icon=icondata::AiUserOutlined class="w-4 h-4 mr-2" />
        //                             "Basic Information"
        //                         </FieldsetLegend>
        //                         <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        //                             <div class="form-control">
        //                                 <Label class="label">
        //                                     <LabelText>
        //                                         "First Name "
        //                                         <span class="text-error">"*"</span>
        //                                     </LabelText>
        //                                 </Label>
        //                                 <Input placeholder="John" class="input input-bordered" required=true />
        //                             </div>
        //                             <div class="form-control">
        //                                 <Label class="label">
        //                                     <LabelText>
        //                                         "Last Name "
        //                                         <span class="text-error">"*"</span>
        //                                     </LabelText>
        //                                 </Label>
        //                                 <Input placeholder="Doe" class="input input-bordered" required=true />
        //                             </div>
        //                             <div class="form-control md:col-span-2">
        //                                 <Label class="label">
        //                                     <LabelText>
        //                                         "Email Address "
        //                                         <span class="text-error">"*"</span>
        //                                     </LabelText>
        //                                 </Label>
        //                                 <Input placeholder="john.doe@example.com" input_type="email" class="input input-bordered" required=true />
        //                             </div>
        //                             <div class="form-control md:col-span-2">
        //                                 <Label class="label">
        //                                     <LabelText>"Date of Birth"</LabelText>
        //                                 </Label>
        //                                 <Input input_type="date" class="input input-bordered" />
        //                             </div>
        //                         </div>
        //                     </Fieldset>

        //                     <Fieldset>
        //                         <FieldsetLegend>
        //                             <Icon icon=icondata::AiLockOutlined class="w-4 h-4 mr-2" />
        //                             "Security"
        //                         </FieldsetLegend>
        //                         <div class="space-y-4">
        //                             <div class="form-control">
        //                                 <Label class="label">
        //                                     <LabelText>
        //                                         "Password "
        //                                         <span class="text-error">"*"</span>
        //                                     </LabelText>
        //                                     <LabelTextAlt>"At least 8 characters"</LabelTextAlt>
        //                                 </Label>
        //                                 <Input placeholder="••••••••" input_type="password" class="input input-bordered" required=true />
        //                             </div>
        //                             <div class="form-control">
        //                                 <Label class="label">
        //                                     <LabelText>
        //                                         "Confirm Password "
        //                                         <span class="text-error">"*"</span>
        //                                     </LabelText>
        //                                 </Label>
        //                                 <Input placeholder="••••••••" input_type="password" class="input input-bordered" required=true />
        //                             </div>
        //                             <div class="form-control">
        //                                 <Label class="label cursor-pointer justify-start gap-2">
        //                                     <Checkbox />
        //                                     <LabelText>"Enable two-factor authentication (recommended)"</LabelText>
        //                                 </Label>
        //                             </div>
        //                         </div>
        //                     </Fieldset>

        //                     <Fieldset>
        //                         <FieldsetLegend>
        //                             <Icon icon=icondata::AiSettingOutlined class="w-4 h-4 mr-2" />
        //                             "Preferences"
        //                         </FieldsetLegend>
        //                         <div class="space-y-4">
        //                             <div class="form-control">
        //                                 <Label class="label">
        //                                     <LabelText>"Newsletter Subscription"</LabelText>
        //                                 </Label>
        //                                 <div class="space-y-2">
        //                                     <Label class="label cursor-pointer justify-start gap-2">
        //                                         <Checkbox />
        //                                         <LabelText>"Weekly newsletter"</LabelText>
        //                                     </Label>
        //                                     <Label class="label cursor-pointer justify-start gap-2">
        //                                         <Checkbox />
        //                                         <LabelText>"Product updates"</LabelText>
        //                                     </Label>
        //                                     <Label class="label cursor-pointer justify-start gap-2">
        //                                         <Checkbox />
        //                                         <LabelText>"Special offers"</LabelText>
        //                                     </Label>
        //                                 </div>
        //                             </div>
        //                             <div class="form-control">
        //                                 <Label class="label">
        //                                     <LabelText>"Communication Preference"</LabelText>
        //                                 </Label>
        //                                 <div class="flex gap-4">
        //                                     <Label class="label cursor-pointer justify-start gap-2">
        //                                         <Radio name="communication" />
        //                                         <LabelText>"Email only"</LabelText>
        //                                     </Label>
        //                                     <Label class="label cursor-pointer justify-start gap-2">
        //                                         <Radio name="communication" />
        //                                         <LabelText>"SMS only"</LabelText>
        //                                     </Label>
        //                                     <Label class="label cursor-pointer justify-start gap-2">
        //                                         <Radio name="communication" />
        //                                         <LabelText>"Both"</LabelText>
        //                                     </Label>
        //                                 </div>
        //                             </div>
        //                         </div>
        //                     </Fieldset>

        //                     <Fieldset>
        //                         <FieldsetLegend>
        //                             <Icon icon=icondata::AiFileTextOutlined class="w-4 h-4 mr-2" />
        //                             "Terms and Conditions"
        //                         </FieldsetLegend>
        //                         <div class="space-y-2">
        //                             <Label class="label cursor-pointer justify-start gap-2">
        //                                 <Checkbox required=true />
        //                                 <LabelText>
        //                                     "I agree to the "
        //                                     <Link href="#" class="link link-primary">"Terms of Service"</Link>
        //                                     " "
        //                                     <span class="text-error">"*"</span>
        //                                 </LabelText>
        //                             </Label>
        //                             <Label class="label cursor-pointer justify-start gap-2">
        //                                 <Checkbox required=true />
        //                                 <LabelText>
        //                                     "I accept the "
        //                                     <Link href="#" class="link link-primary">"Privacy Policy"</Link>
        //                                     " "
        //                                     <span class="text-error">"*"</span>
        //                                 </LabelText>
        //                             </Label>
        //                             <Label class="label cursor-pointer justify-start gap-2">
        //                                 <Checkbox />
        //                                 <LabelText>
        //                                     "I consent to receive marketing communications"
        //                                 </LabelText>
        //                             </Label>
        //                         </div>
        //                     </Fieldset>

        //                     <div class="card-actions justify-end">
        //                         <Button style=ButtonStyle::Ghost>
        //                             "Cancel"
        //                         </Button>
        //                         <Button color=ButtonColor::Primary input_type="submit">
        //                             "Create Account"
        //                         </Button>
        //                     </div>
        //                 </form>
        //             </CardBody>
        //         </Card>
        //     </div>
        // </div>
    }
}
