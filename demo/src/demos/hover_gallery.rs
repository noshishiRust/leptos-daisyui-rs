use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn HoverGalleryDemo() -> impl IntoView {
    view! {
        <ContentLayout
            title="Hover Gallery"
            description="Interactive image gallery with horizontal hover reveal"
        >
            <Section title="Basic Gallery (3 Images)">
                <div class="flex justify-center">
                    <HoverGallery class="w-96 h-64">
                        <img
                            src="https://img.daisyui.com/images/stock/photo-1559703248-dcaaec9fab78.webp"
                            alt="Image 1"
                        />
                        <img
                            src="https://img.daisyui.com/images/stock/photo-1565098772267-60af42b81ef2.webp"
                            alt="Image 2"
                        />
                        <img
                            src="https://img.daisyui.com/images/stock/photo-1572635148818-ef6fd45eb394.webp"
                            alt="Image 3"
                        />
                    </HoverGallery>
                </div>
                <div class="text-center text-sm mt-2 opacity-70">
                    "Hover horizontally across the image to reveal more"
                </div>
            </Section>

            <Section title="Multiple Images (5)">
                <div class="flex justify-center">
                    <HoverGallery class="w-full max-w-2xl h-80">
                        <img
                            src="https://img.daisyui.com/images/stock/photo-1559703248-dcaaec9fab78.webp"
                            alt="Shoe 1"
                        />
                        <img
                            src="https://img.daisyui.com/images/stock/photo-1565098772267-60af42b81ef2.webp"
                            alt="Shoe 2"
                        />
                        <img
                            src="https://img.daisyui.com/images/stock/photo-1572635148818-ef6fd45eb394.webp"
                            alt="Shoe 3"
                        />
                        <img
                            src="https://img.daisyui.com/images/stock/photo-1494253109108-2e30c049369b.webp"
                            alt="Shoe 4"
                        />
                        <img
                            src="https://img.daisyui.com/images/stock/photo-1550258987-190a2d41a8ba.webp"
                            alt="Shoe 5"
                        />
                    </HoverGallery>
                </div>
            </Section>

            <Section title="Product Gallery with Card">
                <div class="flex justify-center">
                    <Card class="w-full max-w-2xl bg-base-200 shadow-xl">
                        <figure>
                            <HoverGallery class="w-full h-96">
                                <img
                                    src="https://img.daisyui.com/images/stock/photo-1606107557195-0e29a4b5b4aa.webp"
                                    alt="Product view 1"
                                />
                                <img
                                    src="https://img.daisyui.com/images/stock/photo-1559703248-dcaaec9fab78.webp"
                                    alt="Product view 2"
                                />
                                <img
                                    src="https://img.daisyui.com/images/stock/photo-1565098772267-60af42b81ef2.webp"
                                    alt="Product view 3"
                                />
                                <img
                                    src="https://img.daisyui.com/images/stock/photo-1572635148818-ef6fd45eb394.webp"
                                    alt="Product view 4"
                                />
                            </HoverGallery>
                        </figure>
                        <CardBody>
                            <h2 class="card-title">"Premium Sneakers"</h2>
                            <p>"Hover over the image to see different views"</p>
                            <div class="card-actions justify-end">
                                <Badge color=BadgeColor::Primary>"$129.99"</Badge>
                            </div>
                        </CardBody>
                    </Card>
                </div>
            </Section>

            <Section title="Small Gallery">
                <div class="flex justify-center gap-4 flex-wrap">
                    <HoverGallery class="w-64 h-48 rounded-lg overflow-hidden shadow-lg">
                        <img
                            src="https://img.daisyui.com/images/stock/photo-1559703248-dcaaec9fab78.webp"
                            alt="Gallery 1"
                        />
                        <img
                            src="https://img.daisyui.com/images/stock/photo-1565098772267-60af42b81ef2.webp"
                            alt="Gallery 2"
                        />
                        <img
                            src="https://img.daisyui.com/images/stock/photo-1572635148818-ef6fd45eb394.webp"
                            alt="Gallery 3"
                        />
                    </HoverGallery>

                    <HoverGallery class="w-64 h-48 rounded-lg overflow-hidden shadow-lg">
                        <img
                            src="https://img.daisyui.com/images/stock/photo-1494253109108-2e30c049369b.webp"
                            alt="Gallery 4"
                        />
                        <img
                            src="https://img.daisyui.com/images/stock/photo-1550258987-190a2d41a8ba.webp"
                            alt="Gallery 5"
                        />
                        <img
                            src="https://img.daisyui.com/images/stock/photo-1606107557195-0e29a4b5b4aa.webp"
                            alt="Gallery 6"
                        />
                    </HoverGallery>
                </div>
            </Section>

            <Section title="How It Works">
                <div class="alert alert-info">
                    <div>
                        <strong>"Tip:"</strong>
                        " The component creates invisible columns for each image. "
                        "As you hover horizontally across the gallery, different images are revealed. "
                        "The first image is shown by default. Supports up to 10 images."
                    </div>
                </div>
            </Section>
        </ContentLayout>
    }
}
