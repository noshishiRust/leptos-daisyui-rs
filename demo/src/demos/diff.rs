use leptos::prelude::*;
use leptos_daisyui_rs::components::*;
use leptos_icons::Icon;

#[component]
pub fn DiffDemo() -> impl IntoView {
    view! {
        // <div class="space-y-6">
        //     <h1 class="text-3xl font-bold">"Diff"</h1>
        //     <p class="text-base-content/70">
        //         "Diff component shows the difference between two pieces of content"
        //     </p>

        //     <div class="space-y-4">
        //         <h2 class="text-xl font-semibold">"Basic Diff"</h2>
        //         <Diff class="aspect-[16/9]">
        //             <DiffItem1>
        //                 <div class="bg-primary text-primary-content text-9xl font-black grid place-content-center">
        //                     "DAISY"
        //                 </div>
        //             </DiffItem1>
        //             <DiffItem2>
        //                 <div class="bg-base-200 text-9xl font-black grid place-content-center">
        //                     "DAISY"
        //                 </div>
        //             </DiffItem2>
        //             <DiffResizer />
        //         </Diff>

        //         <h2 class="text-xl font-semibold">"Text Comparison"</h2>
        //         <Diff class="aspect-[16/9]">
        //             <DiffItem1>
        //                 <div class="bg-warning text-warning-content p-8 text-center">
        //                     <h2 class="text-2xl font-bold mb-4">"Old Version"</h2>
        //                     <p>"Welcome to our website! We offer great services."</p>
        //                     <p class="mt-2">"Contact us at: info@example.com"</p>
        //                     <p class="mt-2">"Phone: (555) 123-4567"</p>
        //                 </div>
        //             </DiffItem1>
        //             <DiffItem2>
        //                 <div class="bg-success text-success-content p-8 text-center">
        //                     <h2 class="text-2xl font-bold mb-4">"New Version"</h2>
        //                     <p>"Welcome to our improved website! We offer amazing services and support."</p>
        //                     <p class="mt-2">"Contact us at: hello@newdomain.com"</p>
        //                     <p class="mt-2">"Phone: (555) 987-6543"</p>
        //                     <p class="mt-2">"Live Chat Available 24/7"</p>
        //                 </div>
        //             </DiffItem2>
        //             <DiffResizer />
        //         </Diff>

        //         <h2 class="text-xl font-semibold">"Image Comparison"</h2>
        //         <Diff class="aspect-[16/9]">
        //             <DiffItem1>
        //                 <img
        //                     alt="Before"
        //                     src="https://picsum.photos/800/600?random=1&blur=2"
        //                     class="w-full h-full object-cover"
        //                 />
        //             </DiffItem1>
        //             <DiffItem2>
        //                 <img
        //                     alt="After"
        //                     src="https://picsum.photos/800/600?random=1"
        //                     class="w-full h-full object-cover"
        //                 />
        //             </DiffItem2>
        //             <DiffResizer />
        //         </Diff>

        //         <h2 class="text-xl font-semibold">"UI Component Comparison"</h2>
        //         <Diff class="aspect-[16/9]">
        //             <DiffItem1>
        //                 <div class="bg-base-300 p-8 h-full">
        //                     <h3 class="text-lg font-bold mb-4">"Old Design"</h3>
        //                     <div class="space-y-4">
        //                         <button class="btn btn-sm">
        //                             "Small Button"
        //                         </button>
        //                         <div class="alert alert-info">
        //                             <Icon icon=icondata::AiInfoCircleOutlined />
        //                             "Basic alert message"
        //                         </div>
        //                         <div class="card bg-base-100 shadow-md">
        //                             <div class="card-body">
        //                                 <h2 class="card-title text-sm">"Simple Card"</h2>
        //                                 <p class="text-xs">"Basic card content"</p>
        //                             </div>
        //                         </div>
        //                     </div>
        //                 </div>
        //             </DiffItem1>
        //             <DiffItem2>
        //                 <div class="bg-gradient-to-br from-purple-500 to-blue-500 p-8 h-full">
        //                     <h3 class="text-lg font-bold mb-4 text-white">"New Design"</h3>
        //                     <div class="space-y-4">
        //                         <Button color=ButtonColor::Primary size=ButtonSize::Lg>
        //                             <Icon icon=icondata::AiStarOutlined class="mr-2" />
        //                             "Enhanced Button"
        //                         </Button>
        //                         <Alert color=AlertColor::Success>
        //                             <Icon icon=icondata::AiCheckCircleOutlined />
        //                             "Improved alert with better styling"
        //                         </Alert>
        //                         <Card class="bg-white/90 backdrop-blur shadow-xl">
        //                             <CardBody>
        //                                 <h2 class="card-title">"Modern Card"</h2>
        //                                 <p>"Enhanced card with better typography and spacing"</p>
        //                                 <div class="card-actions justify-end">
        //                                     <Button color=ButtonColor::Primary size=ButtonSize::Sm>
        //                                         "Action"
        //                                     </Button>
        //                                 </div>
        //                             </CardBody>
        //                         </Card>
        //                     </div>
        //                 </div>
        //             </DiffItem2>
        //             <DiffResizer />
        //         </Diff>

        //         <h2 class="text-xl font-semibold">"Code Comparison"</h2>
        //         <Diff class="aspect-[16/9]">
        //             <DiffItem1>
        //                 <div class="bg-base-300 p-6 h-full overflow-auto">
        //                     <h3 class="font-bold mb-4 text-error">"❌ Before (HTML)"</h3>
        //                     <div class="mockup-code text-sm">
        //                         <pre data-prefix="1">
        //                             <code>"<div class=\"container\">"</code>
        //                         </pre>
        //                         <pre data-prefix="2">
        //                             <code>"  <div class=\"row\">"</code>
        //                         </pre>
        //                         <pre data-prefix="3">
        //                             <code>"    <div class=\"col-12\">"</code>
        //                         </pre>
        //                         <pre data-prefix="4">
        //                             <code>"      <h1>Title</h1>"</code>
        //                         </pre>
        //                         <pre data-prefix="5">
        //                             <code>"      <p>Content</p>"</code>
        //                         </pre>
        //                         <pre data-prefix="6">
        //                             <code>"      <button onclick=\"alert('Hi')\">"</code>
        //                         </pre>
        //                         <pre data-prefix="7">
        //                             <code>"        Click me"</code>
        //                         </pre>
        //                         <pre data-prefix="8">
        //                             <code>"      </button>"</code>
        //                         </pre>
        //                         <pre data-prefix="9">
        //                             <code>"    </div>"</code>
        //                         </pre>
        //                         <pre data-prefix="10">
        //                             <code>"  </div>"</code>
        //                         </pre>
        //                         <pre data-prefix="11">
        //                             <code>"</div>"</code>
        //                         </pre>
        //                     </div>
        //                 </div>
        //             </DiffItem1>
        //             <DiffItem2>
        //                 <div class="bg-base-100 p-6 h-full overflow-auto">
        //                     <h3 class="font-bold mb-4 text-success">"✅ After (Leptos + daisyUI)"</h3>
        //                     <div class="mockup-code text-sm">
        //                         <pre data-prefix="1">
        //                             <code>"<div class=\"container mx-auto\">"</code>
        //                         </pre>
        //                         <pre data-prefix="2">
        //                             <code>"  <Card class=\"shadow-xl\">"</code>
        //                         </pre>
        //                         <pre data-prefix="3">
        //                             <code>"    <CardBody>"</code>
        //                         </pre>
        //                         <pre data-prefix="4">
        //                             <code>"      <h1 class=\"card-title\">"</code>
        //                         </pre>
        //                         <pre data-prefix="5">
        //                             <code>"        \"Title\""</code>
        //                         </pre>
        //                         <pre data-prefix="6">
        //                             <code>"      </h1>"</code>
        //                         </pre>
        //                         <pre data-prefix="7">
        //                             <code>"      <p>\"Content\"</p>"</code>
        //                         </pre>
        //                         <pre data-prefix="8">
        //                             <code>"      <Button"</code>
        //                         </pre>
        //                         <pre data-prefix="9">
        //                             <code>"        color=ButtonColor::Primary"</code>
        //                         </pre>
        //                         <pre data-prefix="10">
        //                             <code>"        on:click=|_| show_alert()"</code>
        //                         </pre>
        //                         <pre data-prefix="11">
        //                             <code>"      >"</code>
        //                         </pre>
        //                         <pre data-prefix="12">
        //                             <code>"        \"Click me\""</code>
        //                         </pre>
        //                         <pre data-prefix="13">
        //                             <code>"      </Button>"</code>
        //                         </pre>
        //                         <pre data-prefix="14">
        //                             <code>"    </CardBody>"</code>
        //                         </pre>
        //                         <pre data-prefix="15">
        //                             <code>"  </Card>"</code>
        //                         </pre>
        //                         <pre data-prefix="16">
        //                             <code>"</div>"</code>
        //                         </pre>
        //                     </div>
        //                 </div>
        //             </DiffItem2>
        //             <DiffResizer />
        //         </Diff>

        //         <h2 class="text-xl font-semibold">"Color Scheme Comparison"</h2>
        //         <Diff class="aspect-[16/9]">
        //             <DiffItem1>
        //                 <div class="bg-gradient-to-br from-gray-800 to-gray-900 p-8 h-full">
        //                     <h3 class="text-white font-bold mb-6">"Dark Theme"</h3>
        //                     <div class="space-y-4">
        //                         <div class="bg-gray-700 p-4 rounded-lg">
        //                             <h4 class="text-white font-semibold">"Navigation"</h4>
        //                             <div class="flex gap-2 mt-2">
        //                                 <span class="px-3 py-1 bg-gray-600 text-white rounded text-sm">"Home"</span>
        //                                 <span class="px-3 py-1 bg-blue-600 text-white rounded text-sm">"About"</span>
        //                                 <span class="px-3 py-1 bg-gray-600 text-white rounded text-sm">"Contact"</span>
        //                             </div>
        //                         </div>
        //                         <div class="bg-gray-700 p-4 rounded-lg">
        //                             <h4 class="text-white font-semibold">"Content Area"</h4>
        //                             <p class="text-gray-300 text-sm mt-2">"Dark mode provides better readability in low-light conditions."</p>
        //                         </div>
        //                         <div class="bg-gray-700 p-4 rounded-lg">
        //                             <h4 class="text-white font-semibold">"Actions"</h4>
        //                             <div class="flex gap-2 mt-2">
        //                                 <button class="px-4 py-2 bg-blue-600 text-white rounded text-sm">"Primary"</button>
        //                                 <button class="px-4 py-2 bg-gray-600 text-white rounded text-sm">"Secondary"</button>
        //                             </div>
        //                         </div>
        //                     </div>
        //                 </div>
        //             </DiffItem1>
        //             <DiffItem2>
        //                 <div class="bg-gradient-to-br from-blue-50 to-indigo-100 p-8 h-full">
        //                     <h3 class="text-gray-800 font-bold mb-6">"Light Theme"</h3>
        //                     <div class="space-y-4">
        //                         <div class="bg-white p-4 rounded-lg shadow-sm">
        //                             <h4 class="text-gray-800 font-semibold">"Navigation"</h4>
        //                             <div class="flex gap-2 mt-2">
        //                                 <span class="px-3 py-1 bg-gray-200 text-gray-700 rounded text-sm">"Home"</span>
        //                                 <span class="px-3 py-1 bg-blue-500 text-white rounded text-sm">"About"</span>
        //                                 <span class="px-3 py-1 bg-gray-200 text-gray-700 rounded text-sm">"Contact"</span>
        //                             </div>
        //                         </div>
        //                         <div class="bg-white p-4 rounded-lg shadow-sm">
        //                             <h4 class="text-gray-800 font-semibold">"Content Area"</h4>
        //                             <p class="text-gray-600 text-sm mt-2">"Light mode offers a clean, professional appearance for daytime use."</p>
        //                         </div>
        //                         <div class="bg-white p-4 rounded-lg shadow-sm">
        //                             <h4 class="text-gray-800 font-semibold">"Actions"</h4>
        //                             <div class="flex gap-2 mt-2">
        //                                 <button class="px-4 py-2 bg-blue-500 text-white rounded text-sm">"Primary"</button>
        //                                 <button class="px-4 py-2 bg-gray-300 text-gray-700 rounded text-sm">"Secondary"</button>
        //                             </div>
        //                         </div>
        //                     </div>
        //                 </div>
        //             </DiffItem2>
        //             <DiffResizer />
        //         </Diff>

        //         <h2 class="text-xl font-semibold">"Mobile vs Desktop Layout"</h2>
        //         <Diff class="aspect-[16/9]">
        //             <DiffItem1>
        //                 <div class="bg-base-100 p-4 h-full border-2 border-base-300 mx-auto max-w-xs">
        //                     <h3 class="font-bold mb-4 text-center">"📱 Mobile Layout"</h3>
        //                     <div class="space-y-3">
        //                         <div class="navbar bg-primary text-primary-content rounded">
        //                             <div class="navbar-start">
        //                                 <Icon icon=icondata::AiMenuOutlined class="w-5 h-5" />
        //                             </div>
        //                             <div class="navbar-center">
        //                                 <span class="text-sm font-bold">"App"</span>
        //                             </div>
        //                         </div>
        //                         <div class="grid grid-cols-1 gap-2">
        //                             <div class="card bg-base-200 p-3">
        //                                 <h4 class="font-semibold text-xs">"Card 1"</h4>
        //                                 <p class="text-xs opacity-70">"Mobile-first design"</p>
        //                             </div>
        //                             <div class="card bg-base-200 p-3">
        //                                 <h4 class="font-semibold text-xs">"Card 2"</h4>
        //                                 <p class="text-xs opacity-70">"Stacked vertically"</p>
        //                             </div>
        //                             <div class="card bg-base-200 p-3">
        //                                 <h4 class="font-semibold text-xs">"Card 3"</h4>
        //                                 <p class="text-xs opacity-70">"Touch-friendly"</p>
        //                             </div>
        //                         </div>
        //                         <div class="btm-nav btm-nav-xs">
        //                             <button class="active">
        //                                 <Icon icon=icondata::AiHomeOutlined class="w-4 h-4" />
        //                             </button>
        //                             <button>
        //                                 <Icon icon=icondata::AiSearchOutlined class="w-4 h-4" />
        //                             </button>
        //                             <button>
        //                                 <Icon icon=icondata::AiUserOutlined class="w-4 h-4" />
        //                             </button>
        //                         </div>
        //                     </div>
        //                 </div>
        //             </DiffItem1>
        //             <DiffItem2>
        //                 <div class="bg-base-100 p-6 h-full">
        //                     <h3 class="font-bold mb-4 text-center">"🖥️ Desktop Layout"</h3>
        //                     <div class="h-full flex flex-col">
        //                         <div class="navbar bg-primary text-primary-content rounded mb-4">
        //                             <div class="navbar-start">
        //                                 <span class="font-bold">"Desktop App"</span>
        //                             </div>
        //                             <div class="navbar-center hidden lg:flex">
        //                                 <ul class="menu menu-horizontal px-1 text-sm">
        //                                     <li><a>"Home"</a></li>
        //                                     <li><a>"About"</a></li>
        //                                     <li><a>"Contact"</a></li>
        //                                 </ul>
        //                             </div>
        //                             <div class="navbar-end">
        //                                 <Button size=ButtonSize::Sm style=ButtonStyle::Ghost>
        //                                     <Icon icon=icondata::AiUserOutlined />
        //                                 </Button>
        //                             </div>
        //                         </div>
        //                         <div class="flex-1 grid grid-cols-3 gap-4">
        //                             <div class="card bg-base-200">
        //                                 <div class="card-body p-4">
        //                                     <h4 class="card-title text-sm">"Sidebar"</h4>
        //                                     <p class="text-xs opacity-70">"Navigation menu"</p>
        //                                 </div>
        //                             </div>
        //                             <div class="card bg-base-200">
        //                                 <div class="card-body p-4">
        //                                     <h4 class="card-title text-sm">"Main Content"</h4>
        //                                     <p class="text-xs opacity-70">"Primary workspace"</p>
        //                                 </div>
        //                             </div>
        //                             <div class="card bg-base-200">
        //                                 <div class="card-body p-4">
        //                                     <h4 class="card-title text-sm">"Panel"</h4>
        //                                     <p class="text-xs opacity-70">"Additional info"</p>
        //                                 </div>
        //                             </div>
        //                         </div>
        //                     </div>
        //                 </div>
        //             </DiffItem2>
        //             <DiffResizer />
        //         </Diff>
        //     </div>
        // </div>
    }
}
