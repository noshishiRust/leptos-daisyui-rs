use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn IndicatorDemo() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <h1 class="text-3xl font-bold">"Indicator"</h1>
            <p class="text-base-content/70">
                "Indicator is used to place an element on the corner of another element"
            </p>

            <div class="space-y-4">
                <h2 class="text-xl font-semibold">"Basic Indicator"</h2>
                <div class="flex flex-wrap gap-8">
                    <Indicator>
                        <IndicatorItem class="badge badge-secondary">"99+"</IndicatorItem>
                        <div class="grid w-32 h-32 bg-base-300 place-items-center">"content"</div>
                    </Indicator>
                </div>

                <h2 class="text-xl font-semibold">"Positions"</h2>
                <div class="flex flex-wrap gap-8">
                    <Indicator>
                        <IndicatorItem
                            vertical=IndicatorVerticalPlacement::Top
                            horizontal=IndicatorHorizontalPlacement::Start
                        >
                            <Badge color=BadgeColor::Secondary>"Top Start"</Badge>
                        </IndicatorItem>
                        <div class="grid w-32 h-32 bg-base-300 place-items-center">"Top Start"</div>
                    </Indicator>
                    <Indicator>
                        <IndicatorItem
                            vertical=IndicatorVerticalPlacement::Top
                            horizontal=IndicatorHorizontalPlacement::Center
                        >
                            <Badge color=BadgeColor::Secondary>"Top Center"</Badge>
                        </IndicatorItem>
                        <div class="grid w-32 h-32 bg-base-300 place-items-center">
                            "Top Center"
                        </div>
                    </Indicator>
                    <Indicator>
                        <IndicatorItem
                            vertical=IndicatorVerticalPlacement::Top
                            horizontal=IndicatorHorizontalPlacement::End
                        >
                            <Badge color=BadgeColor::Secondary>"Top End"</Badge>
                        </IndicatorItem>
                        <div class="grid w-32 h-32 bg-base-300 place-items-center">"Top End"</div>
                    </Indicator>
                    <Indicator>
                        <IndicatorItem
                            vertical=IndicatorVerticalPlacement::Middle
                            horizontal=IndicatorHorizontalPlacement::Start
                        >
                            <Badge color=BadgeColor::Secondary>"Middle Start"</Badge>
                        </IndicatorItem>
                        <div class="grid w-32 h-32 bg-base-300 place-items-center">
                            "Middle Start"
                        </div>
                    </Indicator>
                    <Indicator>
                        <IndicatorItem
                            vertical=IndicatorVerticalPlacement::Middle
                            horizontal=IndicatorHorizontalPlacement::Center
                        >
                            <Badge color=BadgeColor::Secondary>"Middle Center"</Badge>
                        </IndicatorItem>
                        <div class="grid w-32 h-32 bg-base-300 place-items-center">
                            "Middle Center"
                        </div>
                    </Indicator>
                    <Indicator>
                        <IndicatorItem
                            vertical=IndicatorVerticalPlacement::Middle
                            horizontal=IndicatorHorizontalPlacement::End
                        >
                            <Badge color=BadgeColor::Secondary>"Middle End"</Badge>
                        </IndicatorItem>
                        <div class="grid w-32 h-32 bg-base-300 place-items-center">
                            "Middle End"
                        </div>
                    </Indicator>
                    <Indicator>
                        <IndicatorItem
                            vertical=IndicatorVerticalPlacement::Bottom
                            horizontal=IndicatorHorizontalPlacement::Start
                        >
                            <Badge color=BadgeColor::Secondary>"Bottom Start"</Badge>
                        </IndicatorItem>
                        <div class="grid w-32 h-32 bg-base-300 place-items-center">
                            "Bottom Start"
                        </div>
                    </Indicator>
                    <Indicator>
                        <IndicatorItem
                            vertical=IndicatorVerticalPlacement::Bottom
                            horizontal=IndicatorHorizontalPlacement::Center
                        >
                            <Badge color=BadgeColor::Secondary>"Bottom Center"</Badge>
                        </IndicatorItem>
                        <div class="grid w-32 h-32 bg-base-300 place-items-center">
                            "Bottom Center"
                        </div>
                    </Indicator>
                    <Indicator>
                        <IndicatorItem
                            vertical=IndicatorVerticalPlacement::Bottom
                            horizontal=IndicatorHorizontalPlacement::End
                        >
                            <Badge color=BadgeColor::Secondary>"Bottom End"</Badge>
                        </IndicatorItem>
                        <div class="grid w-32 h-32 bg-base-300 place-items-center">
                            "Bottom End"
                        </div>
                    </Indicator>
                </div>
            </div>
        </div>
    }
}
