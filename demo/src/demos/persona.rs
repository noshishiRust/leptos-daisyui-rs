use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

/// Persona component demo
#[component]
pub fn PersonaDemo() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div>
                <h2 class="text-2xl font-bold mb-4">"Persona Component"</h2>
                <p class="mb-4 opacity-70">
                    "User profile card component with avatar, name, and secondary text. Perfect for displaying user information in lists, cards, and navigation."
                </p>
            </div>

            // Size Variants
            <div class="card bg-base-200">
                <div class="card-body">
                    <h3 class="card-title">"Size Variants"</h3>
                    <div class="space-y-4">
                        <Persona
                            size=PersonaSize::XSmall
                            name="Alice Johnson"
                            secondary_text="Software Engineer"
                        />
                        <Persona
                            size=PersonaSize::Small
                            name="Bob Smith"
                            secondary_text="Product Manager"
                        />
                        <Persona
                            size=PersonaSize::Medium
                            name="Carol Williams"
                            secondary_text="UX Designer"
                        />
                        <Persona
                            size=PersonaSize::Large
                            name="David Brown"
                            secondary_text="Engineering Manager"
                        />
                        <Persona
                            size=PersonaSize::XLarge
                            name="Eve Davis"
                            secondary_text="VP of Engineering"
                        />
                    </div>
                </div>
            </div>

            // With Images
            <div class="card bg-base-200">
                <div class="card-body">
                    <h3 class="card-title">"With Avatar Images"</h3>
                    <p class="text-sm opacity-70 mb-4">
                        "Personas can display avatar images or fallback to initials"
                    </p>
                    <div class="space-y-3">
                        <Persona
                            name="John Doe"
                            secondary_text="john.doe@example.com"
                            image_url="https://i.pravatar.cc/150?img=1"
                        />
                        <Persona
                            name="Jane Smith"
                            secondary_text="jane.smith@example.com"
                            image_url="https://i.pravatar.cc/150?img=2"
                        />
                        <Persona
                            name="Mike Johnson"
                            secondary_text="mike.johnson@example.com"
                            image_url="https://i.pravatar.cc/150?img=3"
                        />
                    </div>
                </div>
            </div>

            // Initials Fallback
            <div class="card bg-base-200">
                <div class="card-body">
                    <h3 class="card-title">"Initials Fallback"</h3>
                    <p class="text-sm opacity-70 mb-4">
                        "When no image is provided, initials are automatically generated from the name"
                    </p>
                    <div class="space-y-3">
                        <Persona
                            name="Alice Johnson"
                            secondary_text="Frontend Developer"
                        />
                        <Persona
                            name="Bob Smith"
                            secondary_text="Backend Developer"
                        />
                        <Persona
                            name="Carol Williams"
                            secondary_text="DevOps Engineer"
                        />
                        <Persona
                            name="David Brown"
                            secondary_text="Tech Lead"
                        />
                    </div>
                </div>
            </div>

            // Use Cases
            <div class="card bg-base-200">
                <div class="card-body">
                    <h3 class="card-title">"Use Cases"</h3>

                    <div class="space-y-6">
                        // Team Members List
                        <div>
                            <h4 class="font-semibold mb-3">"Team Members"</h4>
                            <div class="space-y-2">
                                <Persona
                                    name="Emma Davis"
                                    secondary_text="emma.davis@company.com • San Francisco"
                                    size=PersonaSize::Medium
                                />
                                <Persona
                                    name="James Wilson"
                                    secondary_text="james.wilson@company.com • New York"
                                    size=PersonaSize::Medium
                                />
                                <Persona
                                    name="Sophia Martinez"
                                    secondary_text="sophia.martinez@company.com • Austin"
                                    size=PersonaSize::Medium
                                />
                                <Persona
                                    name="Oliver Brown"
                                    secondary_text="oliver.brown@company.com • Seattle"
                                    size=PersonaSize::Medium
                                />
                            </div>
                        </div>

                        // Contact List
                        <div>
                            <h4 class="font-semibold mb-3">"Contact List"</h4>
                            <div class="space-y-2">
                                <Persona
                                    name="Design Team"
                                    secondary_text="5 members"
                                    size=PersonaSize::Small
                                />
                                <Persona
                                    name="Sarah Chen"
                                    secondary_text="Product Designer"
                                    size=PersonaSize::Small
                                />
                                <Persona
                                    name="Engineering"
                                    secondary_text="12 members"
                                    size=PersonaSize::Small
                                />
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            // Interactive Example
            <div class="card bg-base-200">
                <div class="card-body">
                    <h3 class="card-title">"Interactive Example"</h3>
                    <p class="text-sm opacity-70 mb-4">
                        "Click on a persona card (demo styling only)"
                    </p>
                    <div class="space-y-2">
                        <button class="w-full text-left hover:bg-base-300 rounded-lg p-2 transition-colors">
                            <Persona
                                name="Alex Thompson"
                                secondary_text="Senior Software Engineer"
                            />
                        </button>
                        <button class="w-full text-left hover:bg-base-300 rounded-lg p-2 transition-colors">
                            <Persona
                                name="Lisa Anderson"
                                secondary_text="Product Designer"
                            />
                        </button>
                        <button class="w-full text-left hover:bg-base-300 rounded-lg p-2 transition-colors">
                            <Persona
                                name="Michael Lee"
                                secondary_text="Engineering Manager"
                            />
                        </button>
                    </div>
                </div>
            </div>

            // Custom Initials
            <div class="card bg-base-200">
                <div class="card-body">
                    <h3 class="card-title">"Custom Initials"</h3>
                    <p class="text-sm opacity-70 mb-4">
                        "You can override the automatically generated initials"
                    </p>
                    <div class="space-y-3">
                        <Persona
                            name="Administrator"
                            secondary_text="System Account"
                            initials="SYS"
                        />
                        <Persona
                            name="Support Team"
                            secondary_text="24/7 Available"
                            initials="SUP"
                        />
                        <Persona
                            name="Bot Assistant"
                            secondary_text="AI Helper"
                            initials="BOT"
                        />
                    </div>
                </div>
            </div>
        </div>
    }
}
