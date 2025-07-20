use super::style::{ToggleColor, ToggleSize};
use crate::merge_classes;
use leptos::{html::Input, prelude::*};

/// A toggle switch component that provides a visual alternative to traditional checkboxes.
///
/// The Toggle component represents binary state (on/off, enabled/disabled, true/false)
/// with a modern switch-style interface. It maintains checkbox semantics while providing
/// a more intuitive visual representation for users.
///
/// # Props
///
/// * `color` - The color variant of the toggle switch (optional, reactive)
/// * `size` - The size variant of the toggle switch (optional, reactive)
/// * `checked` - Whether the toggle is checked/enabled (optional, reactive)
/// * `disabled` - Whether the toggle is disabled (optional, reactive)
/// * `class` - Additional CSS classes to apply (optional)
/// * `node_ref` - Reference to the underlying HTML input element (optional)
///
/// # CSS Classes
///
/// This component applies the following CSS classes:
/// - Base: `toggle`
/// - Color: Applied from `ToggleColor` enum
/// - Size: Applied from `ToggleSize` enum
/// - Additional: Any classes provided via the `class` prop
///
/// # Example
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::toggle::{Toggle, ToggleColor, ToggleSize};
///
/// #[component]
/// fn SettingsPanel() -> impl IntoView {
///     let (notifications_enabled, set_notifications_enabled) = signal(true);
///     let (dark_mode_enabled, set_dark_mode_enabled) = signal(false);
///
///     view! {
///         <div class="space-y-4">
///             <div class="form-control">
///                 <label class="label cursor-pointer">
///                     <span class="label-text">"Enable Notifications"</span>
///                     <Toggle
///                         color=ToggleColor::Primary
///                         size=ToggleSize::Md
///                         checked=notifications_enabled
///                     />
///                 </label>
///             </div>
///             <div class="form-control">
///                 <label class="label cursor-pointer">
///                     <span class="label-text">"Dark Mode"</span>
///                     <Toggle
///                         color=ToggleColor::Accent
///                         size=ToggleSize::Md
///                         checked=dark_mode_enabled
///                     />
///                 </label>
///             </div>
///         </div>
///     }
/// }
/// ```
///
/// # Advanced Example with Event Handling
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::toggle::{Toggle, ToggleColor};
///
/// #[component]
/// fn FeatureToggle() -> impl IntoView {
///     let (feature_enabled, set_feature_enabled) = signal(false);
///     let (is_loading, set_is_loading) = signal(false);
///
///     let toggle_ref = NodeRef::<Input>::new();
///
///     let handle_toggle = move |_| {
///         if let Some(input) = toggle_ref.get() {
///             let is_checked = input.checked();
///             set_is_loading.set(true);
///             
///             // Simulate API call
///             spawn_local(async move {
///                 TimeoutFuture::new(1000).await;
///                 set_feature_enabled.set(is_checked);
///                 set_is_loading.set(false);
///             });
///         }
///     };
///
///     let color = move || {
///         if is_loading.get() {
///             ToggleColor::Warning
///         } else if feature_enabled.get() {
///             ToggleColor::Success
///         } else {
///             ToggleColor::Default
///         }
///     };
///
///     view! {
///         <div class="form-control">
///             <label class="label cursor-pointer">
///                 <span class="label-text">
///                     {move || if is_loading.get() {
///                         "Updating..."
///                     } else {
///                         "Advanced Features"
///                     }}
///                 </span>
///                 <Toggle
///                     node_ref=toggle_ref
///                     color=move || color()
///                     checked=feature_enabled
///                     disabled=is_loading
///                     on:change=handle_toggle
///                 />
///             </label>
///         </div>
///     }
/// }
/// ```
///
/// # Accessibility
///
/// - Uses semantic `<input type="checkbox">` element with toggle styling
/// - Supports keyboard navigation (Space to toggle, Tab to focus)
/// - Properly announces state changes to screen readers
/// - Compatible with form validation and submission
/// - Supports focus management and visual focus indicators
/// - Works with assistive technologies that understand checkbox semantics
#[component]
pub fn Toggle(
    /// The color variant of the toggle switch
    #[prop(optional, into)]
    color: Signal<ToggleColor>,
    /// The size variant of the toggle switch
    #[prop(optional, into)]
    size: Signal<ToggleSize>,
    /// Whether the toggle is checked/enabled
    #[prop(optional, into)]
    checked: Signal<bool>,
    /// Whether the toggle is disabled
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Additional CSS classes to apply
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Reference to the underlying HTML input element
    #[prop(optional)]
    node_ref: NodeRef<Input>,
) -> impl IntoView {
    view! {
        <input
            node_ref=node_ref
            type="checkbox"
            checked=checked
            disabled=disabled
            class=move || {
                merge_classes!(
                    "toggle",
                    color.get().as_str(),
                    size.get().as_str(),
                    class
                )
            }
        />
    }
}
