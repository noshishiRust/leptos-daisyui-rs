use leptos::prelude::*;

#[component]
pub fn CarouselDemo() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <h1 class="text-3xl font-bold">"Carousel"</h1>
            <p class="text-base-content/70">
                "Carousel shows multiple items, only one item visible at a time"
            </p>

            <div class="space-y-4">
                <h2 class="text-xl font-semibold">"Image Carousel"</h2>
                <div class="carousel w-full">
                    <div id="slide1" class="carousel-item relative w-full">
                        <img
                            src="https://picsum.photos/800/400?random=1"
                            class="w-full"
                            alt="Slide 1"
                        />
                        <div class="absolute flex justify-between transform -translate-y-1/2 left-5 right-5 top-1/2">
                            <a href="#slide4" class="btn btn-circle">
                                "‹"
                            </a>
                            <a href="#slide2" class="btn btn-circle">
                                "›"
                            </a>
                        </div>
                    </div>
                    <div id="slide2" class="carousel-item relative w-full">
                        <img
                            src="https://picsum.photos/800/400?random=2"
                            class="w-full"
                            alt="Slide 2"
                        />
                        <div class="absolute flex justify-between transform -translate-y-1/2 left-5 right-5 top-1/2">
                            <a href="#slide1" class="btn btn-circle">
                                "‹"
                            </a>
                            <a href="#slide3" class="btn btn-circle">
                                "›"
                            </a>
                        </div>
                    </div>
                    <div id="slide3" class="carousel-item relative w-full">
                        <img
                            src="https://picsum.photos/800/400?random=3"
                            class="w-full"
                            alt="Slide 3"
                        />
                        <div class="absolute flex justify-between transform -translate-y-1/2 left-5 right-5 top-1/2">
                            <a href="#slide2" class="btn btn-circle">
                                "‹"
                            </a>
                            <a href="#slide4" class="btn btn-circle">
                                "›"
                            </a>
                        </div>
                    </div>
                    <div id="slide4" class="carousel-item relative w-full">
                        <img
                            src="https://picsum.photos/800/400?random=4"
                            class="w-full"
                            alt="Slide 4"
                        />
                        <div class="absolute flex justify-between transform -translate-y-1/2 left-5 right-5 top-1/2">
                            <a href="#slide3" class="btn btn-circle">
                                "‹"
                            </a>
                            <a href="#slide1" class="btn btn-circle">
                                "›"
                            </a>
                        </div>
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Vertical Carousel"</h2>
                <div class="carousel carousel-vertical rounded-box h-96">
                    <div class="carousel-item h-full">
                        <img src="https://picsum.photos/400/400?random=5" alt="Vertical slide 1" />
                    </div>
                    <div class="carousel-item h-full">
                        <img src="https://picsum.photos/400/400?random=6" alt="Vertical slide 2" />
                    </div>
                    <div class="carousel-item h-full">
                        <img src="https://picsum.photos/400/400?random=7" alt="Vertical slide 3" />
                    </div>
                    <div class="carousel-item h-full">
                        <img src="https://picsum.photos/400/400?random=8" alt="Vertical slide 4" />
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Half Width Carousel"</h2>
                <div class="carousel rounded-box">
                    <div class="carousel-item">
                        <img src="https://picsum.photos/200/200?random=9" alt="Half width 1" />
                    </div>
                    <div class="carousel-item">
                        <img src="https://picsum.photos/200/200?random=10" alt="Half width 2" />
                    </div>
                    <div class="carousel-item">
                        <img src="https://picsum.photos/200/200?random=11" alt="Half width 3" />
                    </div>
                    <div class="carousel-item">
                        <img src="https://picsum.photos/200/200?random=12" alt="Half width 4" />
                    </div>
                    <div class="carousel-item">
                        <img src="https://picsum.photos/200/200?random=13" alt="Half width 5" />
                    </div>
                    <div class="carousel-item">
                        <img src="https://picsum.photos/200/200?random=14" alt="Half width 6" />
                    </div>
                    <div class="carousel-item">
                        <img src="https://picsum.photos/200/200?random=15" alt="Half width 7" />
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Carousel with Indicators"</h2>
                <div class="carousel w-full">
                    <div id="item1" class="carousel-item w-full">
                        <img
                            src="https://picsum.photos/800/300?random=16"
                            class="w-full"
                            alt="Item 1"
                        />
                    </div>
                    <div id="item2" class="carousel-item w-full">
                        <img
                            src="https://picsum.photos/800/300?random=17"
                            class="w-full"
                            alt="Item 2"
                        />
                    </div>
                    <div id="item3" class="carousel-item w-full">
                        <img
                            src="https://picsum.photos/800/300?random=18"
                            class="w-full"
                            alt="Item 3"
                        />
                    </div>
                    <div id="item4" class="carousel-item w-full">
                        <img
                            src="https://picsum.photos/800/300?random=19"
                            class="w-full"
                            alt="Item 4"
                        />
                    </div>
                </div>
                <div class="flex justify-center w-full py-2 gap-2">
                    <a href="#item1" class="btn btn-xs">
                        1
                    </a>
                    <a href="#item2" class="btn btn-xs">
                        2
                    </a>
                    <a href="#item3" class="btn btn-xs">
                        3
                    </a>
                    <a href="#item4" class="btn btn-xs">
                        4
                    </a>
                </div>
            </div>
        </div>
    }
}
