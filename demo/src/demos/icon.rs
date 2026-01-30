use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

/// Icon component demo
#[component]
pub fn IconDemo() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div>
                <h2 class="text-2xl font-bold mb-4">"Icon Component"</h2>
                <p class="mb-4 opacity-70">
                    "Icon component with Lucide icon library support. Supports size variants, colors, and rotation."
                </p>
                <div class="alert alert-warning mb-4">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-current shrink-0 w-6 h-6">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"></path>
                    </svg>
                    <span>
                        "Note: Lucide icons require including the Lucide library in your index.html. "
                        "This demo uses SVG fallbacks for display purposes."
                    </span>
                </div>
            </div>

            // Size Variants
            <div class="card bg-base-200">
                <div class="card-body">
                    <h3 class="card-title">"Size Variants"</h3>
                    <div class="flex items-center gap-4 flex-wrap">
                        <div class="text-center">
                            <Icon name="heart" size=IconSize::XSmall color="text-error" />
                            <p class="text-xs mt-1">"XSmall (16px)"</p>
                        </div>
                        <div class="text-center">
                            <Icon name="star" size=IconSize::Small color="text-warning" />
                            <p class="text-xs mt-1">"Small (20px)"</p>
                        </div>
                        <div class="text-center">
                            <Icon name="check" size=IconSize::Medium color="text-success" />
                            <p class="text-xs mt-1">"Medium (24px)"</p>
                        </div>
                        <div class="text-center">
                            <Icon name="info" size=IconSize::Large color="text-info" />
                            <p class="text-xs mt-1">"Large (32px)"</p>
                        </div>
                        <div class="text-center">
                            <Icon name="alert-circle" size=IconSize::XLarge color="text-error" />
                            <p class="text-xs mt-1">"XLarge (48px)"</p>
                        </div>
                    </div>
                </div>
            </div>

            // Color Variants
            <div class="card bg-base-200">
                <div class="card-body">
                    <h3 class="card-title">"Color Variants"</h3>
                    <div class="flex items-center gap-4 flex-wrap">
                        <Icon name="heart" color="text-primary" />
                        <Icon name="star" color="text-secondary" />
                        <Icon name="zap" color="text-accent" />
                        <Icon name="check-circle" color="text-success" />
                        <Icon name="alert-triangle" color="text-warning" />
                        <Icon name="x-circle" color="text-error" />
                        <Icon name="info" color="text-info" />
                        <Icon name="user" color="text-neutral" />
                    </div>
                </div>
            </div>

            // Usage Examples
            <div class="card bg-base-200">
                <div class="card-body">
                    <h3 class="card-title">"Usage Examples"</h3>
                    <div class="space-y-4">
                        // Buttons with icons
                        <div>
                            <h4 class="font-semibold mb-2">"Buttons with Icons"</h4>
                            <div class="flex gap-2 flex-wrap">
                                <button class="btn btn-primary">
                                    <Icon name="heart" size=IconSize::Small />
                                    "Like"
                                </button>
                                <button class="btn btn-secondary">
                                    <Icon name="share-2" size=IconSize::Small />
                                    "Share"
                                </button>
                                <button class="btn btn-accent">
                                    <Icon name="download" size=IconSize::Small />
                                    "Download"
                                </button>
                                <button class="btn btn-ghost btn-circle">
                                    <Icon name="settings" size=IconSize::Medium />
                                </button>
                            </div>
                        </div>

                        // Alert with icon
                        <div>
                            <h4 class="font-semibold mb-2">"Alerts with Icons"</h4>
                            <div class="space-y-2">
                                <div class="alert alert-success">
                                    <Icon name="check-circle" size=IconSize::Medium color="text-success-content" />
                                    <span>"Operation completed successfully!"</span>
                                </div>
                                <div class="alert alert-warning">
                                    <Icon name="alert-triangle" size=IconSize::Medium color="text-warning-content" />
                                    <span>"Please review your changes before proceeding."</span>
                                </div>
                                <div class="alert alert-error">
                                    <Icon name="x-circle" size=IconSize::Medium color="text-error-content" />
                                    <span>"An error occurred. Please try again."</span>
                                </div>
                            </div>
                        </div>

                        // Navigation items
                        <div>
                            <h4 class="font-semibold mb-2">"Navigation Items"</h4>
                            <ul class="menu bg-base-100 w-56 rounded-box">
                                <li>
                                    <a>
                                        <Icon name="home" size=IconSize::Small />
                                        "Home"
                                    </a>
                                </li>
                                <li>
                                    <a>
                                        <Icon name="user" size=IconSize::Small />
                                        "Profile"
                                    </a>
                                </li>
                                <li>
                                    <a>
                                        <Icon name="settings" size=IconSize::Small />
                                        "Settings"
                                    </a>
                                </li>
                                <li>
                                    <a>
                                        <Icon name="log-out" size=IconSize::Small />
                                        "Logout"
                                    </a>
                                </li>
                            </ul>
                        </div>
                    </div>
                </div>
            </div>

            // Setup Instructions
            <div class="card bg-base-200">
                <div class="card-body">
                    <h3 class="card-title">"Setup Lucide Icons"</h3>
                    <p class="mb-2">
                        "To use Lucide icons in your app, add this to your index.html:"
                    </p>
                    <div class="mockup-code">
                        <pre data-prefix="1"><code>"<script src=\"https://unpkg.com/lucide@latest\"></script>"</code></pre>
                        <pre data-prefix="2"><code>"<script>"</code></pre>
                        <pre data-prefix="3"><code>"  lucide.createIcons();"</code></pre>
                        <pre data-prefix="4"><code>"</script>"</code></pre>
                    </div>
                    <p class="mt-2 text-sm opacity-70">
                        "Browse available icons at: "
                        <a href="https://lucide.dev/icons/" target="_blank" class="link link-primary">
                            "https://lucide.dev/icons/"
                        </a>
                    </p>
                </div>
            </div>
        </div>
    }
}
