# Accordion

A collapsible content component that allows users to expand and collapse sections of information.

## Description

The Accordion component provides an organized way to display large amounts of content in a compact space. Users can expand sections they're interested in while keeping other sections collapsed, making it perfect for FAQs, settings panels, and content organization.

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `open` | `Signal<bool>` | `false` | Whether the accordion item is expanded |
| `arrow` | `Signal<bool>` | `true` | Whether to show expand/collapse arrow |
| `plus_minus` | `Signal<bool>` | `false` | Use plus/minus icons instead of arrow |
| `class` | `&'static str` | `""` | Additional CSS classes |
| `on_toggle` | `Option<Callback>` | - | Callback when accordion is toggled |
| `children` | `Children` | - | Accordion content |

## Subcomponents

### AccordionTitle
The clickable header section that toggles the accordion.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `class` | `&'static str` | `""` | Additional CSS classes |
| `children` | `Children` | - | Title content |

### AccordionContent
The collapsible content area of the accordion.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `class` | `&'static str` | `""` | Additional CSS classes |
| `children` | `Children` | - | Content to show/hide |

## Examples

### Basic Accordion

<details>
<summary>View Code</summary>

```rust
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
fn BasicAccordion() -> impl IntoView {
    let (item1_open, set_item1_open) = signal(false);
    let (item2_open, set_item2_open) = signal(true);
    let (item3_open, set_item3_open) = signal(false);
    
    view! {
        <div class="space-y-2">
            <Accordion 
                class="bg-base-200"
            >
                <AccordionTitle>
                    "What is Leptos?"
                </AccordionTitle>
                <AccordionContent>
                    <p>"Leptos is a full-stack, isomorphic Rust web framework leveraging fine-grained reactivity to build declarative user interfaces."</p>
                </AccordionContent>
            </Accordion>
            
            <Accordion 
                open=Signal::derive(move || item2_open.get())
                on_toggle=Callback::new(move |_| set_item2_open.update(|open| *open = !*open))
                class="bg-base-200"
            >
                <AccordionTitle>
                    "How does it compare to other frameworks?"
                </AccordionTitle>
                <AccordionContent>
                    <p>"Leptos combines the best of React's component model with Rust's performance and safety. It provides:"</p>
                    <ul class="list-disc list-inside mt-2 space-y-1">
                        <li>"Zero-cost fine-grained reactivity"</li>
                        <li>"Server-side rendering and hydration"</li>
                        <li>"Type safety throughout the stack"</li>
                        <li>"Excellent performance characteristics"</li>
                    </ul>
                </AccordionContent>
            </Accordion>
            
            <Accordion 
                open=Signal::derive(move || item3_open.get())
                on_toggle=Callback::new(move |_| set_item3_open.update(|open| *open = !*open))
                class="bg-base-200"
            >
                <AccordionTitle>
                    "Getting Started"
                </AccordionTitle>
                <AccordionContent>
                    <p>"To get started with Leptos, you'll need Rust installed. Then you can create a new project with:"</p>
                    <pre class="bg-base-300 p-2 rounded mt-2">
                        <code>"cargo install leptos-cli\ncargo leptos new my-app"</code>
                    </pre>
                </AccordionContent>
            </Accordion>
        </div>
    }
}
```

</details>

### Plus/Minus Style

<details>
<summary>View Code</summary>

