use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn JoinDemo() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <h1 class="text-3xl font-bold">"Join"</h1>
            <p class="text-base-content/70">
                "Join component is used to group elements together"
            </p>

            <div class="space-y-4">
                <h2 class="text-xl font-semibold">"Basic Join with Buttons"</h2>
                <div class="join">
                    <Button class="join-item">"Button"</Button>
                    <Button class="join-item">"Button"</Button>
                    <Button class="join-item">"Button"</Button>
                </div>

                <h2 class="text-xl font-semibold">"Join with Different Button Colors"</h2>
                <div class="join">
                    <Button color=Signal::derive(|| ButtonColor::Primary) class="join-item">"Primary"</Button>
                    <Button color=Signal::derive(|| ButtonColor::Secondary) class="join-item">"Secondary"</Button>
                    <Button color=Signal::derive(|| ButtonColor::Accent) class="join-item">"Accent"</Button>
                </div>

                <h2 class="text-xl font-semibold">"Join with Radio Buttons"</h2>
                <div class="join">
                    <input class="join-item btn" type="radio" name="options" aria-label="Radio 1" />
                    <input class="join-item btn" type="radio" name="options" aria-label="Radio 2" />
                    <input class="join-item btn" type="radio" name="options" aria-label="Radio 3" />
                </div>

                <h2 class="text-xl font-semibold">"Vertical Join"</h2>
                <div class="join join-vertical">
                    <Button class="join-item">"Button"</Button>
                    <Button class="join-item">"Button"</Button>
                    <Button class="join-item">"Button"</Button>
                </div>

                <h2 class="text-xl font-semibold">"Join with Input and Button"</h2>
                <div class="join">
                    <Input placeholder="Search..." class="join-item input-bordered w-full max-w-xs" />
                    <Button color=Signal::derive(|| ButtonColor::Primary) class="join-item">"Go"</Button>
                </div>

                <h2 class="text-xl font-semibold">"Join with Select and Button"</h2>
                <div class="join">
                    <Select class="join-item select-bordered">
                        <option disabled selected>"Pick category"</option>
                        <option>"Sci-fi"</option>
                        <option>"Drama"</option>
                        <option>"Action"</option>
                    </Select>
                    <Button color=Signal::derive(|| ButtonColor::Primary) class="join-item">"Filter"</Button>
                </div>

                <h2 class="text-xl font-semibold">"Join with Multiple Inputs"</h2>
                <div class="join">
                    <Input placeholder="First name" class="join-item input-bordered w-full max-w-xs" />
                    <Input placeholder="Last name" class="join-item input-bordered w-full max-w-xs" />
                    <Button color=Signal::derive(|| ButtonColor::Success) class="join-item">"Submit"</Button>
                </div>

                <h2 class="text-xl font-semibold">"Toolbar with Join"</h2>
                <div class="join">
                    <Button style=Signal::derive(|| ButtonStyle::Outline) class="join-item">"Bold"</Button>
                    <Button style=Signal::derive(|| ButtonStyle::Outline) class="join-item">"Italic"</Button>
                    <Button style=Signal::derive(|| ButtonStyle::Outline) class="join-item">"Underline"</Button>
                </div>

                <h2 class="text-xl font-semibold">"Pagination with Join"</h2>
                <div class="join">
                    <Button style=Signal::derive(|| ButtonStyle::Outline) class="join-item">"«"</Button>
                    <Button style=Signal::derive(|| ButtonStyle::Outline) class="join-item">"1"</Button>
                    <Button color=Signal::derive(|| ButtonColor::Primary) class="join-item">"2"</Button>
                    <Button style=Signal::derive(|| ButtonStyle::Outline) class="join-item">"3"</Button>
                    <Button style=Signal::derive(|| ButtonStyle::Outline) class="join-item">"4"</Button>
                    <Button style=Signal::derive(|| ButtonStyle::Outline) class="join-item">"»"</Button>
                </div>

                <h2 class="text-xl font-semibold">"Responsive Design Examples"</h2>
                <div class="space-y-4">
                    <div class="join w-full">
                        <Input placeholder="Email address" class="join-item input-bordered flex-1" />
                        <Button color=Signal::derive(|| ButtonColor::Primary) class="join-item">"Subscribe"</Button>
                    </div>

                    <div class="join w-full">
                        <Select class="join-item select-bordered flex-1">
                            <option disabled selected>"Choose a framework"</option>
                            <option>"React"</option>
                            <option>"Vue"</option>
                            <option>"Leptos"</option>
                            <option>"Svelte"</option>
                        </Select>
                        <Button color=Signal::derive(|| ButtonColor::Info) class="join-item">"Learn"</Button>
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Complex Form with Join"</h2>
                <Card class="bg-base-100 shadow-xl">
                    <CardBody>
                        <h2 class="card-title">"Search Products"</h2>
                        
                        <div class="space-y-4">
                            <div class="join w-full">
                                <Input placeholder="Search products..." class="join-item input-bordered flex-1" />
                                <Select class="join-item select-bordered">
                                    <option disabled selected>"Category"</option>
                                    <option>"Electronics"</option>
                                    <option>"Clothing"</option>
                                    <option>"Books"</option>
                                    <option>"Home"</option>
                                </Select>
                                <Button color=Signal::derive(|| ButtonColor::Primary) class="join-item">"Search"</Button>
                            </div>

                            <div class="join">
                                <input class="join-item btn btn-sm" type="radio" name="sort" aria-label="Price: Low to High" />
                                <input class="join-item btn btn-sm" type="radio" name="sort" aria-label="Price: High to Low" />
                                <input class="join-item btn btn-sm" type="radio" name="sort" aria-label="Newest First" />
                                <input class="join-item btn btn-sm" type="radio" name="sort" aria-label="Best Rating" />
                            </div>
                        </div>
                    </CardBody>
                </Card>

                <h2 class="text-xl font-semibold">"Social Media Actions"</h2>
                <div class="join">
                    <Button size=Signal::derive(|| ButtonSize::Sm) style=Signal::derive(|| ButtonStyle::Outline) class="join-item">
                        "👍 Like"
                    </Button>
                    <Button size=Signal::derive(|| ButtonSize::Sm) style=Signal::derive(|| ButtonStyle::Outline) class="join-item">
                        "💬 Comment"
                    </Button>
                    <Button size=Signal::derive(|| ButtonSize::Sm) style=Signal::derive(|| ButtonStyle::Outline) class="join-item">
                        "🔗 Share"
                    </Button>
                </div>

                <h2 class="text-xl font-semibold">"File Upload with Join"</h2>
                <div class="join">
                    <input type="file" class="join-item file-input file-input-bordered flex-1" />
                    <Button color=Signal::derive(|| ButtonColor::Success) class="join-item">"Upload"</Button>
                </div>
            </div>
        </div>
    }
}