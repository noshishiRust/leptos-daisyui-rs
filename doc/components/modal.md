# Modal

A flexible modal dialog component for displaying content in an overlay with backdrop and customizable actions.

## Description

The Modal component creates an overlay dialog that can display forms, confirmations, images, or any content that needs to capture user attention. It supports various sizes, positioning, and interaction patterns.

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `open` | `Signal<bool>` | `false` | Whether the modal is open |
| `backdrop_close` | `Signal<bool>` | `true` | Whether clicking backdrop closes modal |
| `responsive` | `Signal<bool>` | `false` | Whether modal is responsive on mobile |
| `class` | `&'static str` | `""` | Additional CSS classes |
| `on_close` | `Option<Callback>` | - | Callback when modal should close |
| `children` | `Children` | - | Modal content |

## Subcomponents

### ModalBox
The main content container of the modal with proper spacing and layout.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `class` | `&'static str` | `""` | Additional CSS classes |
| `children` | `Children` | - | Modal content |

### ModalAction
Action area for buttons and interactive elements, typically at the bottom.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `class` | `&'static str` | `""` | Additional CSS classes |
| `children` | `Children` | - | Action elements |

## Examples

### Basic Modal

<details>
<summary>View Code</summary>

```rust
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
fn BasicModal() -> impl IntoView {
    let (modal_open, set_modal_open) = signal(false);
    
    view! {
        <div>
            <Button 
                style=Signal::derive(|| ButtonStyle::Primary)
                on:click=move |_| set_modal_open.set(true)
            >
                "Open Modal"
            </Button>
            
            <Modal 
                open=Signal::derive(move || modal_open.get())
                on_close=Callback::new(move |_| set_modal_open.set(false))
            >
                <ModalBox class="w-11/12 max-w-5xl">
                    <h3 class="font-bold text-lg">"Hello World!"</h3>
                    <p class="py-4">"This is a basic modal dialog. You can put any content here."</p>
                    <ModalAction>
                        <Button 
                            style=Signal::derive(|| ButtonStyle::Primary)
                            on:click=move |_| set_modal_open.set(false)
                        >
                            "Close"
                        </Button>
                    </ModalAction>
                </ModalBox>
            </Modal>
        </div>
    }
}
```

</details>

### Confirmation Modal

<details>
<summary>View Code</summary>

```rust
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
fn ConfirmationModal() -> impl IntoView {
    let (modal_open, set_modal_open) = signal(false);
    let (confirmed, set_confirmed) = signal(false);
    
    let handle_confirm = move |_| {
        set_confirmed.set(true);
        set_modal_open.set(false);
    };
    
    let handle_cancel = move |_| {
        set_modal_open.set(false);
    };
    
    view! {
        <div>
            <Button 
                color=Signal::derive(|| ButtonColor::Error)
                on:click=move |_| set_modal_open.set(true)
            >
                "Delete Item"
            </Button>
            
            {move || if confirmed.get() {
                view! {
                    <div class="alert alert-success mt-4">
                        <span>"Item deleted successfully!"</span>
                    </div>
                }.into_any()
            } else {
                view! { <div></div> }.into_any()
            }}
            
            <Modal 
                open=Signal::derive(move || modal_open.get())
                on_close=Callback::new(move |_| set_modal_open.set(false))
            >
                <ModalBox>
                    <h3 class="font-bold text-lg">"Confirm Deletion"</h3>
                    <p class="py-4">"Are you sure you want to delete this item? This action cannot be undone."</p>
                    <ModalAction class="justify-end space-x-2">
                        <Button 
                            style=Signal::derive(|| ButtonStyle::Ghost)
                            on:click=handle_cancel
                        >
                            "Cancel"
                        </Button>
                        <Button 
                            color=Signal::derive(|| ButtonColor::Error)
                            on:click=handle_confirm
                        >
                            "Delete"
                        </Button>
                    </ModalAction>
                </ModalBox>
            </Modal>
        </div>
    }
}
```

</details>

### Form Modal

<details>
<summary>View Code</summary>

