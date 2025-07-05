use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn MaskDemo() -> impl IntoView {
    view! {
        // <div class="space-y-6">
        //     <h1 class="text-3xl font-bold">"Mask"</h1>
        //     <p class="text-base-content/70">
        //         "Mask applies CSS mask to show only a part of an element in the specified shape"
        //     </p>

        //     <div class="space-y-4">
        //         <h2 class="text-xl font-semibold">"Basic Masks"</h2>
        //         <div class="flex flex-wrap gap-4">
        //             <Mask
        //                 mask_type=MaskType::Circle
        //                 class="w-20 h-20 bg-gradient-to-r from-purple-500 to-pink-500"
        //             />
        //             <Mask
        //                 mask_type=MaskType::Heart
        //                 class="w-20 h-20 bg-gradient-to-r from-red-500 to-pink-500"
        //             />
        //             <Mask
        //                 mask_type=MaskType::Star
        //                 class="w-20 h-20 bg-gradient-to-r from-yellow-400 to-orange-500"
        //             />
        //             <Mask
        //                 mask_type=MaskType::Triangle
        //                 class="w-20 h-20 bg-gradient-to-r from-blue-500 to-cyan-500"
        //             />
        //         </div>

        //         <h2 class="text-xl font-semibold">"Polygon Masks"</h2>
        //         <div class="flex flex-wrap gap-4">
        //             <Mask
        //                 mask_type=MaskType::Pentagon
        //                 class="w-20 h-20 bg-gradient-to-r from-green-500 to-teal-500"
        //             />
        //             <Mask
        //                 mask_type=MaskType::Hexagon
        //                 class="w-20 h-20 bg-gradient-to-r from-indigo-500 to-purple-500"
        //             />
        //             <Mask
        //                 mask_type=MaskType::Decagon
        //                 class="w-20 h-20 bg-gradient-to-r from-pink-500 to-rose-500"
        //             />
        //         </div>

        //         <h2 class="text-xl font-semibold">"Shape Masks"</h2>
        //         <div class="flex flex-wrap gap-4">
        //             <Mask
        //                 mask_type=MaskType::Squircle
        //                 class="w-20 h-20 bg-gradient-to-r from-amber-500 to-yellow-500"
        //             />
        //             <Mask
        //                 mask_type=MaskType::Parallelogram
        //                 class="w-20 h-20 bg-gradient-to-r from-lime-500 to-green-500"
        //             />
        //             <Mask
        //                 mask_type=MaskType::Diamond
        //                 class="w-20 h-20 bg-gradient-to-r from-cyan-500 to-blue-500"
        //             />
        //         </div>

        //         <h2 class="text-xl font-semibold">"Masks with Images"</h2>
        //         <div class="flex flex-wrap gap-4">
        //             <Mask
        //                 mask_type=MaskType::Circle
        //                 class="w-24 h-24"
        //             >
        //                 <img
        //                     src="https://picsum.photos/96/96?random=1"
        //                     alt="Masked image"
        //                     class="w-full h-full object-cover"
        //                 />
        //             </Mask>
        //             <Mask
        //                 mask_type=MaskType::Heart
        //                 class="w-24 h-24"
        //             >
        //                 <img
        //                     src="https://picsum.photos/96/96?random=2"
        //                     alt="Masked image"
        //                     class="w-full h-full object-cover"
        //                 />
        //             </Mask>
        //             <Mask
        //                 mask_type=MaskType::Star
        //                 class="w-24 h-24"
        //             >
        //                 <img
        //                     src="https://picsum.photos/96/96?random=3"
        //                     alt="Masked image"
        //                     class="w-full h-full object-cover"
        //                 />
        //             </Mask>
        //             <Mask
        //                 mask_type=MaskType::Hexagon
        //                 class="w-24 h-24"
        //             >
        //                 <img
        //                     src="https://picsum.photos/96/96?random=4"
        //                     alt="Masked image"
        //                     class="w-full h-full object-cover"
        //                 />
        //             </Mask>
        //         </div>

        //         <h2 class="text-xl font-semibold">"Different Sizes"</h2>
        //         <div class="flex flex-wrap items-center gap-4">
        //             <Mask
        //                 mask_type=MaskType::Star
        //                 class="w-12 h-12 bg-gradient-to-r from-purple-400 to-pink-400"
        //             />
        //             <Mask
        //                 mask_type=MaskType::Star
        //                 class="w-16 h-16 bg-gradient-to-r from-purple-400 to-pink-400"
        //             />
        //             <Mask
        //                 mask_type=MaskType::Star
        //                 class="w-20 h-20 bg-gradient-to-r from-purple-400 to-pink-400"
        //             />
        //             <Mask
        //                 mask_type=MaskType::Star
        //                 class="w-24 h-24 bg-gradient-to-r from-purple-400 to-pink-400"
        //             />
        //             <Mask
        //                 mask_type=MaskType::Star
        //                 class="w-32 h-32 bg-gradient-to-r from-purple-400 to-pink-400"
        //             />
        //         </div>

        //         <h2 class="text-xl font-semibold">"Creative Examples"</h2>
        //         <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
        //             <Card class="bg-base-100 shadow-xl">
        //                 <CardBody>
        //                     <h3 class="card-title">"Profile Card"</h3>
        //                     <div class="flex items-center gap-4">
        //                         <Mask
        //                             mask_type=MaskType::Circle
        //                             class="w-16 h-16"
        //                         >
        //                             <img
        //                                 src="https://picsum.photos/64/64?random=10"
        //                                 alt="Profile"
        //                                 class="w-full h-full object-cover"
        //                             />
        //                         </Mask>
        //                         <div>
        //                             <h4 class="font-semibold">"John Doe"</h4>
        //                             <p class="text-sm opacity-60">"Software Engineer"</p>
        //                         </div>
        //                     </div>
        //                 </CardBody>
        //             </Card>

        //             <Card class="bg-base-100 shadow-xl">
        //                 <CardBody>
        //                     <h3 class="card-title">"Achievement Badge"</h3>
        //                     <div class="flex items-center gap-4">
        //                         <Mask
        //                             mask_type=MaskType::Star
        //                             class="w-16 h-16 bg-gradient-to-r from-yellow-400 to-orange-500"
        //                         />
        //                         <div>
        //                             <h4 class="font-semibold">"Star Performer"</h4>
        //                             <p class="text-sm opacity-60">"Top contributor this month"</p>
        //                         </div>
        //                     </div>
        //                 </CardBody>
        //             </Card>
        //         </div>
        //     </div>
        // </div>
    }
}
