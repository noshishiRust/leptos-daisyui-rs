use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;
use leptos_icons::Icon;

#[component]
pub fn LinkDemo() -> impl IntoView {
    view! {
        <ContentLayout
            title="Link"
            description="Link adds the missing underline style to anchor tags"
        >
            <Section title="Basic Link">
                <p>
                    "This is a regular text with a "
                    <Link href="#">
                        "simple link"
                    </Link>
                    " in the middle."
                </p>
            </Section>

            <Section title="Link Colors" row=true>
                <Link href="#" color=LinkColor::Neutral>
                    "Neutral"
                </Link>
                <Link href="#" color=LinkColor::Primary>
                    "Primary"
                </Link>
                <Link href="#" color=LinkColor::Secondary>
                    "Secondary"
                </Link>
                <Link href="#" color=LinkColor::Accent>
                    "Accent"
                </Link>
                <Link href="#" color=LinkColor::Info>
                    "Info"
                </Link>
                <Link href="#" color=LinkColor::Success>
                    "Success"
                </Link>
                <Link href="#" color=LinkColor::Warning>
                    "Warning"
                </Link>
                <Link href="#" color=LinkColor::Error>
                    "Error"
                </Link>
            </Section>

            <Section title="Link Hover Styles" row=true>
                <Link href="#" hover=true>
                    "Hover effect"
                </Link>
                <Link href="#" hover=true color=LinkColor::Primary>
                    "Primary hover"
                </Link>
                <Link href="#" hover=true color=LinkColor::Secondary>
                    "Secondary hover"
                </Link>
                <Link href="#" hover=true color=LinkColor::Accent>
                    "Accent hover"
                </Link>
            </Section>

            <Section title="Links with Icons">
                <div class="space-y-2">
                    <p>
                        <Link href="#" class="inline-flex items-center gap-1">
                            <Icon icon=icondata::AiHomeOutlined />
                            "Home"
                        </Link>
                    </p>
                    <p>
                        <Link href="#" color=LinkColor::Primary class="inline-flex items-center gap-1">
                            <Icon icon=icondata::AiFileTextOutlined />
                            "Documentation"
                        </Link>
                    </p>
                    <p>
                        <Link href="#" color=LinkColor::Success class="inline-flex items-center gap-1">
                            <Icon icon=icondata::AiDownloadOutlined />
                            "Download"
                        </Link>
                    </p>
                    <p>
                        <Link href="#" color=LinkColor::Info class="inline-flex items-center gap-1">
                            "External Link"
                            <Icon icon=icondata::AiExportOutlined />
                        </Link>
                    </p>
                </div>
            </Section>

            <Section title="Navigation Links">
                <div class="flex flex-wrap gap-4">
                    <Link href="#">"About"</Link>
                    <Link href="#">"Services"</Link>
                    <Link href="#">"Portfolio"</Link>
                    <Link href="#">"Contact"</Link>
                </div>
            </Section>

            <Section title="Breadcrumb Links">
                <div class="text-sm breadcrumbs">
                    <ul>
                        <li>
                            <Link href="#">"Home"</Link>
                        </li>
                        <li>
                            <Link href="#">"Documents"</Link>
                        </li>
                        <li>
                            <Link href="#">"Components"</Link>
                        </li>
                        <li>"Link"</li>
                    </ul>
                </div>
            </Section>

            <Section title="Footer Links">
                <div class="bg-base-200 p-6 rounded-lg">
                    <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
                        <div>
                            <h3 class="font-semibold mb-2">"Company"</h3>
                            <div class="space-y-1">
                                <p>
                                    <Link href="#" hover=true class="text-sm">
                                        "About Us"
                                    </Link>
                                </p>
                                <p>
                                    <Link href="#" hover=true class="text-sm">
                                        "Careers"
                                    </Link>
                                </p>
                                <p>
                                    <Link href="#" hover=true class="text-sm">
                                        "Press"
                                    </Link>
                                </p>
                            </div>
                        </div>
                        <div>
                            <h3 class="font-semibold mb-2">"Support"</h3>
                            <div class="space-y-1">
                                <p>
                                    <Link href="#" hover=true class="text-sm">
                                        "Help Center"
                                    </Link>
                                </p>
                                <p>
                                    <Link href="#" hover=true class="text-sm">
                                        "Contact Us"
                                    </Link>
                                </p>
                                <p>
                                    <Link href="#" hover=true class="text-sm">
                                        "Status"
                                    </Link>
                                </p>
                            </div>
                        </div>
                        <div>
                            <h3 class="font-semibold mb-2">"Legal"</h3>
                            <div class="space-y-1">
                                <p>
                                    <Link href="#" hover=true class="text-sm">
                                        "Privacy"
                                    </Link>
                                </p>
                                <p>
                                    <Link href="#" hover=true class="text-sm">
                                        "Terms"
                                    </Link>
                                </p>
                                <p>
                                    <Link href="#" hover=true class="text-sm">
                                        "Cookie Policy"
                                    </Link>
                                </p>
                            </div>
                        </div>
                        <div>
                            <h3 class="font-semibold mb-2">"Social"</h3>
                            <div class="space-y-1">
                                <p>
                                    <Link href="#" hover=true class="text-sm inline-flex items-center gap-1">
                                        <Icon icon=icondata::AiTwitterOutlined />
                                        "Twitter"
                                    </Link>
                                </p>
                                <p>
                                    <Link href="#" hover=true class="text-sm inline-flex items-center gap-1">
                                        <Icon icon=icondata::AiGithubOutlined />
                                        "GitHub"
                                    </Link>
                                </p>
                                <p>
                                    <Link href="#" hover=true class="text-sm inline-flex items-center gap-1">
                                        <Icon icon=icondata::AiLinkedinOutlined />
                                        "LinkedIn"
                                    </Link>
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            </Section>

            <Section title="Call-to-Action Links">
                <div class="space-y-4">
                    <Alert color=AlertColor::Info>
                        <Icon icon=icondata::AiInfoCircleOutlined />
                        <div>
                            "Need help getting started? "
                            <Link href="#" color=LinkColor::Info class="font-semibold">
                                "Read our documentation"
                            </Link>
                            " or "
                            <Link href="#" color=LinkColor::Info class="font-semibold">
                                "contact support"
                            </Link>
                            "."
                        </div>
                    </Alert>

                    <Alert color=AlertColor::Success>
                        <Icon icon=icondata::AiCheckCircleOutlined />
                        <div>
                            "Account created successfully! "
                            <Link href="#" color=LinkColor::Success class="font-semibold">
                                "Complete your profile"
                            </Link>
                            " to get started."
                        </div>
                    </Alert>
                </div>
            </Section>

            <Section title="Article Links">
                <Card class="bg-base-100 shadow-xl">
                    <CardBody>
                        <h2 class="card-title">"Related Articles"</h2>
                        <div class="space-y-3">
                            <div>
                                <h3>
                                    <Link href="#" color=LinkColor::Primary class="font-semibold">
                                        "Getting Started with daisyUI"
                                    </Link>
                                </h3>
                                <p class="text-sm opacity-70">
                                    "Learn the basics of using daisyUI components in your project."
                                </p>
                            </div>
                            <div>
                                <h3>
                                    <Link href="#" color=LinkColor::Primary class="font-semibold">
                                        "Customizing Component Themes"
                                    </Link>
                                </h3>
                                <p class="text-sm opacity-70">
                                    "How to customize colors and styles for your specific needs."
                                </p>
                            </div>
                            <div>
                                <h3>
                                    <Link href="#" color=LinkColor::Primary class="font-semibold">
                                        "Building Responsive Layouts"
                                    </Link>
                                </h3>
                                <p class="text-sm opacity-70">
                                    "Best practices for creating mobile-friendly designs."
                                </p>
                            </div>
                        </div>
                        <div class="card-actions justify-end mt-4">
                            <Link href="#" color=LinkColor::Primary class="inline-flex items-center gap-1">
                                "View all articles"
                                <Icon icon=icondata::AiArrowRightOutlined />
                            </Link>
                        </div>
                    </CardBody>
                </Card>
            </Section>

            <Section title="Button-style Links" row=true>
                <Link href="#" class="btn btn-primary">
                    "Primary Link Button"
                </Link>
                <Link href="#" class="btn btn-secondary">
                    "Secondary Link Button"
                </Link>
                <Link href="#" class="btn btn-ghost">
                    "Ghost Link Button"
                </Link>
                <Link href="#" class="btn btn-outline">
                    "Outline Link Button"
                </Link>
            </Section>
        </ContentLayout>
    }
}
