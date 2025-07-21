use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn MockupBrowserDemo() -> impl IntoView {
    view! {
        <ContentLayout
            title="Mockup Browser"
            description="Styled web browser frame for displaying web interfaces"
        >
            <Section title="Basic Browser Mockup">
                <MockupBrowser>
                    <MockupBrowserToolbar>
                        <Input
                            attr:value="https://example.com"
                            attr:readonly=true
                            class="w-full border border-base-300 rounded"
                        />
                    </MockupBrowserToolbar>
                    <div class="flex justify-center px-4 py-16 bg-base-200">
                        <div class="text-center">
                            <h1 class="text-4xl font-bold mb-4">"Welcome to Example"</h1>
                            <p class="text-lg text-base-content/70 mb-8">
                                "This is a demo website shown in a browser mockup."
                            </p>
                            <Button color=ButtonColor::Primary>"Get Started"</Button>
                        </div>
                    </div>
                </MockupBrowser>
            </Section>

            <Section title="Browser with Navigation">
                <MockupBrowser>
                    <MockupBrowserToolbar>
                        <div class="flex items-center gap-2">
                            <div class="flex gap-1">
                                <div class="w-3 h-3 rounded-full bg-red-500"></div>
                                <div class="w-3 h-3 rounded-full bg-yellow-500"></div>
                                <div class="w-3 h-3 rounded-full bg-green-500"></div>
                            </div>
                            <Input
                                attr:value="https://leptos-daisyui.dev"
                                attr:readonly=true
                                class="flex-1 border border-base-300 rounded"
                            />
                        </div>
                    </MockupBrowserToolbar>
                    <div class="bg-base-100">
                        <Navbar class="bg-primary text-primary-content">
                            <NavbarStart>
                                <Button
                                    size=ButtonSize::Lg
                                    style=ButtonStyle::Ghost
                                    class="text-xl"
                                >
                                    "Leptos DaisyUI"
                                </Button>
                            </NavbarStart>
                            <NavbarCenter class="hidden lg:flex">
                                <Menu direction=MenuDirection::Horizontal class="px-1">
                                    <li>
                                        <a>"Components"</a>
                                    </li>
                                    <li>
                                        <a>"Docs"</a>
                                    </li>
                                    <li>
                                        <a>"Examples"</a>
                                    </li>
                                </Menu>
                            </NavbarCenter>
                            <NavbarEnd>
                                <Button size=ButtonSize::Sm>"Get Started"</Button>
                            </NavbarEnd>
                        </Navbar>
                        <div class="p-8">
                            <Hero>
                                <HeroContent class="text-center">
                                    <div class="max-w-md">
                                        <h1 class="text-5xl font-bold">"Hello there"</h1>
                                        <p class="py-6">
                                            "Provident cupiditate voluptatem et in. Quaerat fugiat ut assumenda excepturi."
                                        </p>
                                        <Button color=ButtonColor::Primary>"Get Started"</Button>
                                    </div>
                                </HeroContent>
                            </Hero>
                        </div>
                    </div>
                </MockupBrowser>
            </Section>

            <Section title="Simple Website Preview">
                <MockupBrowser>
                    <MockupBrowserToolbar>
                        <Input attr:value="https://portfolio-example.com" attr:readonly=true />
                    </MockupBrowserToolbar>
                    <div class="bg-gradient-to-br from-purple-400 to-blue-600 text-white p-8">
                        <div class="text-center">
                            <Avatar class="mb-4">
                                <div class="w-24 rounded-full bg-white/20 flex items-center justify-center">
                                    <span class="text-2xl">"JD"</span>
                                </div>
                            </Avatar>
                            <h2 class="text-2xl font-bold mb-2">"John Developer"</h2>
                            <p class="text-purple-100 mb-6">"Full Stack Developer & UI Designer"</p>
                            <div class="flex justify-center gap-4">
                                <Badge color=BadgeColor::Primary>"React"</Badge>
                                <Badge color=BadgeColor::Secondary>"TypeScript"</Badge>
                                <Badge color=BadgeColor::Accent>"Rust"</Badge>
                            </div>
                        </div>
                    </div>
                </MockupBrowser>
            </Section>
        </ContentLayout>
    }
}
