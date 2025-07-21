use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn LoadingDemo() -> impl IntoView {
    view! {
        <ContentLayout
            title="Loading"
            description="Loading indicators show that something is being processed"
        >
            <Section title="Basic Loading">
                <Loading />
                <Loading loading_type=LoadingType::Dots />
                <Loading loading_type=LoadingType::Ring />
                <Loading loading_type=LoadingType::Ball />
                <Loading loading_type=LoadingType::Bars />
                <Loading loading_type=LoadingType::Infinity />
            </Section>

            <Section title="Sizes">
                <Loading size=LoadingSize::Xs />
                <Loading size=LoadingSize::Sm />
                <Loading size=LoadingSize::Md />
                <Loading size=LoadingSize::Lg />
                <Loading size=LoadingSize::Xl />
            </Section>

            <Section title="Colors">
                <Loading color=LoadingColor::Primary />
                <Loading color=LoadingColor::Secondary />
                <Loading color=LoadingColor::Accent />
                <Loading color=LoadingColor::Info />
                <Loading color=LoadingColor::Success />
                <Loading color=LoadingColor::Warning />
                <Loading color=LoadingColor::Error />
            </Section>

        </ContentLayout>
    }
}
