use super::style::{CarouselDirection, CarouselModifier};
use crate::merge_classes;
use leptos::{html::Div, prelude::*};
use wasm_bindgen::JsCast;
use web_sys;

/// # Carousel Component
///
/// A scrollable container for displaying images or content in a horizontal or vertical layout.
/// Supports various alignment and direction modifiers.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("carousel carousel-item carousel-start carousel-center carousel-end carousel-horizontal carousel-vertical");
/// ```
///
/// ## Node References
/// - `node_ref` - References the top `<div>` element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn Carousel(
    /// Visual modifier for carousel alignment
    #[prop(optional, into)]
    modifier: Signal<CarouselModifier>,

    /// Direction of carousel scroll (horizontal or vertical)
    #[prop(optional, into)]
    direction: Signal<CarouselDirection>,

    /// Additional CSS classes to apply to the carousel container
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the carousel container element
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Child [`CarouselItem`] components
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "carousel",
                    modifier.get().as_str(),
                    direction.get().as_str(),
                    class
                )
            }
        >
            {children()}
        </div>
    }
}

/// # Carousel Item Component
///
/// An individual item within a carousel. Use Tailwind CSS classes like `w-full`
/// to control item dimensions and appearance.
///
/// ## Node References
/// - `node_ref` - References the top `<div>` element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn CarouselItem(
    /// Additional CSS classes to apply to the carousel item
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the carousel container element
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Content for this carousel item
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("carousel-item", class)>
            {children()}
        </div>
    }
}

/// # Carousel Indicators Component
///
/// Displays navigation dots below the carousel showing the active slide.
/// Each dot can be clicked to navigate to that slide.
///
/// ## Usage
/// ```rust,ignore
/// let (active_index, set_active_index) = signal(0);
/// let carousel_ref = NodeRef::<Div>::new();
///
/// view! {
///     <Carousel node_ref=carousel_ref>
///         <CarouselItem id="slide1" class="w-full">...</CarouselItem>
///         <CarouselItem id="slide2" class="w-full">...</CarouselItem>
///         <CarouselItem id="slide3" class="w-full">...</CarouselItem>
///     </Carousel>
///
///     <CarouselIndicators
///         count=3
///         active=active_index
///         carousel_ref=carousel_ref
///         on_click=move |index| {
///             set_active_index.set(index);
///             scroll_to_carousel_item(carousel_ref, index);
///         }
///     />
/// }
/// ```
#[component]
pub fn CarouselIndicators(
    /// Total number of slides
    #[prop(into)]
    count: Signal<usize>,

    /// Current active slide index (0-based)
    #[prop(into)]
    active: Signal<usize>,

    /// Callback when an indicator is clicked with the slide index
    #[prop(into)]
    on_click: Callback<usize>,

    /// Additional CSS classes for the indicators container
    #[prop(optional, into)]
    class: &'static str,
) -> impl IntoView {
    view! {
        <div class=move || merge_classes!("flex justify-center gap-2 py-2", class)>
            {move || {
                (0..count.get())
                    .map(|index| {
                        let is_active = move || active.get() == index;
                        view! {
                            <button
                                class="btn btn-xs"
                                class:btn-active=is_active
                                on:click=move |_| on_click.run(index)
                            >
                                {index + 1}
                            </button>
                        }
                    })
                    .collect_view()
            }}
        </div>
    }
}

/// # Carousel Navigation Buttons Component
///
/// Displays previous/next arrow buttons for carousel navigation.
///
/// ## Usage
/// ```rust,ignore
/// let (active_index, set_active_index) = signal(0);
/// let carousel_ref = NodeRef::<Div>::new();
/// let slide_count = 3;
///
/// view! {
///     <div class="relative">
///         <Carousel node_ref=carousel_ref>
///             <CarouselItem class="w-full">...</CarouselItem>
///             <CarouselItem class="w-full">...</CarouselItem>
///             <CarouselItem class="w-full">...</CarouselItem>
///         </Carousel>
///
///         <CarouselNavButtons
///             carousel_ref=carousel_ref
///             active=active_index
///             count=slide_count
///             on_prev=move || {
///                 let new_index = if active_index.get() == 0 {
///                     slide_count - 1
///                 } else {
///                     active_index.get() - 1
///                 };
///                 set_active_index.set(new_index);
///                 scroll_to_carousel_item(carousel_ref, new_index);
///             }
///             on_next=move || {
///                 let new_index = (active_index.get() + 1) % slide_count;
///                 set_active_index.set(new_index);
///                 scroll_to_carousel_item(carousel_ref, new_index);
///             }
///         />
///     </div>
/// }
/// ```
#[component]
pub fn CarouselNavButtons(
    /// Current active slide index
    #[prop(into)]
    active: Signal<usize>,

    /// Total number of slides (used for wrapping navigation)
    #[prop(into)]
    count: Signal<usize>,

    /// Callback when previous button is clicked
    #[prop(into)]
    on_prev: Callback<()>,

    /// Callback when next button is clicked
    #[prop(into)]
    on_next: Callback<()>,

    /// Additional CSS classes for the button container
    #[prop(optional, into)]
    class: &'static str,

    /// Whether to disable buttons at the start/end or wrap around
    #[prop(optional, into)]
    wrap: Signal<bool>,
) -> impl IntoView {
    view! {
        <div class=move || {
            merge_classes!(
                "absolute flex justify-between transform -translate-y-1/2 left-5 right-5 top-1/2",
                class
            )
        }>
            <button
                class="btn btn-circle"
                on:click=move |_| on_prev.run(())
                disabled=move || !wrap.get() && active.get() == 0
            >
                "❮"
            </button>
            <button
                class="btn btn-circle"
                on:click=move |_| on_next.run(())
                disabled=move || !wrap.get() && (active.get() >= count.get() - 1)
            >
                "❯"
            </button>
        </div>
    }
}

/// Helper function to scroll to a specific carousel item by index.
///
/// ## Usage
/// ```rust,ignore
/// let carousel_ref = NodeRef::<Div>::new();
///
/// // Scroll to the second slide (index 1)
/// scroll_to_carousel_item(carousel_ref, 1);
/// ```
pub fn scroll_to_carousel_item(carousel_ref: NodeRef<Div>, index: usize) {
    if let Some(carousel) = carousel_ref.get_untracked() {
        let carousel_elem = carousel.unchecked_ref::<web_sys::Element>();

        // Get all carousel items
        if let Ok(items) = carousel_elem.query_selector_all(".carousel-item")
            && let Some(item) = items.item(index as u32)
            && let Some(element) = item.dyn_ref::<web_sys::Element>()
        {
            // Use scrollIntoView with inline option to prevent page scroll
            // This uses the simpler API that's always available
            element.scroll_into_view_with_bool(true);
        }
    }
}
