use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn CardDemo() -> impl IntoView {
    view! {
        <ContentLayout
            title="Card"
            description="Cards are used to group related information in a flexible and extensible container"
        >
            <Section title="Basic Card">
                <Card class="w-96 bg-base-100 shadow-xl">
                    <CardBody>
                        <CardTitle>"Card title!"</CardTitle>
                        <p>"If a dog chews shoes whose shoes does he choose?"</p>
                        <CardActions class="justify-end">
                            <Button color=ButtonColor::Primary>"Buy Now"</Button>
                        </CardActions>
                    </CardBody>
                </Card>
            </Section>

            <Section title="Card with Image">
                <Card class="w-96 bg-base-100 shadow-xl">
                    <figure>
                        <img src="https://picsum.photos/400/200?random=1" alt="Demo image" />
                    </figure>
                    <CardBody>
                        <CardTitle>"Shoes!"</CardTitle>
                        <p>"If a dog chews shoes whose shoes does he choose?"</p>
                        <CardActions class="justify-end">
                            <Button color=ButtonColor::Primary>"Buy Now"</Button>
                        </CardActions>
                    </CardBody>
                </Card>
            </Section>

            <Section title="Styles" row=true>
                <Card style=CardStyle::Default class="w-64 bg-base-100 shadow-xl">
                    <CardBody>
                        <CardTitle>"Default"</CardTitle>
                        <p>"Default card style"</p>
                    </CardBody>
                </Card>
                <Card style=CardStyle::Border class="w-64 bg-base-100 shadow-xl border-2 border-base-300">
                    <CardBody>
                        <CardTitle>"Border"</CardTitle>
                        <p>"Card with border"</p>
                    </CardBody>
                </Card>
                <Card style=CardStyle::Dash class="w-64 bg-base-100 shadow-xl border-2 border-base-300">
                    <CardBody>
                        <CardTitle>"Dash"</CardTitle>
                        <p>"Card with dashed border"</p>
                    </CardBody>
                </Card>
            </Section>

            <Section title="Sizes" row=true>
                <Card size=CardSize::Xs class="bg-base-100 shadow-xl">
                    <CardBody>
                        <CardTitle>"XS"</CardTitle>
                        <p>"Extra small card"</p>
                    </CardBody>
                </Card>
                <Card size=CardSize::Sm class="bg-base-100 shadow-xl">
                    <CardBody>
                        <CardTitle>"SM"</CardTitle>
                        <p>"Small card"</p>
                    </CardBody>
                </Card>
                <Card size=CardSize::Lg class="bg-base-100 shadow-xl">
                    <CardBody>
                        <CardTitle>"LG"</CardTitle>
                        <p>"Large card"</p>
                    </CardBody>
                </Card>
                <Card size=CardSize::Xl class="bg-base-100 shadow-xl">
                    <CardBody>
                        <CardTitle>"XL"</CardTitle>
                        <p>"Extra large card"</p>
                    </CardBody>
                </Card>
            </Section>

            <Section title="Layout Options">
                <div class="flex flex-col gap-4">
                    <Card side=true class="w-96 bg-base-100 shadow-xl">
                        <figure>
                            <img
                                src="https://picsum.photos/200/200?random=3"
                                alt="Side image"
                                class="w-32 h-32 object-cover"
                            />
                        </figure>
                        <CardBody>
                            <CardTitle>"Side Layout"</CardTitle>
                            <p>"Card with side-by-side layout"</p>
                            <CardActions class="justify-end">
                                <Button color=ButtonColor::Primary size=ButtonSize::Sm>
                                    "Action"
                                </Button>
                            </CardActions>
                        </CardBody>
                    </Card>

                    <Card image_full=true class="w-96 h-64 bg-base-100 shadow-xl">
                        <figure>
                            <img src="https://picsum.photos/400/300?random=4" alt="Full image" />
                        </figure>
                        <CardBody class="text-neutral-content">
                            <CardTitle>"Image Full"</CardTitle>
                            <p>"Background image covers entire card"</p>
                            <CardActions class="justify-end">
                                <Button color=ButtonColor::Primary>"Action"</Button>
                            </CardActions>
                        </CardBody>
                    </Card>
                </div>
            </Section>
        </ContentLayout>
    }
}
