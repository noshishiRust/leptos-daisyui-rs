# Checkbox

A flexible checkbox component with various sizes, colors, and states for user selection.

## Description

The Checkbox component provides an accessible way to handle boolean selections and multiple choice options. It supports different sizes, colors, and states including indeterminate state for partial selections.

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `checked` | `Signal<bool>` | `false` | Whether checkbox is checked |
| `indeterminate` | `Signal<bool>` | `false` | Whether checkbox is in indeterminate state |
| `size` | `Signal<CheckboxSize>` | `CheckboxSize::Default` | Size of the checkbox |
| `color` | `Signal<CheckboxColor>` | `CheckboxColor::Default` | Color theme of the checkbox |
| `disabled` | `Signal<bool>` | `false` | Whether checkbox is disabled |
| `class` | `&'static str` | `""` | Additional CSS classes |
| `on:change` | `Option<EventHandler>` | - | Change event handler |
| `children` | `Children` | - | Label content (optional) |

## Style Variants

### CheckboxSize
- `Default` - Standard size
- `Large` - Large checkbox
- `Small` - Small checkbox
- `ExtraSmall` - Extra small checkbox

### CheckboxColor
- `Default` - Default color
- `Primary` - Primary color
- `Secondary` - Secondary color
- `Accent` - Accent color
- `Success` - Success color
- `Warning` - Warning color
- `Info` - Info color
- `Error` - Error color

## Examples

### Basic Checkboxes

<details>
<summary>View Code</summary>

```rust
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
fn BasicCheckboxes() -> impl IntoView {
    let (checked1, set_checked1) = signal(false);
    let (checked2, set_checked2) = signal(true);
    let (checked3, set_checked3) = signal(false);
    
    view! {
        <div class="space-y-4">
            <div class="form-control">
                <label class="label cursor-pointer justify-start space-x-2">
                    <Checkbox 
                        checked=Signal::derive(move || checked1.get())
                        on:change=move |_| set_checked1.update(|c| *c = !*c)
                    />
                    <span class="label-text">"Remember me"</span>
                </label>
            </div>
            
            <div class="form-control">
                <label class="label cursor-pointer justify-start space-x-2">
                    <Checkbox 
                        checked=Signal::derive(move || checked2.get())
                        on:change=move |_| set_checked2.update(|c| *c = !*c)
                    />
                    <span class="label-text">"Accept terms and conditions"</span>
                </label>
            </div>
            
            <div class="form-control">
                <label class="label cursor-pointer justify-start space-x-2">
                    <Checkbox 
                        checked=Signal::derive(move || checked3.get())
                        disabled=Signal::derive(|| true)
                    />
                    <span class="label-text opacity-50">"Disabled option"</span>
                </label>
            </div>
        </div>
    }
}
```

</details>

### Checkbox Sizes

<details>
<summary>View Code</summary>

```rust
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
fn CheckboxSizes() -> impl IntoView {
    let (xs_checked, set_xs_checked) = signal(false);
    let (sm_checked, set_sm_checked) = signal(true);
    let (md_checked, set_md_checked) = signal(false);
    let (lg_checked, set_lg_checked) = signal(true);
    
    view! {
        <div class="space-y-4">
            <div class="form-control">
                <label class="label cursor-pointer justify-start space-x-2">
                    <Checkbox 
                        size=Signal::derive(|| CheckboxSize::ExtraSmall)
                        checked=Signal::derive(move || xs_checked.get())
                        on:change=move |_| set_xs_checked.update(|c| *c = !*c)
                    />
                    <span class="label-text">"Extra Small"</span>
                </label>
            </div>
            
            <div class="form-control">
                <label class="label cursor-pointer justify-start space-x-2">
                    <Checkbox 
                        size=Signal::derive(|| CheckboxSize::Small)
                        checked=Signal::derive(move || sm_checked.get())
                        on:change=move |_| set_sm_checked.update(|c| *c = !*c)
                    />
                    <span class="label-text">"Small"</span>
                </label>
            </div>
            
            <div class="form-control">
                <label class="label cursor-pointer justify-start space-x-2">
                    <Checkbox 
                        checked=Signal::derive(move || md_checked.get())
                        on:change=move |_| set_md_checked.update(|c| *c = !*c)
                    />
                    <span class="label-text">"Normal"</span>
                </label>
            </div>
            
            <div class="form-control">
                <label class="label cursor-pointer justify-start space-x-2">
                    <Checkbox 
                        size=Signal::derive(|| CheckboxSize::Large)
                        checked=Signal::derive(move || lg_checked.get())
                        on:change=move |_| set_lg_checked.update(|c| *c = !*c)
                    />
                    <span class="label-text">"Large"</span>
                </label>
            </div>
        </div>
    }
}
```

