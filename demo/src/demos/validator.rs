use leptos::prelude::*;

#[component]
pub fn ValidatorDemo() -> impl IntoView {
    // let (email_value, set_email_value) = signal("".to_string());
    // let (password_value, set_password_value) = signal("".to_string());
    // let (confirm_password_value, set_confirm_password_value) = signal("".to_string());
    // let (username_value, set_username_value) = signal("".to_string());

    // let email_valid = move || {
    //     let email = email_value();
    //     if email.is_empty() {
    //         return None;
    //     }
    //     if email.contains("@") && email.contains(".") {
    //         Some(true)
    //     } else {
    //         Some(false)
    //     }
    // };

    // let password_valid = move || {
    //     let password = password_value();
    //     if password.is_empty() {
    //         return None;
    //     }
    //     if password.len() >= 8
    //         && password.chars().any(|c| c.is_uppercase())
    //         && password.chars().any(|c| c.is_numeric())
    //     {
    //         Some(true)
    //     } else {
    //         Some(false)
    //     }
    // };

    // let passwords_match = move || {
    //     let password = password_value();
    //     let confirm = confirm_password_value();
    //     if password.is_empty() || confirm.is_empty() {
    //         return None;
    //     }
    //     Some(password == confirm)
    // };

    // let username_valid = move || {
    //     let username = username_value();
    //     if username.is_empty() {
    //         return None;
    //     }
    //     if username.len() >= 3 && username.chars().all(|c| c.is_alphanumeric() || c == '_') {
    //         Some(true)
    //     } else {
    //         Some(false)
    //     }
    // };

    view! {
        // <div class="space-y-6">
        //     <h1 class="text-3xl font-bold">"Validator"</h1>
        //     <p class="text-base-content/70">
        //         "Validator components for form validation and feedback"
        //     </p>

        //     <div class="space-y-4">
        //         <h2 class="text-xl font-semibold">"Input Validation"</h2>
        //         <Card class="bg-base-100 shadow-xl">
        //             <CardBody>
        //                 <h3 class="card-title mb-4">"Registration Form"</h3>
        //                 <form class="space-y-4">
        //                     <div class="form-control">
        //                         <Label class="label">
        //                             <LabelText>"Username"</LabelText>
        //                         </Label>
        //                         <Input
        //                             placeholder="Enter username"
        //                             class=move || match username_valid() {
        //                                 Some(true) => "input input-bordered input-success",
        //                                 Some(false) => "input input-bordered input-error",
        //                                 None => "input input-bordered"
        //                             }
        //                             value=username_value
        //                             on:input=move |ev| {
        //                                 set_username_value(event_target_value(&ev));
        //                             }
        //                         />
        //                         <Validator
        //                             valid=username_valid()
        //                             success_message="Username is available"
        //                             error_message="Username must be at least 3 characters and contain only letters, numbers, or underscores"
        //                         />
        //                     </div>

        //                     <div class="form-control">
        //                         <Label class="label">
        //                             <LabelText>"Email"</LabelText>
        //                         </Label>
        //                         <Input
        //                             input_type="email"
        //                             placeholder="Enter email address"
        //                             class=move || match email_valid() {
        //                                 Some(true) => "input input-bordered input-success",
        //                                 Some(false) => "input input-bordered input-error",
        //                                 None => "input input-bordered"
        //                             }
        //                             value=email_value
        //                             on:input=move |ev| {
        //                                 set_email_value(event_target_value(&ev));
        //                             }
        //                         />
        //                         <Validator
        //                             valid=email_valid()
        //                             success_message="Valid email address"
        //                             error_message="Please enter a valid email address"
        //                         />
        //                     </div>

        //                     <div class="form-control">
        //                         <Label class="label">
        //                             <LabelText>"Password"</LabelText>
        //                         </Label>
        //                         <Input
        //                             input_type="password"
        //                             placeholder="Enter password"
        //                             class=move || match password_valid() {
        //                                 Some(true) => "input input-bordered input-success",
        //                                 Some(false) => "input input-bordered input-error",
        //                                 None => "input input-bordered"
        //                             }
        //                             value=password_value
        //                             on:input=move |ev| {
        //                                 set_password_value(event_target_value(&ev));
        //                             }
        //                         />
        //                         <Validator
        //                             valid=password_valid()
        //                             success_message="Strong password"
        //                             error_message="Password must be at least 8 characters with uppercase letter and number"
        //                         />
        //                     </div>

        //                     <div class="form-control">
        //                         <Label class="label">
        //                             <LabelText>"Confirm Password"</LabelText>
        //                         </Label>
        //                         <Input
        //                             input_type="password"
        //                             placeholder="Confirm password"
        //                             class=move || match passwords_match() {
        //                                 Some(true) => "input input-bordered input-success",
        //                                 Some(false) => "input input-bordered input-error",
        //                                 None => "input input-bordered"
        //                             }
        //                             value=confirm_password_value
        //                             on:input=move |ev| {
        //                                 set_confirm_password_value(event_target_value(&ev));
        //                             }
        //                         />
        //                         <Validator
        //                             valid=passwords_match()
        //                             success_message="Passwords match"
        //                             error_message="Passwords do not match"
        //                         />
        //                     </div>

        //                     <div class="card-actions">
        //                         <Button
        //                             color=ButtonColor::Primary
        //                             disabled=move || {
        //                                 email_valid() != Some(true) ||
        //                                 password_valid() != Some(true) ||
        //                                 passwords_match() != Some(true) ||
        //                                 username_valid() != Some(true)
        //                             }
        //                         >
        //                             "Create Account"
        //                         </Button>
        //                     </div>
        //                 </form>
        //             </CardBody>
        //         </Card>

        //         <h2 class="text-xl font-semibold">"Validation States"</h2>
        //         <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        //             <Card class="bg-base-100 shadow-xl">
        //                 <CardBody>
        //                     <h3 class="card-title text-lg text-success">"Valid State"</h3>
        //                     <div class="form-control">
        //                         <Input
        //                             placeholder="Valid input"
        //                             class="input input-bordered input-success"
        //                             value="user@example.com"
        //                         />
        //                         <Validator
        //                             valid=Some(true)
        //                             success_message="This field is valid"
        //                         />
        //                     </div>
        //                 </CardBody>
        //             </Card>

        //             <Card class="bg-base-100 shadow-xl">
        //                 <CardBody>
        //                     <h3 class="card-title text-lg text-error">"Invalid State"</h3>
        //                     <div class="form-control">
        //                         <Input
        //                             placeholder="Invalid input"
        //                             class="input input-bordered input-error"
        //                             value="invalid-email"
        //                         />
        //                         <Validator
        //                             valid=Some(false)
        //                             error_message="This field contains errors"
        //                         />
        //                     </div>
        //                 </CardBody>
        //             </Card>

        //             <Card class="bg-base-100 shadow-xl">
        //                 <CardBody>
        //                     <h3 class="card-title text-lg">"Neutral State"</h3>
        //                     <div class="form-control">
        //                         <Input
        //                             placeholder="Not yet validated"
        //                             class="input input-bordered"
        //                         />
        //                         <Validator
        //                             valid=None
        //                             info_message="Enter a value to validate"
        //                         />
        //                     </div>
        //                 </CardBody>
        //             </Card>
        //         </div>

        //         <h2 class="text-xl font-semibold">"Complex Validation"</h2>
        //         <Card class="bg-base-100 shadow-xl">
        //             <CardBody>
        //                 <h3 class="card-title mb-4">"Password Strength Checker"</h3>
        //                 <div class="form-control">
        //                     <Label class="label">
        //                         <LabelText>"New Password"</LabelText>
        //                     </Label>
        //                     <Input
        //                         input_type="password"
        //                         placeholder="Enter a strong password"
        //                         class="input input-bordered"
        //                     />

        //                     <div class="mt-4 space-y-2">
        //                         <h4 class="font-semibold text-sm">"Password Requirements:"</h4>
        //                         <div class="space-y-1 text-sm">
        //                             <Validator
        //                                 valid=Some(true)
        //                                 success_message="At least 8 characters"
        //                                 show_icon=true
        //                             />
        //                             <Validator
        //                                 valid=Some(true)
        //                                 success_message="Contains uppercase letter"
        //                                 show_icon=true
        //                             />
        //                             <Validator
        //                                 valid=Some(false)
        //                                 error_message="Contains lowercase letter"
        //                                 show_icon=true
        //                             />
        //                             <Validator
        //                                 valid=Some(true)
        //                                 success_message="Contains number"
        //                                 show_icon=true
        //                             />
        //                             <Validator
        //                                 valid=Some(false)
        //                                 error_message="Contains special character"
        //                                 show_icon=true
        //                             />
        //                         </div>
        //                     </div>

        //                     <div class="mt-4">
        //                         <Label class="label">
        //                             <LabelText>"Password Strength"</LabelText>
        //                         </Label>
        //                         <Progress value=60 color=ProgressColor::Warning class="mb-2" />
        //                         <div class="flex justify-between text-xs">
        //                             <span>"Weak"</span>
        //                             <span>"Medium"</span>
        //                             <span>"Strong"</span>
        //                         </div>
        //                     </div>
        //                 </div>
        //             </CardBody>
        //         </Card>

        //         <h2 class="text-xl font-semibold">"File Upload Validation"</h2>
        //         <Card class="bg-base-100 shadow-xl">
        //             <CardBody>
        //                 <h3 class="card-title mb-4">"Document Upload"</h3>
        //                 <div class="form-control">
        //                     <Label class="label">
        //                         <LabelText>"Upload File"</LabelText>
        //                     </Label>
        //                     <FileInput class="file-input file-input-bordered" />
        //                     <div class="mt-3 space-y-2">
        //                         <Validator
        //                             valid=Some(true)
        //                             success_message="File type is supported (PDF, DOC, DOCX)"
        //                         />
        //                         <Validator
        //                             valid=Some(true)
        //                             success_message="File size is within limit (< 10MB)"
        //                         />
        //                         <Validator
        //                             valid=Some(false)
        //                             error_message="File contains potentially malicious content"
        //                         />
        //                     </div>
        //                 </div>
        //             </CardBody>
        //         </Card>

        //         <h2 class="text-xl font-semibold">"Form Summary"</h2>
        //         <Card class="bg-base-100 shadow-xl">
        //             <CardBody>
        //                 <h3 class="card-title mb-4">"Validation Summary"</h3>
        //                 <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
        //                     <div>
        //                         <h4 class="font-semibold mb-3 text-success">"✓ Valid Fields"</h4>
        //                         <div class="space-y-2 text-sm">
        //                             <div class="flex items-center gap-2">
        //                                 <Icon icon=icondata::AiCheckCircleOutlined class="text-success" />
        //                                 <span>"Username"</span>
        //                             </div>
        //                             <div class="flex items-center gap-2">
        //                                 <Icon icon=icondata::AiCheckCircleOutlined class="text-success" />
        //                                 <span>"Email Address"</span>
        //                             </div>
        //                             <div class="flex items-center gap-2">
        //                                 <Icon icon=icondata::AiCheckCircleOutlined class="text-success" />
        //                                 <span>"Phone Number"</span>
        //                             </div>
        //                             <div class="flex items-center gap-2">
        //                                 <Icon icon=icondata::AiCheckCircleOutlined class="text-success" />
        //                                 <span>"Terms Accepted"</span>
        //                             </div>
        //                         </div>
        //                     </div>

        //                     <div>
        //                         <h4 class="font-semibold mb-3 text-error">"✗ Invalid Fields"</h4>
        //                         <div class="space-y-2 text-sm">
        //                             <div class="flex items-center gap-2">
        //                                 <Icon icon=icondata::AiCloseCircleOutlined class="text-error" />
        //                                 <span>"Password too weak"</span>
        //                             </div>
        //                             <div class="flex items-center gap-2">
        //                                 <Icon icon=icondata::AiCloseCircleOutlined class="text-error" />
        //                                 <span>"Passwords don't match"</span>
        //                             </div>
        //                             <div class="flex items-center gap-2">
        //                                 <Icon icon=icondata::AiCloseCircleOutlined class="text-error" />
        //                                 <span>"Invalid file format"</span>
        //                             </div>
        //                         </div>
        //                     </div>
        //                 </div>

        //                 <div class="mt-6 p-4 bg-error/10 border border-error/20 rounded-lg">
        //                     <div class="flex items-center gap-2 mb-2">
        //                         <Icon icon=icondata::AiWarningOutlined class="text-error" />
        //                         <span class="font-semibold text-error">"Form has validation errors"</span>
        //                     </div>
        //                     <p class="text-sm">"Please fix the errors above before submitting the form."</p>
        //                 </div>
        //             </CardBody>
        //         </Card>

        //         <h2 class="text-xl font-semibold">"Real-time Validation"</h2>
        //         <Card class="bg-base-100 shadow-xl">
        //             <CardBody>
        //                 <h3 class="card-title mb-4">"Live Feedback"</h3>
        //                 <div class="space-y-4">
        //                     <div class="form-control">
        //                         <Label class="label">
        //                             <LabelText>"Credit Card Number"</LabelText>
        //                         </Label>
        //                         <Input
        //                             placeholder="1234 5678 9012 3456"
        //                             class="input input-bordered input-success"
        //                         />
        //                         <Validator
        //                             valid=Some(true)
        //                             success_message="Valid Visa card number"
        //                         />
        //                     </div>

        //                     <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        //                         <div class="form-control">
        //                             <Label class="label">
        //                                 <LabelText>"Expiry Date"</LabelText>
        //                             </Label>
        //                             <Input
        //                                 placeholder="MM/YY"
        //                                 class="input input-bordered input-warning"
        //                             />
        //                             <Validator
        //                                 valid=Some(false)
        //                                 error_message="Card expires soon"
        //                             />
        //                         </div>

        //                         <div class="form-control">
        //                             <Label class="label">
        //                                 <LabelText>"CVV"</LabelText>
        //                             </Label>
        //                             <Input
        //                                 placeholder="123"
        //                                 class="input input-bordered"
        //                             />
        //                             <Validator
        //                                 valid=None
        //                                 info_message="3-4 digits on back of card"
        //                             />
        //                         </div>
        //                     </div>
        //                 </div>
        //             </CardBody>
        //         </Card>
        //     </div>
        // </div>
    }
}
