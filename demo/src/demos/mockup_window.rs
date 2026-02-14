use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;
use leptos_icons::Icon;

#[component]
pub fn MockupWindowDemo() -> impl IntoView {
    view! {
        <ContentLayout
            title="Mockup Window"
            description="Desktop window frame for displaying content"
        >
            <Section title="Basic Window">
                <MockupWindow>
                    <div class="flex justify-center px-4 py-16 bg-base-200">
                        <div class="text-center">
                            <h1 class="text-3xl font-bold mb-4">"Desktop Application"</h1>
                            <p class="text-base-content/70 mb-6">
                                "This is a mockup of a desktop application window."
                            </p>
                            <Button color=ButtonColor::Primary>"Open File"</Button>
                        </div>
                    </div>
                </MockupWindow>
            </Section>

            <Section title="Code Editor Window">
                <MockupWindow>
                    <div class="bg-base-100 h-64">
                        <div class="bg-base-200 px-4 py-2 border-b border-base-300 flex items-center justify-between">
                            <div class="flex items-center gap-2">
                                <div class="flex gap-1">
                                    <div class="w-3 h-3 rounded-full bg-red-500"></div>
                                    <div class="w-3 h-3 rounded-full bg-yellow-500"></div>
                                    <div class="w-3 h-3 rounded-full bg-green-500"></div>
                                </div>
                                <span class="text-sm font-medium">"main.rs - Code Editor"</span>
                            </div>
                            <div class="flex gap-1">
                                <Icon icon=icondata::AiMinusOutlined attr:class="w-4 h-4" />
                                <Icon icon=icondata::AiExpandAltOutlined attr:class="w-4 h-4" />
                                <Icon icon=icondata::AiCloseOutlined attr:class="w-4 h-4" />
                            </div>
                        </div>
                        <div class="bg-base-100 p-4 font-mono text-sm">
                            <div class="space-y-1">
                                <div>
                                    <span class="text-purple-500">"use"</span>
                                    <span class="text-blue-500">"leptos"</span>
                                    <span class="text-base-content">"::prelude::*;"</span>
                                </div>
                                <div></div>
                                <div>
                                    <span class="text-purple-500">"#[component]"</span>
                                </div>
                                <div>
                                    <span class="text-purple-500">"pub fn"</span>
                                    <span class="text-yellow-500">"App"</span>
                                    <span class="text-base-content">"() -> impl IntoView {"</span>
                                </div>
                                <div class="ml-4">
                                    <span class="text-green-500">"view! {"</span>
                                </div>
                                <div class="ml-8">
                                    <span class="text-blue-500">"<h1>"</span>
                                    <span class="text-green-600">"\"Hello, Leptos!\""</span>
                                    <span class="text-blue-500">"</h1>"</span>
                                </div>
                                <div class="ml-4">
                                    <span class="text-green-500">"}"</span>
                                </div>
                                <div>
                                    <span class="text-base-content">"}"</span>
                                </div>
                            </div>
                        </div>
                    </div>
                </MockupWindow>
            </Section>

            <Section title="File Manager">
                <MockupWindow>
                    <div class="bg-base-100 h-72">
                        <div class="bg-base-200 px-4 py-2 border-b border-base-300 flex items-center justify-between">
                            <div class="flex items-center gap-2">
                                <div class="flex gap-1">
                                    <div class="w-3 h-3 rounded-full bg-red-500"></div>
                                    <div class="w-3 h-3 rounded-full bg-yellow-500"></div>
                                    <div class="w-3 h-3 rounded-full bg-green-500"></div>
                                </div>
                                <span class="text-sm font-medium">"File Manager"</span>
                            </div>
                        </div>

                        <div class="flex h-full">
                            <div class="w-48 bg-base-50 border-r border-base-300 p-3">
                                <div class="space-y-2">
                                    <div class="flex items-center gap-2 text-sm p-2 rounded bg-primary text-primary-content">
                                        <Icon icon=icondata::AiFolderFilled attr:class="w-4 h-4" />
                                        "Documents"
                                    </div>
                                    <div class="flex items-center gap-2 text-sm p-2 rounded hover:bg-base-200">
                                        <Icon
                                            icon=icondata::AiFolderOutlined
                                            attr:class="w-4 h-4"
                                        />
                                        "Downloads"
                                    </div>
                                    <div class="flex items-center gap-2 text-sm p-2 rounded hover:bg-base-200">
                                        <Icon
                                            icon=icondata::AiFolderOutlined
                                            attr:class="w-4 h-4"
                                        />
                                        "Pictures"
                                    </div>
                                    <div class="flex items-center gap-2 text-sm p-2 rounded hover:bg-base-200">
                                        <Icon
                                            icon=icondata::AiFolderOutlined
                                            attr:class="w-4 h-4"
                                        />
                                        "Music"
                                    </div>
                                </div>
                            </div>

                            <div class="flex-1 p-4">
                                <div class="grid grid-cols-3 gap-4">
                                    <div class="flex flex-col items-center p-2 rounded hover:bg-base-200 cursor-pointer">
                                        <Icon
                                            icon=icondata::AiFileTextOutlined
                                            attr:class="w-8 h-8 mb-2 text-blue-500"
                                        />
                                        <span class="text-xs text-center">"report.pdf"</span>
                                    </div>
                                    <div class="flex flex-col items-center p-2 rounded hover:bg-base-200 cursor-pointer">
                                        <Icon
                                            icon=icondata::AiFileImageOutlined
                                            attr:class="w-8 h-8 mb-2 text-green-500"
                                        />
                                        <span class="text-xs text-center">"photo.jpg"</span>
                                    </div>
                                    <div class="flex flex-col items-center p-2 rounded hover:bg-base-200 cursor-pointer">
                                        <Icon
                                            icon=icondata::AiFileOutlined
                                            attr:class="w-8 h-8 mb-2 text-gray-500"
                                        />
                                        <span class="text-xs text-center">"notes.txt"</span>
                                    </div>
                                    <div class="flex flex-col items-center p-2 rounded hover:bg-base-200 cursor-pointer">
                                        <Icon
                                            icon=icondata::AiFolderOutlined
                                            attr:class="w-8 h-8 mb-2 text-yellow-500"
                                        />
                                        <span class="text-xs text-center">"Projects"</span>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </MockupWindow>
            </Section>

            <Section title="Calculator App">
                <MockupWindow>
                    <div class="bg-base-100 w-64 mx-auto">
                        <div class="bg-base-200 px-4 py-2 border-b border-base-300 flex items-center justify-between">
                            <span class="text-sm font-medium">"Calculator"</span>
                            <div class="flex gap-1">
                                <div class="w-3 h-3 rounded-full bg-red-500"></div>
                                <div class="w-3 h-3 rounded-full bg-yellow-500"></div>
                                <div class="w-3 h-3 rounded-full bg-green-500"></div>
                            </div>
                        </div>

                        <div class="p-4">
                            <div class="bg-base-200 p-4 mb-4 rounded text-right text-xl font-mono border">
                                "0"
                            </div>

                            <div class="grid grid-cols-4 gap-2">
                                <Button size=ButtonSize::Sm class="aspect-square">
                                    "C"
                                </Button>
                                <Button size=ButtonSize::Sm class="aspect-square">
                                    "+/-"
                                </Button>
                                <Button size=ButtonSize::Sm class="aspect-square">
                                    "%"
                                </Button>
                                <Button
                                    size=ButtonSize::Sm
                                    class="aspect-square"
                                    color=ButtonColor::Primary
                                >
                                    "÷"
                                </Button>

                                <Button size=ButtonSize::Sm class="aspect-square">
                                    "7"
                                </Button>
                                <Button size=ButtonSize::Sm class="aspect-square">
                                    "8"
                                </Button>
                                <Button size=ButtonSize::Sm class="aspect-square">
                                    "9"
                                </Button>
                                <Button
                                    size=ButtonSize::Sm
                                    class="aspect-square"
                                    color=ButtonColor::Primary
                                >
                                    "×"
                                </Button>

                                <Button size=ButtonSize::Sm class="aspect-square">
                                    "4"
                                </Button>
                                <Button size=ButtonSize::Sm class="aspect-square">
                                    "5"
                                </Button>
                                <Button size=ButtonSize::Sm class="aspect-square">
                                    "6"
                                </Button>
                                <Button
                                    size=ButtonSize::Sm
                                    class="aspect-square"
                                    color=ButtonColor::Primary
                                >
                                    "-"
                                </Button>

                                <Button size=ButtonSize::Sm class="aspect-square">
                                    "1"
                                </Button>
                                <Button size=ButtonSize::Sm class="aspect-square">
                                    "2"
                                </Button>
                                <Button size=ButtonSize::Sm class="aspect-square">
                                    "3"
                                </Button>
                                <Button
                                    size=ButtonSize::Sm
                                    class="aspect-square"
                                    color=ButtonColor::Primary
                                >
                                    "+"
                                </Button>

                                <Button size=ButtonSize::Sm class="aspect-square col-span-2">
                                    "0"
                                </Button>
                                <Button size=ButtonSize::Sm class="aspect-square">
                                    "."
                                </Button>
                                <Button
                                    size=ButtonSize::Sm
                                    class="aspect-square"
                                    color=ButtonColor::Success
                                >
                                    "="
                                </Button>
                            </div>
                        </div>
                    </div>
                </MockupWindow>
            </Section>
        </ContentLayout>
    }
}
