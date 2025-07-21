use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn CarouselDemo() -> impl IntoView {
    view! {
        <ContentLayout
            title="Carousel"
            description="Carousel shows multiple items, only one item visible at a time"
        >
            <Section title="Basic Usage" col=true>
                <Carousel>
                    <CarouselItem class="w-full">
                        <img
                            src="https://picsum.photos/800/400?random=1"
                            class="w-full"
                            alt="Slide 1"
                        />
                    </CarouselItem>
                    <CarouselItem class="w-full">
                        <img
                            src="https://picsum.photos/800/400?random=2"
                            class="w-full"
                            alt="Slide 2"
                        />
                    </CarouselItem>
                    <CarouselItem class="w-full">
                        <img
                            src="https://picsum.photos/800/400?random=3"
                            class="w-full"
                            alt="Slide 3"
                        />
                    </CarouselItem>
                </Carousel>
            </Section>

            <Section title="Direction Control" col=true>
                <Carousel direction=CarouselDirection::Vertical class="h-96 w-full">
                    <CarouselItem class="h-full">
                        <img
                            src="https://picsum.photos/400/400?random=4"
                            alt="Item 1"
                            class="h-full w-full object-cover"
                        />
                    </CarouselItem>
                    <CarouselItem class="h-full">
                        <img
                            src="https://picsum.photos/400/400?random=5"
                            alt="Item 2"
                            class="h-full w-full object-cover"
                        />
                    </CarouselItem>
                    <CarouselItem class="h-full">
                        <img
                            src="https://picsum.photos/400/400?random=6"
                            alt="Item 3"
                            class="h-full w-full object-cover"
                        />
                    </CarouselItem>
                </Carousel>
            </Section>

            <Section title="Alignment Modifiers" col=true>
                <Carousel modifier=CarouselModifier::Start class="w-full">
                    <CarouselItem class="w-1/3">
                        <img
                            src="https://picsum.photos/200/200?random=7"
                            alt="Item 1"
                            class="w-full"
                        />
                    </CarouselItem>
                    <CarouselItem class="w-1/3">
                        <img
                            src="https://picsum.photos/200/200?random=8"
                            alt="Item 2"
                            class="w-full"
                        />
                    </CarouselItem>
                    <CarouselItem class="w-1/3">
                        <img
                            src="https://picsum.photos/200/200?random=9"
                            alt="Item 3"
                            class="w-full"
                        />
                    </CarouselItem>
                    <CarouselItem class="w-1/3">
                        <img
                            src="https://picsum.photos/200/200?random=10"
                            alt="Item 4"
                            class="w-full"
                        />
                    </CarouselItem>
                </Carousel>
            </Section>
        </ContentLayout>
    }
}
