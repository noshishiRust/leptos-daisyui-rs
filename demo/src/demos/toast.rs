use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn ToastDemo() -> impl IntoView {
    let show_toast = RwSignal::new(false);
    let toast_message = RwSignal::new("Hello! This is a toast message.".to_string());
    let toast_type = RwSignal::new("info".to_string());

    let show_toast_fn = move |msg: &str, toast_t: &str| {
        toast_message.set(msg.to_string());
        toast_type.set(toast_t.to_string());
        show_toast.set(true);
        
        // Auto-hide after 3 seconds
        set_timeout(
            move || {
                show_toast.set(false);
            },
            std::time::Duration::from_millis(3000),
        );
    };

    view! {
        <div class="space-y-6">
            <h1 class="text-3xl font-bold">"Toast"</h1>
            <p class="text-base-content/70">
                "Toast notifications are used to show brief messages to users"
            </p>

            <div class="space-y-4">
                <h2 class="text-xl font-semibold">"Toast Examples"</h2>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <Button
                        color=ButtonColor::Info
                        on:click=move |_| show_toast_fn("Information message", "info")
                    >
                        "Show Info Toast"
                    </Button>

                    <Button
                        color=ButtonColor::Success
                        on:click=move |_| show_toast_fn("Success! Operation completed.", "success")
                    >
                        "Show Success Toast"
                    </Button>

                    <Button
                        color=ButtonColor::Warning
                        on:click=move |_| show_toast_fn(
                            "Warning: Please check your input.",
                            "warning",
                        )
                    >
                        "Show Warning Toast"
                    </Button>

                    <Button
                        color=ButtonColor::Error
                        on:click=move |_| show_toast_fn("Error: Something went wrong!", "error")
                    >
                        "Show Error Toast"
                    </Button>
                </div>

                <h2 class="text-xl font-semibold">"Static Toast Examples"</h2>
                <div class="space-y-4">
                    <div class="toast toast-top toast-start">
                        <div class="alert alert-info">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                fill="none"
                                viewBox="0 0 24 24"
                                class="stroke-current shrink-0 w-6 h-6"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                                ></path>
                            </svg>
                            <span>"Info: New version available"</span>
                        </div>
                    </div>

                    <div class="toast toast-top toast-center">
                        <div class="alert alert-success">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                class="stroke-current shrink-0 h-6 w-6"
                                fill="none"
                                viewBox="0 0 24 24"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
                                />
                            </svg>
                            <span>"File saved successfully!"</span>
                        </div>
                    </div>

                    <div class="toast toast-top toast-end">
                        <div class="alert alert-warning">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                class="stroke-current shrink-0 h-6 w-6"
                                fill="none"
                                viewBox="0 0 24 24"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16c-.77.833.192 2.5 1.732 2.5z"
                                />
                            </svg>
                            <span>"Connection unstable"</span>
                        </div>
                    </div>

                    <div class="toast toast-middle toast-start">
                        <div class="alert alert-error">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                class="stroke-current shrink-0 h-6 w-6"
                                fill="none"
                                viewBox="0 0 24 24"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
                                />
                            </svg>
                            <span>"Failed to upload file"</span>
                        </div>
                    </div>

                    <div class="toast toast-middle toast-center">
                        <div class="alert">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                fill="none"
                                viewBox="0 0 24 24"
                                class="stroke-current shrink-0 w-6 h-6"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                                ></path>
                            </svg>
                            <span>"Your session will expire in 5 minutes"</span>
                        </div>
                    </div>

                    <div class="toast toast-middle toast-end">
                        <div class="alert alert-info">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                fill="none"
                                viewBox="0 0 24 24"
                                class="stroke-current shrink-0 w-6 h-6"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                                ></path>
                            </svg>
                            <span>"Updates available"</span>
                        </div>
                    </div>

                    <div class="toast toast-bottom toast-start">
                        <div class="alert alert-success">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                class="stroke-current shrink-0 h-6 w-6"
                                fill="none"
                                viewBox="0 0 24 24"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
                                />
                            </svg>
                            <span>"Download completed"</span>
                        </div>
                    </div>

                    <div class="toast toast-bottom toast-center">
                        <div class="alert alert-warning">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                class="stroke-current shrink-0 h-6 w-6"
                                fill="none"
                                viewBox="0 0 24 24"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16c-.77.833.192 2.5 1.732 2.5z"
                                />
                            </svg>
                            <span>"Low storage space"</span>
                        </div>
                    </div>

                    <div class="toast toast-bottom toast-end">
                        <div class="alert alert-error">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                class="stroke-current shrink-0 h-6 w-6"
                                fill="none"
                                viewBox="0 0 24 24"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
                                />
                            </svg>
                            <span>"Network error occurred"</span>
                        </div>
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Toast with Actions"</h2>
                <div class="toast toast-top toast-start">
                    <div class="alert alert-info">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            fill="none"
                            viewBox="0 0 24 24"
                            class="stroke-current shrink-0 w-6 h-6"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                            ></path>
                        </svg>
                        <span>"Cookie consent required"</span>
                        <div>
                            <Button size=ButtonSize::Sm style=ButtonStyle::Ghost>
                                "Deny"
                            </Button>
                            <Button size=ButtonSize::Sm color=ButtonColor::Primary>
                                "Accept"
                            </Button>
                        </div>
                    </div>
                </div>

                <div class="toast toast-top toast-center">
                    <div class="alert alert-warning">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="stroke-current shrink-0 h-6 w-6"
                            fill="none"
                            viewBox="0 0 24 24"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16c-.77.833.192 2.5 1.732 2.5z"
                            />
                        </svg>
                        <span>"Unsaved changes detected"</span>
                        <div>
                            <Button size=ButtonSize::Sm style=ButtonStyle::Outline>
                                "Discard"
                            </Button>
                            <Button size=ButtonSize::Sm color=ButtonColor::Success>
                                "Save"
                            </Button>
                        </div>
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Multiple Toasts"</h2>
                <div class="toast toast-top toast-end">
                    <div class="alert alert-info">
                        <span>"New message received"</span>
                    </div>
                    <div class="alert alert-success">
                        <span>"Settings saved"</span>
                    </div>
                    <div class="alert alert-warning">
                        <span>"Update available"</span>
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Toast Notification System"</h2>
                <Card class="bg-base-100 shadow-xl">
                    <CardBody>
                        <h2 class="card-title">"Notification Center"</h2>
                        <p>
                            "Click the buttons below to trigger different types of notifications:"
                        </p>

                        <div class="grid grid-cols-2 md:grid-cols-4 gap-2 mt-4">
                            <Button
                                size=ButtonSize::Sm
                                color=ButtonColor::Info
                                on:click=move |_| show_toast_fn(
                                    "📧 You have 3 new messages",
                                    "info",
                                )
                            >
                                "📧 Messages"
                            </Button>

                            <Button
                                size=ButtonSize::Sm
                                color=ButtonColor::Success
                                on:click=move |_| show_toast_fn(
                                    "✅ Profile updated successfully!",
                                    "success",
                                )
                            >
                                "✅ Profile"
                            </Button>

                            <Button
                                size=ButtonSize::Sm
                                color=ButtonColor::Warning
                                on:click=move |_| show_toast_fn(
                                    "⚠️ Password expires in 3 days",
                                    "warning",
                                )
                            >
                                "⚠️ Security"
                            </Button>

                            <Button
                                size=ButtonSize::Sm
                                color=ButtonColor::Error
                                on:click=move |_| show_toast_fn(
                                    "❌ Connection lost. Retrying...",
                                    "error",
                                )
                            >
                                "❌ Network"
                            </Button>

                            <Button
                                size=ButtonSize::Sm
                                color=ButtonColor::Primary
                                on:click=move |_| show_toast_fn(
                                    "🎉 Welcome to our platform!",
                                    "info",
                                )
                            >
                                "🎉 Welcome"
                            </Button>

                            <Button
                                size=ButtonSize::Sm
                                color=ButtonColor::Secondary
                                on:click=move |_| show_toast_fn(
                                    "📥 File uploaded: document.pdf",
                                    "success",
                                )
                            >
                                "📥 Upload"
                            </Button>

                            <Button
                                size=ButtonSize::Sm
                                color=ButtonColor::Accent
                                on:click=move |_| show_toast_fn(
                                    "🔔 Meeting starts in 10 minutes",
                                    "warning",
                                )
                            >
                                "🔔 Meeting"
                            </Button>

                            <Button
                                size=ButtonSize::Sm
                                style=ButtonStyle::Outline
                                on:click=move |_| show_toast_fn(
                                    "🚀 Version 2.0 is now available!",
                                    "info",
                                )
                            >
                                "🚀 Update"
                            </Button>
                        </div>
                    </CardBody>
                </Card>

                <h2 class="text-xl font-semibold">"E-commerce Toasts"</h2>
                <Card class="bg-base-100 shadow-xl">
                    <CardBody>
                        <h2 class="card-title">"Shopping Actions"</h2>
                        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                            <div class="p-4 border rounded-lg">
                                <img
                                    src="https://picsum.photos/200/150?random=1"
                                    alt="Product"
                                    class="w-full h-32 object-cover rounded mb-2"
                                />
                                <h3 class="font-semibold">"Wireless Headphones"</h3>
                                <p class="text-lg font-bold text-primary">"$99.99"</p>
                                <Button
                                    class="w-full mt-2"
                                    size=ButtonSize::Sm
                                    color=ButtonColor::Primary
                                    on:click=move |_| show_toast_fn(
                                        "🛒 Wireless Headphones added to cart!",
                                        "success",
                                    )
                                >
                                    "Add to Cart"
                                </Button>
                            </div>

                            <div class="p-4 border rounded-lg">
                                <img
                                    src="https://picsum.photos/200/150?random=2"
                                    alt="Product"
                                    class="w-full h-32 object-cover rounded mb-2"
                                />
                                <h3 class="font-semibold">"Smart Watch"</h3>
                                <p class="text-lg font-bold text-primary">"$299.99"</p>
                                <Button
                                    class="w-full mt-2"
                                    size=ButtonSize::Sm
                                    color=ButtonColor::Primary
                                    on:click=move |_| show_toast_fn(
                                        "🛒 Smart Watch added to cart!",
                                        "success",
                                    )
                                >
                                    "Add to Cart"
                                </Button>
                            </div>

                            <div class="p-4 border rounded-lg">
                                <img
                                    src="https://picsum.photos/200/150?random=3"
                                    alt="Product"
                                    class="w-full h-32 object-cover rounded mb-2"
                                />
                                <h3 class="font-semibold">"Bluetooth Speaker"</h3>
                                <p class="text-lg font-bold text-primary">"$79.99"</p>
                                <Button
                                    class="w-full mt-2"
                                    size=ButtonSize::Sm
                                    color=ButtonColor::Primary
                                    on:click=move |_| show_toast_fn(
                                        "🛒 Bluetooth Speaker added to cart!",
                                        "success",
                                    )
                                >
                                    "Add to Cart"
                                </Button>
                            </div>
                        </div>

                        <div class="mt-4 flex gap-2">
                            <Button
                                color=ButtonColor::Success
                                on:click=move |_| show_toast_fn(
                                    "✅ Order placed successfully! Order #12345",
                                    "success",
                                )
                            >
                                "Complete Purchase"
                            </Button>

                            <Button
                                color=ButtonColor::Warning
                                on:click=move |_| show_toast_fn(
                                    "⚠️ Some items in your cart are low in stock",
                                    "warning",
                                )
                            >
                                "Check Stock"
                            </Button>

                            <Button
                                color=ButtonColor::Error
                                on:click=move |_| show_toast_fn(
                                    "❌ Payment failed. Please try again.",
                                    "error",
                                )
                            >
                                "Simulate Payment Error"
                            </Button>
                        </div>
                    </CardBody>
                </Card>

                // Dynamic toast that appears based on state
                {move || {
                    show_toast
                        .get()
                        .then(|| {
                            let toast_class = match toast_type.get().as_str() {
                                "success" => "alert-success",
                                "warning" => "alert-warning",
                                "error" => "alert-error",
                                _ => "alert-info",
                            };
                            let icon = match toast_type.get().as_str() {
                                "success" => {

                                    view! {
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            class="stroke-current shrink-0 h-6 w-6"
                                            fill="none"
                                            viewBox="0 0 24 24"
                                        >
                                            <path
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                stroke-width="2"
                                                d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
                                            />
                                        </svg>
                                    }
                                }
                                "warning" => {
                                    view! {
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            class="stroke-current shrink-0 h-6 w-6"
                                            fill="none"
                                            viewBox="0 0 24 24"
                                        >
                                            <path
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                stroke-width="2"
                                                d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16c-.77.833.192 2.5 1.732 2.5z"
                                            />
                                        </svg>
                                    }
                                }
                                "error" => {
                                    view! {
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            class="stroke-current shrink-0 h-6 w-6"
                                            fill="none"
                                            viewBox="0 0 24 24"
                                        >
                                            <path
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                stroke-width="2"
                                                d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
                                            />
                                        </svg>
                                    }
                                }
                                _ => {
                                    view! {
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            fill="none"
                                            viewBox="0 0 24 24"
                                            class="stroke-current shrink-0 w-6 h-6"
                                        >
                                            <path
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                stroke-width="2"
                                                d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                                            ></path>
                                        </svg>
                                    }
                                }
                            };

                            view! {
                                <div class="toast toast-top toast-center z-50">
                                    <div class=format!(
                                        "alert {}",
                                        toast_class,
                                    )>
                                        {icon} <span>{toast_message.get()}</span>
                                        <button
                                            class="btn btn-sm btn-circle btn-ghost ml-2"
                                            on:click=move |_| show_toast.set(false)
                                        >
                                            "✕"
                                        </button>
                                    </div>
                                </div>
                            }
                        })
                }}
            </div>
        </div>
    }
}