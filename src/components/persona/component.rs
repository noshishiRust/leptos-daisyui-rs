use super::style::PersonaSize;
use leptos::{html, prelude::*};

/// A persona component for displaying user profiles with avatar and information.
///
/// # Example
/// ```rust,ignore
/// use leptos::prelude::*;
/// use leptos_daisyui_rs::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <Persona
///             name="John Doe"
///             secondary_text="Software Engineer"
///             image_url="/avatar.jpg"
///             size=PersonaSize::Medium
///         />
///     }
/// }
/// ```
///
/// # CSS Classes
/// Add to your `input.css`:
/// ```css
/// @source inline("flex items-center gap-2 gap-3 gap-4");
/// @source inline("w-8 w-10 w-12 w-16 w-24 rounded-full");
/// @source inline("text-xs text-sm text-base text-lg text-xl");
/// @source inline("font-semibold text-base-content opacity-60");
/// @source inline("avatar placeholder");
/// ```
#[component]
pub fn Persona(
    /// Primary name/title
    #[prop(into)]
    name: Signal<String>,
    /// Secondary text (e.g., role, email)
    #[prop(optional, into)]
    secondary_text: Signal<String>,
    /// Image URL for avatar
    #[prop(optional, into)]
    image_url: Signal<String>,
    /// Initials to show when no image (derived from name if not provided)
    #[prop(optional, into)]
    initials: Signal<String>,
    /// Size of the persona
    #[prop(optional, into)]
    size: Signal<PersonaSize>,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Reference to the underlying DOM node
    #[prop(optional)]
    node_ref: NodeRef<html::Div>,
) -> impl IntoView {
    let computed_initials = move || {
        let init = initials.get();
        if !init.is_empty() {
            init
        } else {
            // Generate initials from name
            let name_val = name.get();
            name_val
                .split_whitespace()
                .take(2)
                .filter_map(|word| word.chars().next())
                .collect::<String>()
                .to_uppercase()
        }
    };

    let gap_class = move || match size.get() {
        PersonaSize::XSmall | PersonaSize::Small => "gap-2",
        PersonaSize::Medium => "gap-3",
        PersonaSize::Large | PersonaSize::XLarge => "gap-4",
    };

    let container_class = move || {
        let mut classes = vec!["flex", "items-center", gap_class()];
        if !class.is_empty() {
            classes.push(class);
        }
        classes.join(" ")
    };

    view! {
        <div
            node_ref=node_ref
            class=container_class
        >
            <div class="avatar placeholder">
                <Show
                    when=move || !image_url.get().is_empty()
                    fallback=move || view! {
                        <div class=move || {
                            format!("{} rounded-full bg-neutral text-neutral-content",
                                size.get().avatar_class())
                        }>
                            <span class=move || size.get().name_class()>
                                {computed_initials()}
                            </span>
                        </div>
                    }
                >
                    <div class=move || format!("{} rounded-full", size.get().avatar_class())>
                        <img src=move || image_url.get() alt=move || name.get() />
                    </div>
                </Show>
            </div>
            <div class="flex flex-col">
                <span class=move || format!("font-semibold {}", size.get().name_class())>
                    {move || name.get()}
                </span>
                <Show when=move || !secondary_text.get().is_empty()>
                    <span class=move || format!("text-base-content opacity-60 {}", size.get().secondary_class())>
                        {move || secondary_text.get()}
                    </span>
                </Show>
            </div>
        </div>
    }
}
