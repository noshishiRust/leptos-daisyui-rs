use leptos::prelude::*;

#[component]
pub fn FilterDemo() -> impl IntoView {
    let (_selected_category, _set_selected_category) = signal("all".to_string());
    let (_selected_price, _set_selected_price) = signal("all".to_string());
    let (_selected_rating, _set_selected_rating) = signal("all".to_string());
    let (_search_query, _set_search_query) = signal("".to_string());

    view! {
        // <div class="space-y-6">
        //     <h1 class="text-3xl font-bold">"Filter"</h1>
        //     <p class="text-base-content/70">
        //         "Filter components for searching and filtering data"
        //     </p>

        //     <div class="space-y-4">
        //         <h2 class="text-xl font-semibold">"Search Filter"</h2>
        //         <Card class="bg-base-100 shadow-xl">
        //             <CardBody>
        //                 <div class="flex flex-col sm:flex-row gap-4">
        //                     <div class="flex-1">
        //                         <Input
        //                             placeholder="Search products..."
        //                             class="input input-bordered w-full"
        //                             value=search_query
        //                             on:input=move |ev| {
        //                                 set_search_query(event_target_value(&ev));
        //                             }
        //                         />
        //                     </div>
        //                     <Button color=ButtonColor::Primary>
        //                         <Icon icon=icondata::AiSearchOutlined class="mr-2" />
        //                         "Search"
        //                     </Button>
        //                 </div>
        //             </CardBody>
        //         </Card>

        //         <h2 class="text-xl font-semibold">"Category Filters"</h2>
        //         <Card class="bg-base-100 shadow-xl">
        //             <CardBody>
        //                 <h3 class="card-title mb-4">"Product Categories"</h3>
        //                 <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
        //                     <Filter
        //                         active=move || selected_category() == "all"
        //                         on:click=move |_| set_selected_category("all".to_string())
        //                     >
        //                         <Icon icon=icondata::AiAppstoreOutlined class="mr-2" />
        //                         "All"
        //                     </Filter>
        //                     <Filter
        //                         active=move || selected_category() == "electronics"
        //                         on:click=move |_| set_selected_category("electronics".to_string())
        //                     >
        //                         <Icon icon=icondata::AiLaptopOutlined class="mr-2" />
        //                         "Electronics"
        //                     </Filter>
        //                     <Filter
        //                         active=move || selected_category() == "clothing"
        //                         on:click=move |_| set_selected_category("clothing".to_string())
        //                     >
        //                         <Icon icon=icondata::AiShoppingOutlined class="mr-2" />
        //                         "Clothing"
        //                     </Filter>
        //                     <Filter
        //                         active=move || selected_category() == "books"
        //                         on:click=move |_| set_selected_category("books".to_string())
        //                     >
        //                         <Icon icon=icondata::AiBookOutlined class="mr-2" />
        //                         "Books"
        //                     </Filter>
        //                 </div>
        //             </CardBody>
        //         </Card>

        //         <h2 class="text-xl font-semibold">"Price Range Filter"</h2>
        //         <Card class="bg-base-100 shadow-xl">
        //             <CardBody>
        //                 <h3 class="card-title mb-4">"Filter by Price"</h3>
        //                 <div class="space-y-4">
        //                     <div class="flex flex-wrap gap-2">
        //                         <Filter
        //                             active=move || selected_price() == "all"
        //                             on:click=move |_| set_selected_price("all".to_string())
        //                         >
        //                             "All Prices"
        //                         </Filter>
        //                         <Filter
        //                             active=move || selected_price() == "under-25"
        //                             on:click=move |_| set_selected_price("under-25".to_string())
        //                         >
        //                             "Under $25"
        //                         </Filter>
        //                         <Filter
        //                             active=move || selected_price() == "25-50"
        //                             on:click=move |_| set_selected_price("25-50".to_string())
        //                         >
        //                             "$25 - $50"
        //                         </Filter>
        //                         <Filter
        //                             active=move || selected_price() == "50-100"
        //                             on:click=move |_| set_selected_price("50-100".to_string())
        //                         >
        //                             "$50 - $100"
        //                         </Filter>
        //                         <Filter
        //                             active=move || selected_price() == "over-100"
        //                             on:click=move |_| set_selected_price("over-100".to_string())
        //                         >
        //                             "Over $100"
        //                         </Filter>
        //                     </div>
        //                     <div class="form-control">
        //                         <Label class="label">
        //                             <LabelText>"Custom Range"</LabelText>
        //                         </Label>
        //                         <div class="flex gap-2">
        //                             <Input placeholder="Min" class="input input-bordered input-sm" />
        //                             <Input placeholder="Max" class="input input-bordered input-sm" />
        //                             <Button size=ButtonSize::Sm style=ButtonStyle::Outline>
        //                                 "Apply"
        //                             </Button>
        //                         </div>
        //                     </div>
        //                 </div>
        //             </CardBody>
        //         </Card>

        //         <h2 class="text-xl font-semibold">"Rating Filter"</h2>
        //         <Card class="bg-base-100 shadow-xl">
        //             <CardBody>
        //                 <h3 class="card-title mb-4">"Filter by Rating"</h3>
        //                 <div class="space-y-2">
        //                     <Filter
        //                         active=move || selected_rating() == "all"
        //                         on:click=move |_| set_selected_rating("all".to_string())
        //                         class="w-full justify-start"
        //                     >
        //                         "All Ratings"
        //                     </Filter>
        //                     <Filter
        //                         active=move || selected_rating() == "5"
        //                         on:click=move |_| set_selected_rating("5".to_string())
        //                         class="w-full justify-start"
        //                     >
        //                         <div class="flex items-center gap-2">
        //                             <Rating value=5 size=RatingSize::Sm />
        //                             <span>"5 Stars"</span>
        //                         </div>
        //                     </Filter>
        //                     <Filter
        //                         active=move || selected_rating() == "4"
        //                         on:click=move |_| set_selected_rating("4".to_string())
        //                         class="w-full justify-start"
        //                     >
        //                         <div class="flex items-center gap-2">
        //                             <Rating value=4 size=RatingSize::Sm />
        //                             <span>"4 Stars & Up"</span>
        //                         </div>
        //                     </Filter>
        //                     <Filter
        //                         active=move || selected_rating() == "3"
        //                         on:click=move |_| set_selected_rating("3".to_string())
        //                         class="w-full justify-start"
        //                     >
        //                         <div class="flex items-center gap-2">
        //                             <Rating value=3 size=RatingSize::Sm />
        //                             <span>"3 Stars & Up"</span>
        //                         </div>
        //                     </Filter>
        //                 </div>
        //             </CardBody>
        //         </Card>

        //         <h2 class="text-xl font-semibold">"Advanced Filters"</h2>
        //         <Card class="bg-base-100 shadow-xl">
        //             <CardBody>
        //                 <h3 class="card-title mb-4">"Product Filters"</h3>
        //                 <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
        //                     <div>
        //                         <h4 class="font-semibold mb-3">"Brand"</h4>
        //                         <div class="space-y-2">
        //                             <Label class="label cursor-pointer justify-start gap-2">
        //                                 <Checkbox />
        //                                 <LabelText>"Apple"</LabelText>
        //                             </Label>
        //                             <Label class="label cursor-pointer justify-start gap-2">
        //                                 <Checkbox />
        //                                 <LabelText>"Samsung"</LabelText>
        //                             </Label>
        //                             <Label class="label cursor-pointer justify-start gap-2">
        //                                 <Checkbox />
        //                                 <LabelText>"Google"</LabelText>
        //                             </Label>
        //                             <Label class="label cursor-pointer justify-start gap-2">
        //                                 <Checkbox />
        //                                 <LabelText>"Microsoft"</LabelText>
        //                             </Label>
        //                         </div>
        //                     </div>
        //                     <div>
        //                         <h4 class="font-semibold mb-3">"Features"</h4>
        //                         <div class="space-y-2">
        //                             <Label class="label cursor-pointer justify-start gap-2">
        //                                 <Checkbox />
        //                                 <LabelText>"Wireless"</LabelText>
        //                             </Label>
        //                             <Label class="label cursor-pointer justify-start gap-2">
        //                                 <Checkbox />
        //                                 <LabelText>"Waterproof"</LabelText>
        //                             </Label>
        //                             <Label class="label cursor-pointer justify-start gap-2">
        //                                 <Checkbox />
        //                                 <LabelText>"Fast Charging"</LabelText>
        //                             </Label>
        //                             <Label class="label cursor-pointer justify-start gap-2">
        //                                 <Checkbox />
        //                                 <LabelText>"Bluetooth"</LabelText>
        //                             </Label>
        //                         </div>
        //                     </div>
        //                     <div>
        //                         <h4 class="font-semibold mb-3">"Availability"</h4>
        //                         <div class="space-y-2">
        //                             <Label class="label cursor-pointer justify-start gap-2">
        //                                 <Radio name="availability" />
        //                                 <LabelText>"In Stock"</LabelText>
        //                             </Label>
        //                             <Label class="label cursor-pointer justify-start gap-2">
        //                                 <Radio name="availability" />
        //                                 <LabelText>"Out of Stock"</LabelText>
        //                             </Label>
        //                             <Label class="label cursor-pointer justify-start gap-2">
        //                                 <Radio name="availability" />
        //                                 <LabelText>"Pre-order"</LabelText>
        //                             </Label>
        //                         </div>
        //                     </div>
        //                 </div>
        //                 <div class="mt-6 flex gap-2">
        //                     <Button color=ButtonColor::Primary>
        //                         "Apply Filters"
        //                     </Button>
        //                     <Button style=ButtonStyle::Ghost>
        //                         "Clear All"
        //                     </Button>
        //                 </div>
        //             </CardBody>
        //         </Card>

        //         <h2 class="text-xl font-semibold">"Sort Options"</h2>
        //         <Card class="bg-base-100 shadow-xl">
        //             <CardBody>
        //                 <h3 class="card-title mb-4">"Sort Results"</h3>
        //                 <div class="flex flex-wrap gap-2">
        //                     <Filter active=true>
        //                         "Relevance"
        //                     </Filter>
        //                     <Filter>
        //                         "Price: Low to High"
        //                     </Filter>
        //                     <Filter>
        //                         "Price: High to Low"
        //                     </Filter>
        //                     <Filter>
        //                         "Newest First"
        //                     </Filter>
        //                     <Filter>
        //                         "Best Rating"
        //                     </Filter>
        //                     <Filter>
        //                         "Most Popular"
        //                     </Filter>
        //                 </div>
        //             </CardBody>
        //         </Card>

        //         <h2 class="text-xl font-semibold">"Filter Results"</h2>
        //         <Card class="bg-base-100 shadow-xl">
        //             <CardBody>
        //                 <div class="flex justify-between items-center mb-4">
        //                     <h3 class="card-title">"Products"</h3>
        //                     <div class="flex items-center gap-2">
        //                         <span class="text-sm">"Showing 1-12 of 48 results"</span>
        //                         <Button size=ButtonSize::Sm style=ButtonStyle::Ghost>
        //                             <Icon icon=icondata::AiFilterOutlined />
        //                         </Button>
        //                     </div>
        //                 </div>

        //                 <div class="mb-4">
        //                     <h4 class="font-semibold mb-2">"Active Filters"</h4>
        //                     <div class="flex flex-wrap gap-2">
        //                         <Badge color=BadgeColor::Primary class="gap-2">
        //                             "Electronics"
        //                             <Button size=ButtonSize::Xs style=ButtonStyle::Ghost class="p-0">
        //                                 <Icon icon=icondata::AiCloseOutlined />
        //                             </Button>
        //                         </Badge>
        //                         <Badge color=BadgeColor::Secondary class="gap-2">
        //                             "$25 - $50"
        //                             <Button size=ButtonSize::Xs style=ButtonStyle::Ghost class="p-0">
        //                                 <Icon icon=icondata::AiCloseOutlined />
        //                             </Button>
        //                         </Badge>
        //                         <Badge color=BadgeColor::Accent class="gap-2">
        //                             "4 Stars & Up"
        //                             <Button size=ButtonSize::Xs style=ButtonStyle::Ghost class="p-0">
        //                                 <Icon icon=icondata::AiCloseOutlined />
        //                             </Button>
        //                         </Badge>
        //                     </div>
        //                 </div>

        //                 <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        //                     <Card class="bg-base-200">
        //                         <CardBody class="p-4">
        //                             <div class="bg-base-300 h-32 rounded mb-2 flex items-center justify-center">
        //                                 <Icon icon=icondata::AiLaptopOutlined class="w-8 h-8" />
        //                             </div>
        //                             <h4 class="font-semibold text-sm">"Wireless Headphones"</h4>
        //                             <p class="text-xs opacity-70">"Premium sound quality"</p>
        //                             <div class="flex items-center justify-between mt-2">
        //                                 <span class="font-bold text-primary">"$45"</span>
        //                                 <Rating value=4 size=RatingSize::Sm />
        //                             </div>
        //                         </CardBody>
        //                     </Card>
        //                     <Card class="bg-base-200">
        //                         <CardBody class="p-4">
        //                             <div class="bg-base-300 h-32 rounded mb-2 flex items-center justify-center">
        //                                 <Icon icon=icondata::AiMobileOutlined class="w-8 h-8" />
        //                             </div>
        //                             <h4 class="font-semibold text-sm">"Smartphone Case"</h4>
        //                             <p class="text-xs opacity-70">"Protective and stylish"</p>
        //                             <div class="flex items-center justify-between mt-2">
        //                                 <span class="font-bold text-primary">"$29"</span>
        //                                 <Rating value=5 size=RatingSize::Sm />
        //                             </div>
        //                         </CardBody>
        //                     </Card>
        //                     <Card class="bg-base-200">
        //                         <CardBody class="p-4">
        //                             <div class="bg-base-300 h-32 rounded mb-2 flex items-center justify-center">
        //                                 <Icon icon=icondata::AiUsbOutlined class="w-8 h-8" />
        //                             </div>
        //                             <h4 class="font-semibold text-sm">"USB-C Cable"</h4>
        //                             <p class="text-xs opacity-70">"Fast charging cable"</p>
        //                             <div class="flex items-center justify-between mt-2">
        //                                 <span class="font-bold text-primary">"$15"</span>
        //                                 <Rating value=4 size=RatingSize::Sm />
        //                             </div>
        //                         </CardBody>
        //                     </Card>
        //                 </div>

        //                 <div class="mt-6 flex justify-center">
        //                     <Join>
        //                         <Button class="join-item" style=ButtonStyle::Outline>
        //                             <Icon icon=icondata::AiLeftOutlined />
        //                         </Button>
        //                         <Button class="join-item" color=ButtonColor::Primary>
        //                             "1"
        //                         </Button>
        //                         <Button class="join-item" style=ButtonStyle::Outline>
        //                             "2"
        //                         </Button>
        //                         <Button class="join-item" style=ButtonStyle::Outline>
        //                             "3"
        //                         </Button>
        //                         <Button class="join-item" style=ButtonStyle::Outline>
        //                             <Icon icon=icondata::AiRightOutlined />
        //                         </Button>
        //                     </Join>
        //                 </div>
        //             </CardBody>
        //         </Card>
        //     </div>
        // </div>
    }
}
