# Input

A versatile input component with support for different types, sizes, states, and validation styling.

## Description

The Input component provides a comprehensive form input solution with built-in styling, validation states, and accessibility features. It supports various input types, sizes, and visual states to match different use cases.

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `input_type` | `Signal<InputType>` | `InputType::Text` | Type of input field |
| `size` | `Signal<InputSize>` | `InputSize::Default` | Size of the input |
| `color` | `Signal<InputColor>` | `InputColor::Default` | Color theme of the input |
| `bordered` | `Signal<bool>` | `true` | Whether input has a border |
| `ghost` | `Signal<bool>` | `false` | Whether input has ghost style |
| `value` | `Signal<String>` | `""` | Current value of the input |
| `placeholder` | `&'static str` | `""` | Placeholder text |
| `disabled` | `Signal<bool>` | `false` | Whether input is disabled |
| `readonly` | `Signal<bool>` | `false` | Whether input is read-only |
| `class` | `&'static str` | `""` | Additional CSS classes |
| `on:input` | `Option<EventHandler>` | - | Input event handler |
| `on:change` | `Option<EventHandler>` | - | Change event handler |
| `on:focus` | `Option<EventHandler>` | - | Focus event handler |
| `on:blur` | `Option<EventHandler>` | - | Blur event handler |

## Style Variants

### InputType
- `Text` - Text input
- `Password` - Password input
- `Email` - Email input
- `Number` - Number input
- `Tel` - Telephone input
- `Url` - URL input
- `Search` - Search input
- `Date` - Date input
- `Time` - Time input
- `DateTime` - DateTime input
- `File` - File input

### InputSize
- `Default` - Standard size
- `Large` - Large input
- `Small` - Small input
- `ExtraSmall` - Extra small input

### InputColor
- `Default` - Default styling
- `Primary` - Primary color
- `Secondary` - Secondary color
- `Accent` - Accent color
- `Info` - Info color
- `Success` - Success color
- `Warning` - Warning color
- `Error` - Error color

## Examples

### Basic Inputs

<details>
<summary>View Code</summary>

```rust
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
fn BasicInputs() -> impl IntoView {
    let (text_value, set_text_value) = signal("".to_string());
    let (email_value, set_email_value) = signal("".to_string());
    let (password_value, set_password_value) = signal("".to_string());
    
    view! {
        <div class="space-y-4">
            <div class="form-control w-full max-w-xs">
                <label class="label">
                    <span class="label-text">"Text Input"</span>
                </label>
                <Input 
                    placeholder="Enter your name"
                    value=Signal::derive(move || text_value.get())
                    on:input=move |ev| set_text_value.set(event_target_value(&ev))
                />
            </div>
            
            <div class="form-control w-full max-w-xs">
                <label class="label">
                    <span class="label-text">"Email Input"</span>
                </label>
                <Input 
                    input_type=Signal::derive(|| InputType::Email)
                    placeholder="your.email@example.com"
                    value=Signal::derive(move || email_value.get())
                    on:input=move |ev| set_email_value.set(event_target_value(&ev))
                />
            </div>
            
            <div class="form-control w-full max-w-xs">
                <label class="label">
                    <span class="label-text">"Password Input"</span>
                </label>
                <Input 
                    input_type=Signal::derive(|| InputType::Password)
                    placeholder="Enter password"
                    value=Signal::derive(move || password_value.get())
                    on:input=move |ev| set_password_value.set(event_target_value(&ev))
                />
            </div>
        </div>
    }
}
```

</details>

### Input Sizes

<details>
<summary>View Code</summary>

```rust
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
fn InputSizes() -> impl IntoView {
    view! {
        <div class="space-y-4">
            <div class="form-control w-full max-w-xs">
                <label class="label">
                    <span class="label-text">"Extra Small"</span>
                </label>
                <Input 
                    size=Signal::derive(|| InputSize::ExtraSmall)
                    placeholder="Extra small input"
                />
            </div>
            
            <div class="form-control w-full max-w-xs">
                <label class="label">
                    <span class="label-text">"Small"</span>
                </label>
                <Input 
                    size=Signal::derive(|| InputSize::Small)
                    placeholder="Small input"
                />
            </div>
            
            <div class="form-control w-full max-w-xs">
                <label class="label">
                    <span class="label-text">"Normal"</span>
                </label>
                <Input placeholder="Normal input" />
            </div>
            
            <div class="form-control w-full max-w-xs">
                <label class="label">
                    <span class="label-text">"Large"</span>
                </label>
                <Input 
                    size=Signal::derive(|| InputSize::Large)
                    placeholder="Large input"
                />
            </div>
        </div>
    }
}
```

