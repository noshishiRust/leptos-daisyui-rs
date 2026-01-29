use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn FilterDemo() -> impl IntoView {
    view! {
        <ContentLayout
            title="Filter"
            description="Filter components use radio buttons to allow users to filter and sort content"
        >
            <Section title="Basic Filter">
                <Filter>
                    <input
                        r#type="radio"
                        name="filter-basic"
                        aria-label="All"
                        class="btn"
                        checked=true
                    />
                    <input r#type="radio" name="filter-basic" aria-label="Electronics" class="btn" />
                    <input r#type="radio" name="filter-basic" aria-label="Clothing" class="btn" />
                    <input r#type="radio" name="filter-basic" aria-label="Books" class="btn" />
                    <FilterReset name="filter-basic" />
                </Filter>
            </Section>

            <Section title="Filter with Colors">
                <Filter>
                    <input
                        r#type="radio"
                        name="filter-colors"
                        aria-label="All"
                        class="btn btn-primary"
                        checked=true
                    />
                    <input
                        r#type="radio"
                        name="filter-colors"
                        aria-label="Premium"
                        class="btn btn-secondary"
                    />
                    <input
                        r#type="radio"
                        name="filter-colors"
                        aria-label="Popular"
                        class="btn btn-accent"
                    />
                    <input
                        r#type="radio"
                        name="filter-colors"
                        aria-label="New"
                        class="btn btn-success"
                    />
                    <FilterReset name="filter-colors" />
                </Filter>
            </Section>

            <Section title="Filter with Different Sizes">
                <Filter>
                    <input
                        r#type="radio"
                        name="filter-sizes"
                        aria-label="All Products"
                        class="btn btn-xs"
                        checked=true
                    />
                    <input
                        r#type="radio"
                        name="filter-sizes"
                        aria-label="Electronics"
                        class="btn btn-sm"
                    />
                    <input
                        r#type="radio"
                        name="filter-sizes"
                        aria-label="Clothing"
                        class="btn btn-md"
                    />
                    <input r#type="radio" name="filter-sizes" aria-label="Books" class="btn btn-lg" />
                    <FilterReset name="filter-sizes" />
                </Filter>
            </Section>

            <Section title="Price Range Filter">
                <Filter>
                    <input
                        r#type="radio"
                        name="filter-price"
                        aria-label="All Prices"
                        class="btn"
                        checked=true
                    />
                    <input r#type="radio" name="filter-price" aria-label="Under $25" class="btn" />
                    <input r#type="radio" name="filter-price" aria-label="$25-$50" class="btn" />
                    <input r#type="radio" name="filter-price" aria-label="$50-$100" class="btn" />
                    <input r#type="radio" name="filter-price" aria-label="Over $100" class="btn" />
                    <FilterReset name="filter-price" />
                </Filter>
            </Section>

            <Section title="Filter with Form Wrapper">
                <p class="text-sm text-base-content/70 mb-4">
                    "Using FilterForm provides better semantic structure"
                </p>
                <FilterForm>
                    <input
                        r#type="radio"
                        name="filter-form"
                        aria-label="Featured"
                        class="btn btn-primary"
                        checked=true
                    />
                    <input
                        r#type="radio"
                        name="filter-form"
                        aria-label="Best Selling"
                        class="btn btn-secondary"
                    />
                    <input
                        r#type="radio"
                        name="filter-form"
                        aria-label="On Sale"
                        class="btn btn-accent"
                    />
                    <input
                        r#type="radio"
                        name="filter-form"
                        aria-label="New Arrivals"
                        class="btn btn-info"
                    />
                    <FilterReset name="filter-form" />
                </FilterForm>
            </Section>

            <Section title="Sort Options">
                <Filter>
                    <input
                        r#type="radio"
                        name="filter-sort"
                        aria-label="Relevance"
                        class="btn"
                        checked=true
                    />
                    <input
                        r#type="radio"
                        name="filter-sort"
                        aria-label="Price: Low to High"
                        class="btn"
                    />
                    <input
                        r#type="radio"
                        name="filter-sort"
                        aria-label="Price: High to Low"
                        class="btn"
                    />
                    <input r#type="radio" name="filter-sort" aria-label="Newest First" class="btn" />
                    <input r#type="radio" name="filter-sort" aria-label="Best Rating" class="btn" />
                    <FilterReset name="filter-sort" />
                </Filter>
            </Section>

            <Section title="Vertical Stack Filter">
                <Filter class="flex-col items-start">
                    <input
                        r#type="radio"
                        name="filter-vertical"
                        aria-label="All Categories"
                        class="btn justify-start w-full"
                        checked=true
                    />
                    <input
                        r#type="radio"
                        name="filter-vertical"
                        aria-label="Technology"
                        class="btn justify-start w-full"
                    />
                    <input
                        r#type="radio"
                        name="filter-vertical"
                        aria-label="Fashion"
                        class="btn justify-start w-full"
                    />
                    <input
                        r#type="radio"
                        name="filter-vertical"
                        aria-label="Home & Garden"
                        class="btn justify-start w-full"
                    />
                    <FilterReset name="filter-vertical" class="w-full" />
                </Filter>
            </Section>

            <Section title="Filter with Custom Styling">
                <Filter>
                    <input
                        r#type="radio"
                        name="filter-custom"
                        aria-label="All"
                        class="btn btn-outline btn-primary"
                        checked=true
                    />
                    <input
                        r#type="radio"
                        name="filter-custom"
                        aria-label="Active"
                        class="btn btn-outline btn-secondary"
                    />
                    <input
                        r#type="radio"
                        name="filter-custom"
                        aria-label="Pending"
                        class="btn btn-outline btn-warning"
                    />
                    <input
                        r#type="radio"
                        name="filter-custom"
                        aria-label="Completed"
                        class="btn btn-outline btn-success"
                    />
                    <FilterReset name="filter-custom" />
                </Filter>
            </Section>
        </ContentLayout>
    }
}