</details>

### Checkbox Colors

<details>
<summary>View Code</summary>

```rust
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
fn CheckboxColors() -> impl IntoView {
    let (primary_checked, set_primary_checked) = signal(true);
    let (secondary_checked, set_secondary_checked) = signal(true);
    let (accent_checked, set_accent_checked) = signal(true);
    let (success_checked, set_success_checked) = signal(true);
    let (warning_checked, set_warning_checked) = signal(true);
    let (info_checked, set_info_checked) = signal(true);
    let (error_checked, set_error_checked) = signal(true);
    
    view! {
        <div class="space-y-4">
            <div class="form-control">
                <label class="label cursor-pointer justify-start space-x-2">
                    <Checkbox 
                        color=Signal::derive(|| CheckboxColor::Primary)
                        checked=Signal::derive(move || primary_checked.get())
                        on:change=move |_| set_primary_checked.update(|c| *c = !*c)
                    />
                    <span class="label-text">"Primary"</span>
                </label>
            </div>
            
            <div class="form-control">
                <label class="label cursor-pointer justify-start space-x-2">
                    <Checkbox 
                        color=Signal::derive(|| CheckboxColor::Secondary)
                        checked=Signal::derive(move || secondary_checked.get())
                        on:change=move |_| set_secondary_checked.update(|c| *c = !*c)
                    />
                    <span class="label-text">"Secondary"</span>
                </label>
            </div>
            
            <div class="form-control">
                <label class="label cursor-pointer justify-start space-x-2">
                    <Checkbox 
                        color=Signal::derive(|| CheckboxColor::Accent)
                        checked=Signal::derive(move || accent_checked.get())
                        on:change=move |_| set_accent_checked.update(|c| *c = !*c)
                    />
                    <span class="label-text">"Accent"</span>
                </label>
            </div>
            
            <div class="form-control">
                <label class="label cursor-pointer justify-start space-x-2">
                    <Checkbox 
                        color=Signal::derive(|| CheckboxColor::Success)
                        checked=Signal::derive(move || success_checked.get())
                        on:change=move |_| set_success_checked.update(|c| *c = !*c)
                    />
                    <span class="label-text">"Success"</span>
                </label>
            </div>
            
            <div class="form-control">
                <label class="label cursor-pointer justify-start space-x-2">
                    <Checkbox 
                        color=Signal::derive(|| CheckboxColor::Warning)
                        checked=Signal::derive(move || warning_checked.get())
                        on:change=move |_| set_warning_checked.update(|c| *c = !*c)
                    />
                    <span class="label-text">"Warning"</span>
                </label>
            </div>
            
            <div class="form-control">
                <label class="label cursor-pointer justify-start space-x-2">
                    <Checkbox 
                        color=Signal::derive(|| CheckboxColor::Info)
                        checked=Signal::derive(move || info_checked.get())
                        on:change=move |_| set_info_checked.update(|c| *c = !*c)
                    />
                    <span class="label-text">"Info"</span>
                </label>
            </div>
            
            <div class="form-control">
                <label class="label cursor-pointer justify-start space-x-2">
                    <Checkbox 
                        color=Signal::derive(|| CheckboxColor::Error)
                        checked=Signal::derive(move || error_checked.get())
                        on:change=move |_| set_error_checked.update(|c| *c = !*c)
                    />
                    <span class="label-text">"Error"</span>
                </label>
            </div>
        </div>
    }
}
```

</details>

### Indeterminate State

<details>
<summary>View Code</summary>

