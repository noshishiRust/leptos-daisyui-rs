use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn CarouselDemo() -> impl IntoView {
    // State for carousel with indicators
    let (active_index, set_active_index) = signal(0);
    let carousel_ref = NodeRef::<leptos::html::Div>::new();
    let slide_count = 3;

    // State for carousel with navigation buttons
    let (nav_active_index, set_nav_active_index) = signal(0);
    let nav_carousel_ref = NodeRef::<leptos::html::Div>::new();
    let nav_slide_count = 4;

    view! {
        <ContentLayout
            title="Carousel"
            description="Carousel shows multiple items, only one item visible at a time"
        >
            <Section title="Basic Usage (Scroll to Navigate)" col=true>
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
                <div class="text-sm text-center mt-2 opacity-70">
                    "Scroll or swipe horizontally to see more slides"
                </div>
            </Section>

            <Section title="With Indicator Dots" col=true>
                <Carousel node_ref=carousel_ref>
                    <CarouselItem class="w-full">
                        <img
                            src="https://picsum.photos/800/400?random=4"
                            class="w-full"
                            alt="Slide 1"
                        />
                    </CarouselItem>
                    <CarouselItem class="w-full">
                        <img
                            src="https://picsum.photos/800/400?random=5"
                            class="w-full"
                            alt="Slide 2"
                        />
                    </CarouselItem>
                    <CarouselItem class="w-full">
                        <img
                            src="https://picsum.photos/800/400?random=6"
                            class="w-full"
                            alt="Slide 3"
                        />
                    </CarouselItem>
                </Carousel>

                <CarouselIndicators
                    count=slide_count
                    active=active_index
                    on_click=Callback::new(move |index| {
                        set_active_index.set(index);
                        scroll_to_carousel_item(carousel_ref, index);
                    })
                />
            </Section>

            <Section title="With Navigation Buttons" col=true>
                <div class="relative">
                    <Carousel node_ref=nav_carousel_ref>
                        <CarouselItem class="w-full">
                            <img
                                src="https://picsum.photos/800/400?random=7"
                                class="w-full"
                                alt="Slide 1"
                            />
                        </CarouselItem>
                        <CarouselItem class="w-full">
                            <img
                                src="https://picsum.photos/800/400?random=8"
                                class="w-full"
                                alt="Slide 2"
                            />
                        </CarouselItem>
                        <CarouselItem class="w-full">
                            <img
                                src="https://picsum.photos/800/400?random=9"
                                class="w-full"
                                alt="Slide 3"
                            />
                        </CarouselItem>
                        <CarouselItem class="w-full">
                            <img
                                src="https://picsum.photos/800/400?random=10"
                                class="w-full"
                                alt="Slide 4"
                            />
                        </CarouselItem>
                    </Carousel>

                    <CarouselNavButtons
                        active=nav_active_index
                        count=nav_slide_count
                        wrap=false
                        on_prev=Callback::new(move |_| {
                            if nav_active_index.get() > 0 {
                                let new_index = nav_active_index.get() - 1;
                                set_nav_active_index.set(new_index);
                                scroll_to_carousel_item(nav_carousel_ref, new_index);
                            }
                        })
                        on_next=Callback::new(move |_| {
                            if nav_active_index.get() < nav_slide_count - 1 {
                                let new_index = nav_active_index.get() + 1;
                                set_nav_active_index.set(new_index);
                                scroll_to_carousel_item(nav_carousel_ref, new_index);
                            }
                        })
                    />
                </div>

                <CarouselIndicators
                    count=nav_slide_count
                    active=nav_active_index
                    on_click=Callback::new(move |index| {
                        set_nav_active_index.set(index);
                        scroll_to_carousel_item(nav_carousel_ref, index);
                    })
                />
            </Section>

            <Section title="Direction Control" col=true>
                <Carousel direction=CarouselDirection::Vertical class="h-96 w-full">
                    <CarouselItem class="h-full">
                        <img
                            src="https://picsum.photos/400/400?random=11"
                            alt="Item 1"
                            class="h-full w-full object-cover"
                        />
                    </CarouselItem>
                    <CarouselItem class="h-full">
                        <img
                            src="https://picsum.photos/400/400?random=12"
                            alt="Item 2"
                            class="h-full w-full object-cover"
                        />
                    </CarouselItem>
                    <CarouselItem class="h-full">
                        <img
                            src="https://picsum.photos/400/400?random=13"
                            alt="Item 3"
                            class="h-full w-full object-cover"
                        />
                    </CarouselItem>
                </Carousel>
                <div class="text-sm text-center mt-2 opacity-70">
                    "Vertical carousel - scroll vertically to navigate"
                </div>
            </Section>

            <Section title="Alignment Modifiers" col=true>
                <Carousel modifier=CarouselModifier::Start class="w-full">
                    <CarouselItem class="w-1/3">
                        <img
                            src="https://picsum.photos/200/200?random=14"
                            alt="Item 1"
                            class="w-full"
                        />
                    </CarouselItem>
                    <CarouselItem class="w-1/3">
                        <img
                            src="https://picsum.photos/200/200?random=15"
                            alt="Item 2"
                            class="w-full"
                        />
                    </CarouselItem>
                    <CarouselItem class="w-1/3">
                        <img
                            src="https://picsum.photos/200/200?random=16"
                            alt="Item 3"
                            class="w-full"
                        />
                    </CarouselItem>
                    <CarouselItem class="w-1/3">
                        <img
                            src="https://picsum.photos/200/200?random=17"
                            alt="Item 4"
                            class="w-full"
                        />
                    </CarouselItem>
                </Carousel>
                <div class="text-sm text-center mt-2 opacity-70">
                    "Multiple items visible - start alignment"
                </div>
            </Section>
        </ContentLayout>
    }
}