```rust
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
fn FormModal() -> impl IntoView {
    let (modal_open, set_modal_open) = signal(false);
    let (name, set_name) = signal("".to_string());
    let (email, set_email) = signal("".to_string());
    let (submitted, set_submitted) = signal(false);
    
    let handle_submit = move |_| {
        // Simulate form submission
        set_submitted.set(true);
        set_modal_open.set(false);
        // Reset form
        set_name.set("".to_string());
        set_email.set("".to_string());
    };
    
    view! {
        <div>
            <Button 
                style=Signal::derive(|| ButtonStyle::Primary)
                on:click=move |_| set_modal_open.set(true)
            >
                "Add User"
            </Button>
            
            {move || if submitted.get() {
                view! {
                    <div class="alert alert-success mt-4">
                        <span>"User added successfully!"</span>
                    </div>
                }.into_any()
            } else {
                view! { <div></div> }.into_any()
            }}
            
            <Modal 
                open=Signal::derive(move || modal_open.get())
                on_close=Callback::new(move |_| set_modal_open.set(false))
            >
                <ModalBox>
                    <h3 class="font-bold text-lg">"Add New User"</h3>
                    <div class="form-control w-full space-y-4 mt-4">
                        <div>
                            <label class="label">
                                <span class="label-text">"Name"</span>
                            </label>
                            <Input 
                                placeholder="Enter name"
                                value=Signal::derive(move || name.get())
                                on:input=move |ev| set_name.set(event_target_value(&ev))
                            />
                        </div>
                        <div>
                            <label class="label">
                                <span class="label-text">"Email"</span>
                            </label>
                            <Input 
                                placeholder="Enter email"
                                input_type=Signal::derive(|| InputType::Email)
                                value=Signal::derive(move || email.get())
                                on:input=move |ev| set_email.set(event_target_value(&ev))
                            />
                        </div>
                    </div>
                    <ModalAction class="justify-end space-x-2 mt-6">
                        <Button 
                            style=Signal::derive(|| ButtonStyle::Ghost)
                            on:click=move |_| set_modal_open.set(false)
                        >
                            "Cancel"
                        </Button>
                        <Button 
                            style=Signal::derive(|| ButtonStyle::Primary)
                            disabled=Signal::derive(move || name.get().is_empty() || email.get().is_empty())
                            on:click=handle_submit
                        >
                            "Add User"
                        </Button>
                    </ModalAction>
                </ModalBox>
            </Modal>
        </div>
    }
}
```

</details>

### Image Modal

<details>
<summary>View Code</summary>

```rust
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
fn ImageModal() -> impl IntoView {
    let (modal_open, set_modal_open) = signal(false);
    
    view! {
        <div>
            <div class="cursor-pointer" on:click=move |_| set_modal_open.set(true)>
                <img 
                    src="https://via.placeholder.com/300x200" 
                    alt="Click to enlarge"
                    class="rounded-lg shadow-lg hover:shadow-xl transition-shadow"
                />
                <p class="text-sm text-gray-600 mt-2">"Click to enlarge"</p>
            </div>
            
            <Modal 
                open=Signal::derive(move || modal_open.get())
                on_close=Callback::new(move |_| set_modal_open.set(false))
            >
                <ModalBox class="w-11/12 max-w-5xl">
                    <div class="relative">
                        <button 
                            class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2"
                            on:click=move |_| set_modal_open.set(false)
                        >
                            "✕"
                        </button>
                        <img 
                            src="https://via.placeholder.com/800x600" 
                            alt="Enlarged view"
                            class="w-full h-auto rounded-lg"
                        />
                    </div>
                    <div class="py-4">
                        <h3 class="font-bold text-lg">"Image Title"</h3>
                        <p>"This is a detailed view of the image with additional information."</p>
                    </div>
                </ModalBox>
            </Modal>
        </div>
    }
}
```

</details>

### Multi-step Modal

<details>
<summary>View Code</summary>