</details>

### Input Colors & States

<details>
<summary>View Code</summary>

```rust
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
fn InputStates() -> impl IntoView {
    view! {
        <div class="space-y-4">
            <div class="form-control w-full max-w-xs">
                <label class="label">
                    <span class="label-text">"Primary"</span>
                </label>
                <Input 
                    color=Signal::derive(|| InputColor::Primary)
                    placeholder="Primary input"
                />
            </div>
            
            <div class="form-control w-full max-w-xs">
                <label class="label">
                    <span class="label-text">"Success"</span>
                </label>
                <Input 
                    color=Signal::derive(|| InputColor::Success)
                    placeholder="Success input"
                    value=Signal::derive(|| "Valid input".to_string())
                />
                <label class="label">
                    <span class="label-text-alt text-success">"Email is valid"</span>
                </label>
            </div>
            
            <div class="form-control w-full max-w-xs">
                <label class="label">
                    <span class="label-text">"Warning"</span>
                </label>
                <Input 
                    color=Signal::derive(|| InputColor::Warning)
                    placeholder="Warning input"
                    value=Signal::derive(|| "Weak password".to_string())
                />
                <label class="label">
                    <span class="label-text-alt text-warning">"Password strength: weak"</span>
                </label>
            </div>
            
            <div class="form-control w-full max-w-xs">
                <label class="label">
                    <span class="label-text">"Error"</span>
                </label>
                <Input 
                    color=Signal::derive(|| InputColor::Error)
                    placeholder="Error input"
                    value=Signal::derive(|| "invalid-email".to_string())
                />
                <label class="label">
                    <span class="label-text-alt text-error">"Invalid email format"</span>
                </label>
            </div>
            
            <div class="form-control w-full max-w-xs">
                <label class="label">
                    <span class="label-text">"Disabled"</span>
                </label>
                <Input 
                    disabled=Signal::derive(|| true)
                    placeholder="Disabled input"
                    value=Signal::derive(|| "Cannot edit this".to_string())
                />
            </div>
        </div>
    }
}
```

</details>

### Ghost Style Input

<details>
<summary>View Code</summary>

```rust
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
fn GhostInputs() -> impl IntoView {
    view! {
        <div class="space-y-4">
            <div class="form-control w-full max-w-xs">
                <label class="label">
                    <span class="label-text">"Ghost Style"</span>
                </label>
                <Input 
                    ghost=Signal::derive(|| true)
                    placeholder="Ghost input (no border)"
                />
            </div>
            
            <div class="form-control w-full max-w-xs">
                <label class="label">
                    <span class="label-text">"Borderless"</span>
                </label>
                <Input 
                    bordered=Signal::derive(|| false)
                    placeholder="No border input"
                />
            </div>
        </div>
    }
}
```

</details>

### Form Validation Example

<details>
<summary>View Code</summary>