```rust
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
fn PlusMinusAccordion() -> impl IntoView {
    let (section1, set_section1) = signal(false);
    let (section2, set_section2) = signal(false);
    let (section3, set_section3) = signal(false);
    
    view! {
        <div class="space-y-2">
            <Accordion 
                open=Signal::derive(move || section1.get())
                plus_minus=Signal::derive(|| true)
                on_toggle=Callback::new(move |_| set_section1.update(|open| *open = !*open))
                class="bg-base-200"
            >
                <AccordionTitle>
                    "Account Settings"
                </AccordionTitle>
                <AccordionContent>
                    <div class="space-y-4">
                        <div class="form-control">
                            <label class="label">
                                <span class="label-text">"Email"</span>
                            </label>
                            <Input placeholder="your.email@example.com" />
                        </div>
                        <div class="form-control">
                            <label class="label cursor-pointer justify-start space-x-2">
                                <Checkbox />
                                <span class="label-text">"Email notifications"</span>
                            </label>
                        </div>
                    </div>
                </AccordionContent>
            </Accordion>
            
            <Accordion 
                open=Signal::derive(move || section2.get())
                plus_minus=Signal::derive(|| true)
                on_toggle=Callback::new(move |_| set_section2.update(|open| *open = !*open))
                class="bg-base-200"
            >
                <AccordionTitle>
                    "Privacy Settings"
                </AccordionTitle>
                <AccordionContent>
                    <div class="space-y-2">
                        <div class="form-control">
                            <label class="label cursor-pointer justify-start space-x-2">
                                <Checkbox />
                                <span class="label-text">"Make profile public"</span>
                            </label>
                        </div>
                        <div class="form-control">
                            <label class="label cursor-pointer justify-start space-x-2">
                                <Checkbox />
                                <span class="label-text">"Allow data collection"</span>
                            </label>
                        </div>
                        <div class="form-control">
                            <label class="label cursor-pointer justify-start space-x-2">
                                <Checkbox />
                                <span class="label-text">"Third-party integrations"</span>
                            </label>
                        </div>
                    </div>
                </AccordionContent>
            </Accordion>
            
            <Accordion 
                open=Signal::derive(move || section3.get())
                plus_minus=Signal::derive(|| true)
                on_toggle=Callback::new(move |_| set_section3.update(|open| *open = !*open))
                class="bg-base-200"
            >
                <AccordionTitle>
                    "Advanced Options"
                </AccordionTitle>
                <AccordionContent>
                    <p class="text-sm text-gray-600">"These settings are for advanced users only. Proceed with caution."</p>
                    <div class="mt-4 space-y-2">
                        <Button size=Signal::derive(|| ButtonSize::Small) style=Signal::derive(|| ButtonStyle::Ghost)>
                            "Export Data"
                        </Button>
                        <Button size=Signal::derive(|| ButtonSize::Small) color=Signal::derive(|| ButtonColor::Warning)>
                            "Reset Settings"
                        </Button>
                        <Button size=Signal::derive(|| ButtonSize::Small) color=Signal::derive(|| ButtonColor::Error)>
                            "Delete Account"
                        </Button>
                    </div>
                </AccordionContent>
            </Accordion>
        </div>
    }
}
```

</details>

### Accordion Group with Single Open

<details>
<summary>View Code</summary>

```rust
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
fn AccordionGroup() -> impl IntoView {
    let (active_item, set_active_item) = signal(Option::<usize>::None);
    
    let toggle_item = move |index: usize| {
        set_active_item.update(|current| {
            *current = if *current == Some(index) { None } else { Some(index) };
        });
    };
    
    let faq_items = vec![
        (
            "How do I reset my password?",
            "To reset your password, click on the 'Forgot Password' link on the login page. You'll receive an email with instructions to create a new password."
        ),
        (
            "Can I change my subscription plan?",
            "Yes, you can upgrade or downgrade your subscription at any time from your account settings. Changes will be reflected in your next billing cycle."
        ),
        (
            "How do I contact support?",
            "You can reach our support team through the chat widget in the bottom right corner, or by emailing support@example.com. We typically respond within 24 hours."
        ),
        (
            "Is my data secure?",
            "Absolutely. We use industry-standard encryption and security practices to protect your data. All communications are encrypted using TLS 1.3."
        ),
        (
            "Can I export my data?",
            "Yes, you can export all your data at any time from the account settings page. We provide exports in JSON and CSV formats."
        ),
    ];
    
    view! {
        <div class="space-y-1">
            <h2 class="text-2xl font-bold mb-4">"Frequently Asked Questions"</h2>
            {faq_items.into_iter().enumerate().map(|(index, (question, answer))| {
                view! {
                    <Accordion 
                        key=index
                        open=Signal::derive(move || active_item.get() == Some(index))
                        on_toggle=Callback::new(move |_| toggle_item(index))
                        class="bg-base-200"
                    >
                        <AccordionTitle>
                            {question}
                        </AccordionTitle>
                        <AccordionContent>
                            <p>{answer}</p>
                        </AccordionContent>
                    </Accordion>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}
```