```rust
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
fn IndeterminateCheckbox() -> impl IntoView {
    let (option1, set_option1) = signal(true);
    let (option2, set_option2) = signal(false);
    let (option3, set_option3) = signal(true);
    
    let parent_state = move || {
        let checked_count = [option1.get(), option2.get(), option3.get()]
            .iter()
            .filter(|&&x| x)
            .count();
        
        match checked_count {
            0 => (false, false), // unchecked, not indeterminate
            3 => (true, false),  // checked, not indeterminate
            _ => (false, true),  // unchecked, indeterminate
        }
    };
    
    let toggle_all = move |_| {
        let (is_checked, _) = parent_state();
        let new_value = !is_checked;
        set_option1.set(new_value);
        set_option2.set(new_value);
        set_option3.set(new_value);
    };
    
    view! {
        <div class="space-y-4">
            <div class="form-control">
                <label class="label cursor-pointer justify-start space-x-2">
                    <Checkbox 
                        checked=Signal::derive(move || parent_state().0)
                        indeterminate=Signal::derive(move || parent_state().1)
                        on:change=toggle_all
                    />
                    <span class="label-text font-semibold">"Select All Features"</span>
                </label>
            </div>
            
            <div class="ml-6 space-y-2">
                <div class="form-control">
                    <label class="label cursor-pointer justify-start space-x-2">
                        <Checkbox 
                            checked=Signal::derive(move || option1.get())
                            on:change=move |_| set_option1.update(|c| *c = !*c)
                        />
                        <span class="label-text">"Email notifications"</span>
                    </label>
                </div>
                
                <div class="form-control">
                    <label class="label cursor-pointer justify-start space-x-2">
                        <Checkbox 
                            checked=Signal::derive(move || option2.get())
                            on:change=move |_| set_option2.update(|c| *c = !*c)
                        />
                        <span class="label-text">"Push notifications"</span>
                    </label>
                </div>
                
                <div class="form-control">
                    <label class="label cursor-pointer justify-start space-x-2">
                        <Checkbox 
                            checked=Signal::derive(move || option3.get())
                            on:change=move |_| set_option3.update(|c| *c = !*c)
                        />
                        <span class="label-text">"SMS notifications"</span>
                    </label>
                </div>
            </div>
        </div>
    }
}
```

</details>

### Checkbox List with Selection

<details>
<summary>View Code</summary>

```rust
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[derive(Clone, PartialEq)]
struct TodoItem {
    id: u32,
    text: String,
    completed: bool,
}

#[component]
fn CheckboxList() -> impl IntoView {
    let (todos, set_todos) = signal(vec![
        TodoItem { id: 1, text: "Learn Leptos".to_string(), completed: true },
        TodoItem { id: 2, text: "Build awesome UI".to_string(), completed: false },
        TodoItem { id: 3, text: "Deploy to production".to_string(), completed: false },
        TodoItem { id: 4, text: "Celebrate success".to_string(), completed: false },
    ]);
    
    let toggle_todo = move |id: u32| {
        set_todos.update(|todos| {
            if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
                todo.completed = !todo.completed;
            }
        });
    };
    
    let completed_count = move || {
        todos.get().iter().filter(|todo| todo.completed).count()
    };
    
    let total_count = move || todos.get().len();
    
    view! {
        <div class="space-y-4">
            <div class="flex justify-between items-center">
                <h3 class="text-lg font-semibold">"Todo List"</h3>
                <div class="text-sm text-gray-600">
                    {move || format!("{}/{} completed", completed_count(), total_count())}
                </div>
            </div>
            
            <div class="space-y-2">
                {move || {
                    todos.get().into_iter().map(|todo| {
                        let todo_id = todo.id;
                        let is_completed = todo.completed;
                        view! {
                            <div key=todo_id class="form-control">
                                <label class="label cursor-pointer justify-start space-x-3 p-3 rounded-lg hover:bg-base-200">
                                    <Checkbox 
                                        checked=Signal::derive(move || is_completed)
                                        color=Signal::derive(|| CheckboxColor::Success)
                                        on:change=move |_| toggle_todo(todo_id)
                                    />
                                    <span class=format!(
                                        "label-text {}",
                                        if is_completed { "line-through opacity-60" } else { "" }
                                    )>
                                        {todo.text}
                                    </span>
                                </label>
                            </div>
                        }
                    }).collect::<Vec<_>>()
                }}
            </div>
            
            <div class="flex justify-between items-center pt-4 border-t">
                <span class="text-sm text-gray-600">
                    {move || {
                        let remaining = total_count() - completed_count();
                        format!("{} items remaining", remaining)
                    }}
                </span>
                <Button 
                    size=Signal::derive(|| ButtonSize::Small)
                    style=Signal::derive(|| ButtonStyle::Ghost)
                    on:click=move |_| {
                        set_todos.update(|todos| {
                            todos.retain(|todo| !todo.completed);
                        });
                    }
                >
                    "Clear completed"
                </Button>
            </div>
        </div>
    }
}
```