```rust
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
fn MultiStepModal() -> impl IntoView {
    let (modal_open, set_modal_open) = signal(false);
    let (current_step, set_current_step) = signal(1);
    let (form_data, set_form_data) = signal(HashMap::new());
    
    let handle_next = move |_| {
        set_current_step.update(|step| *step += 1);
    };
    
    let handle_previous = move |_| {
        set_current_step.update(|step| *step -= 1);
    };
    
    let handle_finish = move |_| {
        // Process form data
        set_modal_open.set(false);
        set_current_step.set(1);
    };
    
    view! {
        <div>
            <Button 
                style=Signal::derive(|| ButtonStyle::Primary)
                on:click=move |_| set_modal_open.set(true)
            >
                "Start Setup"
            </Button>
            
            <Modal 
                open=Signal::derive(move || modal_open.get())
                backdrop_close=Signal::derive(|| false)
            >
                <ModalBox class="w-11/12 max-w-2xl">
                    <h3 class="font-bold text-lg">
                        {move || format!("Setup - Step {} of 3", current_step.get())}
                    </h3>
                    
                    // Progress indicator
                    <div class="flex justify-between items-center mt-4 mb-6">
                        {(1..=3).map(|step| {
                            let is_current = move || current_step.get() == step;
                            let is_completed = move || current_step.get() > step;
                            view! {
                                <div class=format!(
                                    "w-8 h-8 rounded-full flex items-center justify-center text-sm font-bold {}",
                                    if is_completed() || is_current() { "bg-primary text-primary-content" } else { "bg-base-300" }
                                )>
                                    {step}
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                    
                    // Step content
                    <div class="py-4">
                        {move || match current_step.get() {
                            1 => view! {
                                <div>
                                    <h4 class="font-semibold mb-2">"Personal Information"</h4>
                                    <div class="space-y-4">
                                        <Input placeholder="Full Name" />
                                        <Input placeholder="Email Address" />
                                    </div>
                                </div>
                            }.into_any(),
                            2 => view! {
                                <div>
                                    <h4 class="font-semibold mb-2">"Preferences"</h4>
                                    <div class="space-y-4">
                                        <Checkbox>"Enable notifications"</Checkbox>
                                        <Checkbox>"Subscribe to newsletter"</Checkbox>
                                    </div>
                                </div>
                            }.into_any(),
                            3 => view! {
                                <div>
                                    <h4 class="font-semibold mb-2">"Review"</h4>
                                    <p>"Please review your information and click finish to complete setup."</p>
                                </div>
                            }.into_any(),
                            _ => view! { <div></div> }.into_any()
                        }}
                    </div>
                    
                    <ModalAction class="justify-between">
                        <div>
                            {move || if current_step.get() > 1 {
                                view! {
                                    <Button 
                                        style=Signal::derive(|| ButtonStyle::Ghost)
                                        on:click=handle_previous
                                    >
                                        "Previous"
                                    </Button>
                                }.into_any()
                            } else {
                                view! { <div></div> }.into_any()
                            }}
                        </div>
                        <div class="space-x-2">
                            <Button 
                                style=Signal::derive(|| ButtonStyle::Ghost)
                                on:click=move |_| {
                                    set_modal_open.set(false);
                                    set_current_step.set(1);
                                }
                            >
                                "Cancel"
                            </Button>
                            {move || if current_step.get() < 3 {
                                view! {
                                    <Button 
                                        style=Signal::derive(|| ButtonStyle::Primary)
                                        on:click=handle_next
                                    >
                                        "Next"
                                    </Button>
                                }.into_any()
                            } else {
                                view! {
                                    <Button 
                                        style=Signal::derive(|| ButtonStyle::Primary)
                                        on:click=handle_finish
                                    >
                                        "Finish"
                                    </Button>
                                }.into_any()
                            }}
                        </div>
                    </ModalAction>
                </ModalBox>
            </Modal>
        </div>
    }
}
```

</details>

## Accessibility

- Proper focus management when modal opens/closes
- Escape key support for closing modal
- ARIA attributes for screen readers
- Focus trap within modal content
- Backdrop click to close (configurable)

## Best Practices

1. Always provide a way to close the modal
2. Use appropriate modal sizes for content
3. Consider keyboard navigation and accessibility
4. Disable backdrop close for critical operations
5. Show loading states for async operations
6. Use confirmation patterns for destructive actions
7. Keep modal content focused and concise