use leptos::prelude::*;

#[component]
pub fn LabelDemo() -> impl IntoView {
    view! {
        // <div class="space-y-6">
        //     <h1 class="text-3xl font-bold">"Label"</h1>
        //     <p class="text-base-content/70">
        //         "Label is used to add a caption for form elements"
        //     </p>

        //     <div class="space-y-4">
        //         <h2 class="text-xl font-semibold">"Basic Label"</h2>
        //         <div class="form-control w-full max-w-xs">
        //             <Label class="label">
        //                 <LabelText>"What is your name?"</LabelText>
        //             </Label>
        //             <Input placeholder="Type here" class="input input-bordered w-full max-w-xs" />
        //         </div>

        //         <h2 class="text-xl font-semibold">"Label with Helper Text"</h2>
        //         <div class="form-control w-full max-w-xs">
        //             <Label class="label">
        //                 <LabelText>"What is your name?"</LabelText>
        //                 <LabelTextAlt>"Top Right label"</LabelTextAlt>
        //             </Label>
        //             <Input placeholder="Type here" class="input input-bordered w-full max-w-xs" />
        //             <Label class="label">
        //                 <LabelTextAlt>"Bottom Left label"</LabelTextAlt>
        //                 <LabelTextAlt>"Bottom Right label"</LabelTextAlt>
        //             </Label>
        //         </div>

        //         <h2 class="text-xl font-semibold">"Required Field"</h2>
        //         <div class="form-control w-full max-w-xs">
        //             <Label class="label">
        //                 <LabelText>
        //                     "Email Address "
        //                     <span class="text-error">"*"</span>
        //                 </LabelText>
        //             </Label>
        //             <Input
        //                 placeholder="email@example.com"
        //                 input_type="email"
        //                 class="input input-bordered w-full max-w-xs"
        //             />
        //             <Label class="label">
        //                 <LabelTextAlt class="text-error">"Required field"</LabelTextAlt>
        //             </Label>
        //         </div>

        //         <h2 class="text-xl font-semibold">"Label Colors"</h2>
        //         <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        //             <div class="form-control w-full max-w-xs">
        //                 <Label class="label">
        //                     <LabelText color=LabelColor::Primary>"Primary Label"</LabelText>
        //                 </Label>
        //                 <Input placeholder="Type here" class="input input-bordered w-full max-w-xs" />
        //             </div>
        //             <div class="form-control w-full max-w-xs">
        //                 <Label class="label">
        //                     <LabelText color=LabelColor::Secondary>"Secondary Label"</LabelText>
        //                 </Label>
        //                 <Input placeholder="Type here" class="input input-bordered w-full max-w-xs" />
        //             </div>
        //             <div class="form-control w-full max-w-xs">
        //                 <Label class="label">
        //                     <LabelText color=LabelColor::Accent>"Accent Label"</LabelText>
        //                 </Label>
        //                 <Input placeholder="Type here" class="input input-bordered w-full max-w-xs" />
        //             </div>
        //             <div class="form-control w-full max-w-xs">
        //                 <Label class="label">
        //                     <LabelText color=LabelColor::Info>"Info Label"</LabelText>
        //                 </Label>
        //                 <Input placeholder="Type here" class="input input-bordered w-full max-w-xs" />
        //             </div>
        //             <div class="form-control w-full max-w-xs">
        //                 <Label class="label">
        //                     <LabelText color=LabelColor::Success>"Success Label"</LabelText>
        //                 </Label>
        //                 <Input placeholder="Type here" class="input input-bordered w-full max-w-xs" />
        //             </div>
        //             <div class="form-control w-full max-w-xs">
        //                 <Label class="label">
        //                     <LabelText color=LabelColor::Warning>"Warning Label"</LabelText>
        //                 </Label>
        //                 <Input placeholder="Type here" class="input input-bordered w-full max-w-xs" />
        //             </div>
        //             <div class="form-control w-full max-w-xs">
        //                 <Label class="label">
        //                     <LabelText color=LabelColor::Error>"Error Label"</LabelText>
        //                 </Label>
        //                 <Input placeholder="Type here" class="input input-bordered w-full max-w-xs" />
        //             </div>
        //         </div>

        //         <h2 class="text-xl font-semibold">"Label with Icons"</h2>
        //         <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        //             <div class="form-control w-full max-w-xs">
        //                 <Label class="label">
        //                     <LabelText>
        //                         <Icon icon=icondata::AiUserOutlined class="w-4 h-4 mr-2" />
        //                         "Username"
        //                     </LabelText>
        //                 </Label>
        //                 <Input placeholder="Enter username" class="input input-bordered w-full max-w-xs" />
        //             </div>
        //             <div class="form-control w-full max-w-xs">
        //                 <Label class="label">
        //                     <LabelText>
        //                         <Icon icon=icondata::AiLockOutlined class="w-4 h-4 mr-2" />
        //                         "Password"
        //                     </LabelText>
        //                 </Label>
        //                 <Input
        //                     placeholder="Enter password"
        //                     input_type="password"
        //                     class="input input-bordered w-full max-w-xs"
        //                 />
        //             </div>
        //         </div>

        //         <h2 class="text-xl font-semibold">"Different Form Elements"</h2>
        //         <div class="space-y-4">
        //             <div class="form-control w-full max-w-xs">
        //                 <Label class="label">
        //                     <LabelText>"Select an option"</LabelText>
        //                 </Label>
        //                 <Select class="select select-bordered w-full max-w-xs">
        //                     <option disabled=true selected=true>"Pick one"</option>
        //                     <option>"Option 1"</option>
        //                     <option>"Option 2"</option>
        //                     <option>"Option 3"</option>
        //                 </Select>
        //             </div>

        //             <div class="form-control w-full max-w-xs">
        //                 <Label class="label">
        //                     <LabelText>"Your message"</LabelText>
        //                 </Label>
        //                 <Textarea
        //                     placeholder="Write your message here..."
        //                     class="textarea textarea-bordered w-full max-w-xs"
        //                 />
        //                 <Label class="label">
        //                     <LabelTextAlt>"Maximum 500 characters"</LabelTextAlt>
        //                 </Label>
        //             </div>

        //             <div class="form-control">
        //                 <Label class="label cursor-pointer justify-start gap-2 max-w-xs">
        //                     <Checkbox />
        //                     <LabelText>"Remember me"</LabelText>
        //                 </Label>
        //             </div>

        //             <div class="form-control">
        //                 <Label class="label cursor-pointer justify-start gap-2 max-w-xs">
        //                     <Radio name="radio-demo" />
        //                     <LabelText>"I agree to the terms and conditions"</LabelText>
        //                 </Label>
        //             </div>
        //         </div>

        //         <h2 class="text-xl font-semibold">"Validation States"</h2>
        //         <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        //             <div class="form-control w-full max-w-xs">
        //                 <Label class="label">
        //                     <LabelText color=LabelColor::Success>"Valid Input"</LabelText>
        //                 </Label>
        //                 <Input
        //                     placeholder="john@example.com"
        //                     class="input input-bordered input-success w-full max-w-xs"
        //                 />
        //                 <Label class="label">
        //                     <LabelTextAlt class="text-success">
        //                         <Icon icon=icondata::AiCheckCircleOutlined class="w-4 h-4 mr-1" />
        //                         "Email is valid"
        //                     </LabelTextAlt>
        //                 </Label>
        //             </div>

        //             <div class="form-control w-full max-w-xs">
        //                 <Label class="label">
        //                     <LabelText color=LabelColor::Warning>"Warning Input"</LabelText>
        //                 </Label>
        //                 <Input
        //                     placeholder="john@"
        //                     class="input input-bordered input-warning w-full max-w-xs"
        //                 />
        //                 <Label class="label">
        //                     <LabelTextAlt class="text-warning">
        //                         <Icon icon=icondata::AiExclamationCircleOutlined class="w-4 h-4 mr-1" />
        //                         "Email seems incomplete"
        //                     </LabelTextAlt>
        //                 </Label>
        //             </div>

        //             <div class="form-control w-full max-w-xs">
        //                 <Label class="label">
        //                     <LabelText color=LabelColor::Error>"Invalid Input"</LabelText>
        //                 </Label>
        //                 <Input
        //                     placeholder="invalid-email"
        //                     class="input input-bordered input-error w-full max-w-xs"
        //                 />
        //                 <Label class="label">
        //                     <LabelTextAlt class="text-error">
        //                         <Icon icon=icondata::AiCloseCircleOutlined class="w-4 h-4 mr-1" />
        //                         "Invalid email format"
        //                     </LabelTextAlt>
        //                 </Label>
        //             </div>
        //         </div>

        //         <h2 class="text-xl font-semibold">"Complex Form Example"</h2>
        //         <Card class="bg-base-100 shadow-xl max-w-md">
        //             <CardBody>
        //                 <h2 class="card-title">"Sign Up Form"</h2>
        //                 <div class="space-y-4">
        //                     <div class="form-control w-full">
        //                         <Label class="label">
        //                             <LabelText>
        //                                 "Full Name "
        //                                 <span class="text-error">"*"</span>
        //                             </LabelText>
        //                         </Label>
        //                         <Input
        //                             placeholder="John Doe"
        //                             class="input input-bordered w-full"
        //                         />
        //                     </div>

        //                     <div class="form-control w-full">
        //                         <Label class="label">
        //                             <LabelText>
        //                                 "Email Address "
        //                                 <span class="text-error">"*"</span>
        //                             </LabelText>
        //                             <LabelTextAlt>"We'll never share your email"</LabelTextAlt>
        //                         </Label>
        //                         <Input
        //                             placeholder="john@example.com"
        //                             input_type="email"
        //                             class="input input-bordered w-full"
        //                         />
        //                     </div>

        //                     <div class="form-control w-full">
        //                         <Label class="label">
        //                             <LabelText>"Password"</LabelText>
        //                             <LabelTextAlt>"Must be at least 8 characters"</LabelTextAlt>
        //                         </Label>
        //                         <Input
        //                             placeholder="••••••••"
        //                             input_type="password"
        //                             class="input input-bordered w-full"
        //                         />
        //                     </div>

        //                     <div class="form-control">
        //                         <Label class="label cursor-pointer justify-start gap-2">
        //                             <Checkbox />
        //                             <LabelText class="text-sm">
        //                                 "I agree to the "
        //                                 <a href="#" class="link link-primary">"Terms of Service"</a>
        //                                 " and "
        //                                 <a href="#" class="link link-primary">"Privacy Policy"</a>
        //                             </LabelText>
        //                         </Label>
        //                     </div>

        //                     <div class="form-control">
        //                         <Label class="label cursor-pointer justify-start gap-2">
        //                             <Checkbox />
        //                             <LabelText class="text-sm">
        //                                 "Subscribe to our newsletter for updates"
        //                             </LabelText>
        //                         </Label>
        //                     </div>

        //                     <Button color=ButtonColor::Primary class="w-full">
        //                         "Create Account"
        //                     </Button>
        //                 </div>
        //             </CardBody>
        //         </Card>
        //     </div>
        // </div>
    }
}