```rust
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
fn ValidationForm() -> impl IntoView {
    let (email, set_email) = signal("".to_string());
    let (password, set_password) = signal("".to_string());
    let (confirm_password, set_confirm_password) = signal("".to_string());
    
    let email_valid = move || {
        let email_val = email.get();
        email_val.contains('@') && email_val.contains('.')
    };
    
    let password_valid = move || password.get().len() >= 8;
    
    let passwords_match = move || {
        !password.get().is_empty() && password.get() == confirm_password.get()
    };
    
    let form_valid = move || email_valid() && password_valid() && passwords_match();
    
    view! {
        <div class="space-y-4 w-full max-w-xs">
            <div class="form-control">
                <label class="label">
                    <span class="label-text">"Email"</span>
                </label>
                <Input 
                    input_type=Signal::derive(|| InputType::Email)
                    placeholder="your.email@example.com"
                    value=Signal::derive(move || email.get())
                    color=Signal::derive(move || {
                        if email.get().is_empty() {
                            InputColor::Default
                        } else if email_valid() {
                            InputColor::Success
                        } else {
                            InputColor::Error
                        }
                    })
                    on:input=move |ev| set_email.set(event_target_value(&ev))
                />
                <label class="label">
                    <span class=format!(
                        "label-text-alt {}",
                        if email.get().is_empty() {
                            ""
                        } else if email_valid() {
                            "text-success"
                        } else {
                            "text-error"
                        }
                    )>
                        {move || {
                            if email.get().is_empty() {
                                "Enter your email address".to_string()
                            } else if email_valid() {
                                "Email is valid".to_string()
                            } else {
                                "Please enter a valid email".to_string()
                            }
                        }}
                    </span>
                </label>
            </div>
            
            <div class="form-control">
                <label class="label">
                    <span class="label-text">"Password"</span>
                </label>
                <Input 
                    input_type=Signal::derive(|| InputType::Password)
                    placeholder="At least 8 characters"
                    value=Signal::derive(move || password.get())
                    color=Signal::derive(move || {
                        if password.get().is_empty() {
                            InputColor::Default
                        } else if password_valid() {
                            InputColor::Success
                        } else {
                            InputColor::Error
                        }
                    })
                    on:input=move |ev| set_password.set(event_target_value(&ev))
                />
                <label class="label">
                    <span class=format!(
                        "label-text-alt {}",
                        if password.get().is_empty() {
                            ""
                        } else if password_valid() {
                            "text-success"
                        } else {
                            "text-error"
                        }
                    )>
                        {move || {
                            if password.get().is_empty() {
                                "Password must be at least 8 characters".to_string()
                            } else if password_valid() {
                                "Password strength: good".to_string()
                            } else {
                                "Password too short".to_string()
                            }
                        }}
                    </span>
                </label>
            </div>
            
            <div class="form-control">
                <label class="label">
                    <span class="label-text">"Confirm Password"</span>
                </label>
                <Input 
                    input_type=Signal::derive(|| InputType::Password)
                    placeholder="Repeat your password"
                    value=Signal::derive(move || confirm_password.get())
                    color=Signal::derive(move || {
                        if confirm_password.get().is_empty() {
                            InputColor::Default
                        } else if passwords_match() {
                            InputColor::Success
                        } else {
                            InputColor::Error
                        }
                    })
                    on:input=move |ev| set_confirm_password.set(event_target_value(&ev))
                />
                <label class="label">
                    <span class=format!(
                        "label-text-alt {}",
                        if confirm_password.get().is_empty() {
                            ""
                        } else if passwords_match() {
                            "text-success"
                        } else {
                            "text-error"
                        }
                    )>
                        {move || {
                            if confirm_password.get().is_empty() {
                                "Confirm your password".to_string()
                            } else if passwords_match() {
                                "Passwords match".to_string()
                            } else {
                                "Passwords do not match".to_string()
                            }
                        }}
                    </span>
                </label>
            </div>
            
            <div class="form-control mt-6">
                <Button 
                    style=Signal::derive(|| ButtonStyle::Primary)
                    disabled=Signal::derive(move || !form_valid())
                    class="w-full"
                >
                    "Create Account"
                </Button>
            </div>
        </div>
    }
}
```

</details>

### Search Input with Clear

<details>
<summary>View Code</summary>

```rust
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
fn SearchInput() -> impl IntoView {
    let (search_value, set_search_value) = signal("".to_string());
    
    view! {
        <div class="form-control w-full max-w-xs">
            <label class="label">
                <span class="label-text">"Search"</span>
            </label>
            <div class="relative">
                <Input 
                    input_type=Signal::derive(|| InputType::Search)
                    placeholder="Search products..."
                    value=Signal::derive(move || search_value.get())
                    on:input=move |ev| set_search_value.set(event_target_value(&ev))
                    class="pr-10"
                />
                {move || if !search_value.get().is_empty() {
                    view! {
                        <button 
                            class="absolute right-3 top-1/2 transform -translate-y-1/2 text-gray-400 hover:text-gray-600"
                            on:click=move |_| set_search_value.set("".to_string())
                        >
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                            </svg>
                        </button>
                    }.into_any()
                } else {
                    view! {
                        <div class="absolute right-3 top-1/2 transform -translate-y-1/2 text-gray-400">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                            </svg>
                        </div>
                    }.into_any()
                }}
            </div>
            {move || if !search_value.get().is_empty() {
                view! {
                    <label class="label">
                        <span class="label-text-alt">
                            {format!("Searching for: \"{}\"", search_value.get())}
                        </span>
                    </label>
                }.into_any()
            } else {
                view! { <div></div> }.into_any()
            }}
        </div>
    }
}
```

</details>

## Accessibility

- Proper label association with `for` attributes
- ARIA attributes for validation states
- Keyboard navigation support
- Screen reader compatible error messages
- Focus management and indication

## Best Practices

1. Always provide descriptive labels for inputs
2. Use appropriate input types for better UX and validation
3. Provide clear validation feedback
4. Use placeholder text wisely (not as labels)
5. Consider input grouping for related fields
6. Implement proper error handling and recovery
7. Test with keyboard navigation and screen readers