</details>

### Checkbox Group with Validation

<details>
<summary>View Code</summary>

```rust
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
fn CheckboxGroup() -> impl IntoView {
    let (selected_interests, set_selected_interests) = signal(Vec::<String>::new());
    let (submitted, set_submitted) = signal(false);
    
    let interests = vec![
        "Web Development",
        "Mobile Apps", 
        "Data Science",
        "Machine Learning",
        "DevOps",
        "UI/UX Design",
        "Game Development",
        "Cybersecurity"
    ];
    
    let toggle_interest = move |interest: String| {
        set_selected_interests.update(|selected| {
            if selected.contains(&interest) {
                selected.retain(|i| i != &interest);
            } else {
                selected.push(interest);
            }
        });
    };
    
    let is_valid = move || selected_interests.get().len() >= 2;
    
    let handle_submit = move |_| {
        if is_valid() {
            set_submitted.set(true);
        }
    };
    
    view! {
        <div class="space-y-6">
            <div>
                <h3 class="text-lg font-semibold mb-2">"Select Your Interests"</h3>
                <p class="text-sm text-gray-600 mb-4">"Choose at least 2 areas of interest"</p>
                
                <div class="grid grid-cols-2 gap-2">
                    {interests.into_iter().map(|interest| {
                        let interest_clone = interest.to_string();
                        let interest_check = interest_clone.clone();
                        view! {
                            <div class="form-control">
                                <label class="label cursor-pointer justify-start space-x-2">
                                    <Checkbox 
                                        checked=Signal::derive(move || {
                                            selected_interests.get().contains(&interest_check)
                                        })
                                        on:change=move |_| toggle_interest(interest_clone.clone())
                                    />
                                    <span class="label-text">{interest}</span>
                                </label>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
                
                <div class="mt-4">
                    {move || if selected_interests.get().len() > 0 && selected_interests.get().len() < 2 {
                        view! {
                            <p class="text-sm text-warning">
                                {format!("Please select at least {} more interest(s)", 2 - selected_interests.get().len())}
                            </p>
                        }.into_any()
                    } else if selected_interests.get().len() >= 2 {
                        view! {
                            <p class="text-sm text-success">
                                {format!("{} interests selected", selected_interests.get().len())}
                            </p>
                        }.into_any()
                    } else {
                        view! { <div></div> }.into_any()
                    }}
                </div>
            </div>
            
            <div class="flex justify-between items-center">
                <Button 
                    style=Signal::derive(|| ButtonStyle::Primary)
                    disabled=Signal::derive(move || !is_valid())
                    on:click=handle_submit
                >
                    "Continue"
                </Button>
                
                {move || if submitted.get() {
                    view! {
                        <div class="text-success text-sm">
                            "Preferences saved!"
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }}
            </div>
        </div>
    }
}
```

</details>

## Accessibility

- Proper ARIA attributes for screen readers
- Keyboard navigation support (Space to toggle)
- Label association for checkbox accessibility
- Indeterminate state announced to assistive technology
- Focus indicators for keyboard users

## Best Practices

1. Always provide clear labels for checkboxes
2. Use indeterminate state for partial selections
3. Group related checkboxes logically
4. Provide validation feedback when needed
5. Consider the visual hierarchy of checkbox groups
6. Use appropriate colors for different contexts
7. Test with keyboard navigation and screen readers