</details>

### Nested Accordion

<details>
<summary>View Code</summary>

```rust
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
fn NestedAccordion() -> impl IntoView {
    let (main_section, set_main_section) = signal(false);
    let (subsection1, set_subsection1) = signal(false);
    let (subsection2, set_subsection2) = signal(false);
    
    view! {
        <div class="space-y-2">
            <Accordion 
                open=Signal::derive(move || main_section.get())
                on_toggle=Callback::new(move |_| set_main_section.update(|open| *open = !*open))
                class="bg-base-200"
            >
                <AccordionTitle>
                    "Development Setup"
                </AccordionTitle>
                <AccordionContent>
                    <p class="mb-4">"Complete development environment setup guide."</p>
                    
                    <div class="space-y-1 ml-4">
                        <Accordion 
                            open=Signal::derive(move || subsection1.get())
                            on_toggle=Callback::new(move |_| set_subsection1.update(|open| *open = !*open))
                            class="bg-base-300"
                        >
                            <AccordionTitle class="text-sm">
                                "Prerequisites"
                            </AccordionTitle>
                            <AccordionContent>
                                <ul class="list-disc list-inside space-y-1 text-sm">
                                    <li>"Rust 1.70 or later"</li>
                                    <li>"Node.js 18 or later"</li>
                                    <li>"Git for version control"</li>
                                    <li>"A code editor (VS Code recommended)"</li>
                                </ul>
                            </AccordionContent>
                        </Accordion>
                        
                        <Accordion 
                            open=Signal::derive(move || subsection2.get())
                            on_toggle=Callback::new(move |_| set_subsection2.update(|open| *open = !*open))
                            class="bg-base-300"
                        >
                            <AccordionTitle class="text-sm">
                                "Installation Steps"
                            </AccordionTitle>
                            <AccordionContent>
                                <ol class="list-decimal list-inside space-y-1 text-sm">
                                    <li>"Install Rust from rustup.rs"</li>
                                    <li>"Install leptos-cli: cargo install leptos-cli"</li>
                                    <li>"Clone the repository"</li>
                                    <li>"Run cargo leptos serve"</li>
                                </ol>
                            </AccordionContent>
                        </Accordion>
                    </div>
                </AccordionContent>
            </Accordion>
        </div>
    }
}
```

</details>

### Accordion with Rich Content

<details>
<summary>View Code</summary>

