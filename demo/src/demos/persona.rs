use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn PersonaDemo() -> impl IntoView {
    view! {
        <ContentLayout
            title="Persona"
            description="Display user profiles with avatars, names, and additional information"
        >
            <Section title="Basic Persona">
                <div class="space-y-4">
                    <Persona
                        name="John Doe".to_string()
                        secondary_text="Software Engineer".to_string()
                        image_url="https://picsum.photos/96/96?random=1".to_string()
                        initials="".to_string()
                        size=PersonaSize::Medium
                    />
                    <Persona
                        name="Jane Smith".to_string()
                        secondary_text="Product Manager".to_string()
                        image_url="https://picsum.photos/96/96?random=2".to_string()
                        initials="".to_string()
                        size=PersonaSize::Medium
                    />
                </div>
            </Section>

            <Section title="Persona Sizes">
                <div class="space-y-4">
                    <Persona
                        name="Extra Small".to_string()
                        secondary_text="XS Size".to_string()
                        image_url="https://picsum.photos/96/96?random=4".to_string()
                        initials="".to_string()
                        size=PersonaSize::XSmall
                    />
                    <Persona
                        name="Small Size".to_string()
                        secondary_text="SM Size".to_string()
                        image_url="https://picsum.photos/96/96?random=5".to_string()
                        initials="".to_string()
                        size=PersonaSize::Small
                    />
                    <Persona
                        name="Medium Size".to_string()
                        secondary_text="MD Size".to_string()
                        image_url="https://picsum.photos/96/96?random=6".to_string()
                        initials="".to_string()
                        size=PersonaSize::Medium
                    />
                    <Persona
                        name="Large Size".to_string()
                        secondary_text="LG Size".to_string()
                        image_url="https://picsum.photos/96/96?random=7".to_string()
                        initials="".to_string()
                        size=PersonaSize::Large
                    />
                </div>
            </Section>

            <Section title="Persona with Initials (No Image)">
                <div class="space-y-4">
                    <Persona
                        name="Alice Cooper".to_string()
                        secondary_text="Marketing Director".to_string()
                        image_url="".to_string()
                        initials="".to_string()
                        size=PersonaSize::Medium
                    />
                    <Persona
                        name="David Miller".to_string()
                        secondary_text="CEO".to_string()
                        image_url="".to_string()
                        initials="".to_string()
                        size=PersonaSize::Medium
                    />
                </div>
            </Section>
        </ContentLayout>
    }
}
