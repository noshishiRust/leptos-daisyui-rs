use leptos::prelude::*;

#[component]
pub fn LinkDemo() -> impl IntoView {
    view! {
        // <div class="space-y-6">
        //     <h1 class="text-3xl font-bold">"Link"</h1>
        //     <p class="text-base-content/70">
        //         "Link adds the missing underline style to anchor tags"
        //     </p>

        //     <div class="space-y-4">
        //         <h2 class="text-xl font-semibold">"Basic Link"</h2>
        //         <div class="space-y-2">
        //             <p>
        //                 "This is a regular text with a "
        //                 <Link href="#" class="link">
        //                     "simple link"
        //                 </Link>
        //                 " in the middle."
        //             </p>
        //         </div>

        //         <h2 class="text-xl font-semibold">"Link Colors"</h2>
        //         <div class="space-y-2">
        //             <p>
        //                 <Link href="#" color=LinkColor::Primary class="link">
        //                     "Primary link"
        //                 </Link>
        //             </p>
        //             <p>
        //                 <Link href="#" color=LinkColor::Secondary class="link">
        //                     "Secondary link"
        //                 </Link>
        //             </p>
        //             <p>
        //                 <Link href="#" color=LinkColor::Accent class="link">
        //                     "Accent link"
        //                 </Link>
        //             </p>
        //             <p>
        //                 <Link href="#" color=LinkColor::Info class="link">
        //                     "Info link"
        //                 </Link>
        //             </p>
        //             <p>
        //                 <Link href="#" color=LinkColor::Success class="link">
        //                     "Success link"
        //                 </Link>
        //             </p>
        //             <p>
        //                 <Link href="#" color=LinkColor::Warning class="link">
        //                     "Warning link"
        //                 </Link>
        //             </p>
        //             <p>
        //                 <Link href="#" color=LinkColor::Error class="link">
        //                     "Error link"
        //                 </Link>
        //             </p>
        //         </div>

        //         <h2 class="text-xl font-semibold">"Link Hover Styles"</h2>
        //         <div class="space-y-2">
        //             <p>
        //                 <Link href="#" style=LinkStyle::Hover>
        //                     "Link with hover effect"
        //                 </Link>
        //             </p>
        //             <p>
        //                 <Link href="#" style=LinkStyle::Hover color=LinkColor::Primary>
        //                     "Primary hover link"
        //                 </Link>
        //             </p>
        //             <p>
        //                 <Link href="#" style=LinkStyle::Hover color=LinkColor::Secondary>
        //                     "Secondary hover link"
        //                 </Link>
        //             </p>
        //         </div>

        //         <h2 class="text-xl font-semibold">"Neutral Link"</h2>
        //         <div class="space-y-2">
        //             <p>
        //                 <Link href="#" color=LinkColor::Neutral>
        //                     "Neutral colored link"
        //                 </Link>
        //             </p>
        //             <p>
        //                 <Link href="#" color=LinkColor::Neutral style=LinkStyle::Hover>
        //                     "Neutral hover link"
        //                 </Link>
        //             </p>
        //         </div>

        //         <h2 class="text-xl font-semibold">"Links with Icons"</h2>
        //         <div class="space-y-2">
        //             <p>
        //                 <Link href="#" class="link inline-flex items-center gap-1">
        //                     <Icon icon=icondata::AiHomeOutlined class="w-4 h-4" />
        //                     "Home"
        //                 </Link>
        //             </p>
        //             <p>
        //                 <Link href="#" color=LinkColor::Primary class="link inline-flex items-center gap-1">
        //                     <Icon icon=icondata::AiFileTextOutlined class="w-4 h-4" />
        //                     "Documentation"
        //                 </Link>
        //             </p>
        //             <p>
        //                 <Link href="#" color=LinkColor::Success class="link inline-flex items-center gap-1">
        //                     <Icon icon=icondata::AiDownloadOutlined class="w-4 h-4" />
        //                     "Download"
        //                 </Link>
        //             </p>
        //             <p>
        //                 <Link href="#" color=LinkColor::Info class="link inline-flex items-center gap-1">
        //                     "External Link"
        //                     <Icon icon=icondata::AiExportOutlined class="w-4 h-4" />
        //                 </Link>
        //             </p>
        //         </div>

        //         <h2 class="text-xl font-semibold">"Navigation Links"</h2>
        //         <div class="flex flex-wrap gap-4">
        //             <Link href="#" class="link">
        //                 "About"
        //             </Link>
        //             <Link href="#" class="link">
        //                 "Services"
        //             </Link>
        //             <Link href="#" class="link">
        //                 "Portfolio"
        //             </Link>
        //             <Link href="#" class="link">
        //                 "Contact"
        //             </Link>
        //         </div>

        //         <h2 class="text-xl font-semibold">"Breadcrumb Links"</h2>
        //         <div class="text-sm breadcrumbs">
        //             <ul>
        //                 <li>
        //                     <Link href="#" class="link">
        //                         "Home"
        //                     </Link>
        //                 </li>
        //                 <li>
        //                     <Link href="#" class="link">
        //                         "Documents"
        //                     </Link>
        //                 </li>
        //                 <li>
        //                     <Link href="#" class="link">
        //                         "Components"
        //                     </Link>
        //                 </li>
        //                 <li>"Link"</li>
        //             </ul>
        //         </div>

        //         <h2 class="text-xl font-semibold">"Footer Links"</h2>
        //         <div class="bg-base-200 p-6 rounded-lg">
        //             <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
        //                 <div>
        //                     <h3 class="font-semibold mb-2">"Company"</h3>
        //                     <div class="space-y-1">
        //                         <p>
        //                             <Link href="#" style=LinkStyle::Hover class="text-sm">
        //                                 "About Us"
        //                             </Link>
        //                         </p>
        //                         <p>
        //                             <Link href="#" style=LinkStyle::Hover class="text-sm">
        //                                 "Careers"
        //                             </Link>
        //                         </p>
        //                         <p>
        //                             <Link href="#" style=LinkStyle::Hover class="text-sm">
        //                                 "Press"
        //                             </Link>
        //                         </p>
        //                     </div>
        //                 </div>
        //                 <div>
        //                     <h3 class="font-semibold mb-2">"Support"</h3>
        //                     <div class="space-y-1">
        //                         <p>
        //                             <Link href="#" style=LinkStyle::Hover class="text-sm">
        //                                 "Help Center"
        //                             </Link>
        //                         </p>
        //                         <p>
        //                             <Link href="#" style=LinkStyle::Hover class="text-sm">
        //                                 "Contact Us"
        //                             </Link>
        //                         </p>
        //                         <p>
        //                             <Link href="#" style=LinkStyle::Hover class="text-sm">
        //                                 "Status"
        //                             </Link>
        //                         </p>
        //                     </div>
        //                 </div>
        //                 <div>
        //                     <h3 class="font-semibold mb-2">"Legal"</h3>
        //                     <div class="space-y-1">
        //                         <p>
        //                             <Link href="#" style=LinkStyle::Hover class="text-sm">
        //                                 "Privacy"
        //                             </Link>
        //                         </p>
        //                         <p>
        //                             <Link href="#" style=LinkStyle::Hover class="text-sm">
        //                                 "Terms"
        //                             </Link>
        //                         </p>
        //                         <p>
        //                             <Link href="#" style=LinkStyle::Hover class="text-sm">
        //                                 "Cookie Policy"
        //                             </Link>
        //                         </p>
        //                     </div>
        //                 </div>
        //                 <div>
        //                     <h3 class="font-semibold mb-2">"Social"</h3>
        //                     <div class="space-y-1">
        //                         <p>
        //                             <Link href="#" style=LinkStyle::Hover class="text-sm inline-flex items-center gap-1">
        //                                 <Icon icon=icondata::AiTwitterOutlined class="w-4 h-4" />
        //                                 "Twitter"
        //                             </Link>
        //                         </p>
        //                         <p>
        //                             <Link href="#" style=LinkStyle::Hover class="text-sm inline-flex items-center gap-1">
        //                                 <Icon icon=icondata::AiGithubOutlined class="w-4 h-4" />
        //                                 "GitHub"
        //                             </Link>
        //                         </p>
        //                         <p>
        //                             <Link href="#" style=LinkStyle::Hover class="text-sm inline-flex items-center gap-1">
        //                                 <Icon icon=icondata::AiLinkedinOutlined class="w-4 h-4" />
        //                                 "LinkedIn"
        //                             </Link>
        //                         </p>
        //                     </div>
        //                 </div>
        //             </div>
        //         </div>

        //         <h2 class="text-xl font-semibold">"Call-to-Action Links"</h2>
        //         <div class="space-y-4">
        //             <Alert color=AlertColor::Info>
        //                 <Icon icon=icondata::AiInfoCircleOutlined />
        //                 <div>
        //                     "Need help getting started? "
        //                     <Link href="#" color=LinkColor::Info class="font-semibold">
        //                         "Read our documentation"
        //                     </Link>
        //                     " or "
        //                     <Link href="#" color=LinkColor::Info class="font-semibold">
        //                         "contact support"
        //                     </Link>
        //                     "."
        //                 </div>
        //             </Alert>

        //             <Alert color=AlertColor::Success>
        //                 <Icon icon=icondata::AiCheckCircleOutlined />
        //                 <div>
        //                     "Account created successfully! "
        //                     <Link href="#" color=LinkColor::Success class="font-semibold">
        //                         "Complete your profile"
        //                     </Link>
        //                     " to get started."
        //                 </div>
        //             </Alert>
        //         </div>

        //         <h2 class="text-xl font-semibold">"Article Links"</h2>
        //         <Card class="bg-base-100 shadow-xl">
        //             <CardBody>
        //                 <h2 class="card-title">"Related Articles"</h2>
        //                 <div class="space-y-3">
        //                     <div>
        //                         <h3>
        //                             <Link href="#" color=LinkColor::Primary class="font-semibold">
        //                                 "Getting Started with daisyUI"
        //                             </Link>
        //                         </h3>
        //                         <p class="text-sm opacity-70">
        //                             "Learn the basics of using daisyUI components in your project."
        //                         </p>
        //                     </div>
        //                     <div>
        //                         <h3>
        //                             <Link href="#" color=LinkColor::Primary class="font-semibold">
        //                                 "Customizing Component Themes"
        //                             </Link>
        //                         </h3>
        //                         <p class="text-sm opacity-70">
        //                             "How to customize colors and styles for your specific needs."
        //                         </p>
        //                     </div>
        //                     <div>
        //                         <h3>
        //                             <Link href="#" color=LinkColor::Primary class="font-semibold">
        //                                 "Building Responsive Layouts"
        //                             </Link>
        //                         </h3>
        //                         <p class="text-sm opacity-70">
        //                             "Best practices for creating mobile-friendly designs."
        //                         </p>
        //                     </div>
        //                 </div>
        //                 <div class="card-actions justify-end mt-4">
        //                     <Link href="#" color=LinkColor::Primary class="inline-flex items-center gap-1">
        //                         "View all articles"
        //                         <Icon icon=icondata::AiArrowRightOutlined class="w-4 h-4" />
        //                     </Link>
        //                 </div>
        //             </CardBody>
        //         </Card>

        //         <h2 class="text-xl font-semibold">"Button-style Links"</h2>
        //         <div class="flex flex-wrap gap-2">
        //             <Link href="#" class="btn btn-primary">
        //                 "Primary Link Button"
        //             </Link>
        //             <Link href="#" class="btn btn-secondary">
        //                 "Secondary Link Button"
        //             </Link>
        //             <Link href="#" class="btn btn-ghost">
        //                 "Ghost Link Button"
        //             </Link>
        //             <Link href="#" class="btn btn-outline">
        //                 "Outline Link Button"
        //             </Link>
        //         </div>
        //     </div>
        // </div>
    }
}