```rust
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
fn RichContentAccordion() -> impl IntoView {
    let (features_open, set_features_open) = signal(false);
    let (pricing_open, set_pricing_open) = signal(false);
    let (examples_open, set_examples_open) = signal(false);
    
    view! {
        <div class="space-y-2">
            <Accordion 
                open=Signal::derive(move || features_open.get())
                on_toggle=Callback::new(move |_| set_features_open.update(|open| *open = !*open))
                class="bg-base-200"
            >
                <AccordionTitle>
                    <div class="flex items-center space-x-2">
                        <span class="text-2xl">"🚀"</span>
                        <span>"Features"</span>
                        <Badge color=Signal::derive(|| BadgeColor::Primary)>"New"</Badge>
                    </div>
                </AccordionTitle>
                <AccordionContent>
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <Card class="bg-base-100">
                            <CardBody>
                                <CardTitle>"Performance"</CardTitle>
                                <p class="text-sm">"Lightning-fast reactive updates with fine-grained reactivity."</p>
                            </CardBody>
                        </Card>
                        <Card class="bg-base-100">
                            <CardBody>
                                <CardTitle>"Type Safety"</CardTitle>
                                <p class="text-sm">"Full type safety from frontend to backend with Rust."</p>
                            </CardBody>
                        </Card>
                    </div>
                </AccordionContent>
            </Accordion>
            
            <Accordion 
                open=Signal::derive(move || pricing_open.get())
                on_toggle=Callback::new(move |_| set_pricing_open.update(|open| *open = !*open))
                class="bg-base-200"
            >
                <AccordionTitle>
                    <div class="flex items-center space-x-2">
                        <span class="text-2xl">"💰"</span>
                        <span>"Pricing"</span>
                    </div>
                </AccordionTitle>
                <AccordionContent>
                    <div class="overflow-x-auto">
                        <table class="table table-zebra w-full">
                            <thead>
                                <tr>
                                    <th>"Plan"</th>
                                    <th>"Price"</th>
                                    <th>"Features"</th>
                                    <th>"Action"</th>
                                </tr>
                            </thead>
                            <tbody>
                                <tr>
                                    <td>"Free"</td>
                                    <td>"$0/month"</td>
                                    <td>"Basic features"</td>
                                    <td>
                                        <Button size=Signal::derive(|| ButtonSize::Small)>
                                            "Get Started"
                                        </Button>
                                    </td>
                                </tr>
                                <tr>
                                    <td>"Pro"</td>
                                    <td>"$10/month"</td>
                                    <td>"Advanced features"</td>
                                    <td>
                                        <Button 
                                            size=Signal::derive(|| ButtonSize::Small)
                                            style=Signal::derive(|| ButtonStyle::Primary)
                                        >
                                            "Upgrade"
                                        </Button>
                                    </td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                </AccordionContent>
            </Accordion>
            
            <Accordion 
                open=Signal::derive(move || examples_open.get())
                on_toggle=Callback::new(move |_| set_examples_open.update(|open| *open = !*open))
                class="bg-base-200"
            >
                <AccordionTitle>
                    <div class="flex items-center space-x-2">
                        <span class="text-2xl">"📝"</span>
                        <span>"Code Examples"</span>
                    </div>
                </AccordionTitle>
                <AccordionContent>
                    <div class="space-y-4">
                        <div>
                            <h4 class="font-semibold mb-2">"Basic Component"</h4>
                            <pre class="bg-base-300 p-4 rounded-lg overflow-x-auto">
                                <code>{r#"#[component]
fn HelloWorld() -> impl IntoView {
    view! {
        <div>"Hello, World!"</div>
    }
}"#}</code>
                            </pre>
                        </div>
                        <div>
                            <h4 class="font-semibold mb-2">"With State"</h4>
                            <pre class="bg-base-300 p-4 rounded-lg overflow-x-auto">
                                <code>{r#"#[component]
fn Counter() -> impl IntoView {
    let (count, set_count) = signal(0);
    
    view! {
        <div>
            <p>"Count: " {count}</p>
            <button on:click=move |_| set_count.update(|n| *n += 1)>
                "Increment"
            </button>
        </div>
    }
}"#}</code>
                            </pre>
                        </div>
                    </div>
                </AccordionContent>
            </Accordion>
        </div>
    }
}
```

</details>

## Accessibility

- Proper ARIA attributes for expandable content
- Keyboard navigation (Enter/Space to toggle)
- Focus management for accordion headers
- Screen reader announcements for state changes
- Semantic HTML structure

## Best Practices

1. Use clear, descriptive titles for accordion sections
2. Consider the initial state - what should be open by default?
3. Provide visual indicators for expand/collapse state
4. Keep content organized and scannable
5. Avoid nesting too deeply (max 2-3 levels)
6. Use consistent spacing and styling
7. Consider mobile experience for touch interactions
8. Group related content logically
