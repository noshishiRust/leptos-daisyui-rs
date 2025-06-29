# Alert

A notification component for displaying important messages with various styles and severity levels.

## Description

The Alert component provides a way to display contextual feedback messages to users. It supports different severity levels, icons, and can be dismissible. Perfect for success messages, warnings, errors, and informational notices.

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `alert_type` | `Signal<AlertType>` | `AlertType::Default` | Type/severity of the alert |
| `dismissible` | `Signal<bool>` | `false` | Whether alert can be dismissed |
| `on_dismiss` | `Option<Callback>` | - | Callback when alert is dismissed |
| `class` | `&'static str` | `""` | Additional CSS classes |
| `children` | `Children` | - | Alert content |

## Alert Types

### AlertType
- `Default` - Standard alert appearance
- `Info` - Informational message (blue)
- `Success` - Success message (green)
- `Warning` - Warning message (yellow/orange)
- `Error` - Error message (red)

## Examples

### Basic Alerts

<details>
<summary>View Code</summary>

```rust
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
fn BasicAlerts() -> impl IntoView {
    view! {
        <div class="space-y-4">
            <Alert>"This is a default alert message."</Alert>
            
            <Alert alert_type=Signal::derive(|| AlertType::Info)>
                "This is an informational alert."
            </Alert>
            
            <Alert alert_type=Signal::derive(|| AlertType::Success)>
                "Operation completed successfully!"
            </Alert>
            
            <Alert alert_type=Signal::derive(|| AlertType::Warning)>
                "Please review your input before proceeding."
            </Alert>
            
            <Alert alert_type=Signal::derive(|| AlertType::Error)>
                "An error occurred while processing your request."
            </Alert>
        </div>
    }
}
```

</details>

### Dismissible Alerts

<details>
<summary>View Code</summary>

```rust
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
fn DismissibleAlerts() -> impl IntoView {
    let (show_success, set_show_success) = signal(true);
    let (show_warning, set_show_warning) = signal(true);
    let (show_error, set_show_error) = signal(true);
    
    view! {
        <div class="space-y-4">
            {move || if show_success.get() {
                view! {
                    <Alert 
                        alert_type=Signal::derive(|| AlertType::Success)
                        dismissible=Signal::derive(|| true)
                        on_dismiss=Callback::new(move |_| set_show_success.set(false))
                    >
                        "File uploaded successfully! You can now process it."
                    </Alert>
                }.into_any()
            } else {
                view! { <div></div> }.into_any()
            }}
            
            {move || if show_warning.get() {
                view! {
                    <Alert 
                        alert_type=Signal::derive(|| AlertType::Warning)
                        dismissible=Signal::derive(|| true)
                        on_dismiss=Callback::new(move |_| set_show_warning.set(false))
                    >
                        "Your session will expire in 5 minutes. Please save your work."
                    </Alert>
                }.into_any()
            } else {
                view! { <div></div> }.into_any()
            }}
            
            {move || if show_error.get() {
                view! {
                    <Alert 
                        alert_type=Signal::derive(|| AlertType::Error)
                        dismissible=Signal::derive(|| true)
                        on_dismiss=Callback::new(move |_| set_show_error.set(false))
                    >
                        "Connection failed. Please check your internet connection and try again."
                    </Alert>
                }.into_any()
            } else {
                view! { <div></div> }.into_any()
            }}
            
            <div class="mt-4">
                <Button 
                    style=Signal::derive(|| ButtonStyle::Ghost)
                    on:click=move |_| {
                        set_show_success.set(true);
                        set_show_warning.set(true);
                        set_show_error.set(true);
                    }
                >
                    "Reset Alerts"
                </Button>
            </div>
        </div>
    }
}
```

</details>

### Alerts with Icons

<details>
<summary>View Code</summary>

```rust
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
fn IconAlerts() -> impl IntoView {
    view! {
        <div class="space-y-4">
            <Alert alert_type=Signal::derive(|| AlertType::Info)>
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-current shrink-0 w-6 h-6">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                </svg>
                <span>"New software update available. Click here to download."</span>
            </Alert>
            
            <Alert alert_type=Signal::derive(|| AlertType::Success)>
                <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
                <span>"Your purchase has been confirmed!"</span>
            </Alert>
            
            <Alert alert_type=Signal::derive(|| AlertType::Warning)>
                <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.732-.833-2.5 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z" />
                </svg>
                <span>"Warning: Invalid email address!"</span>
            </Alert>
            
            <Alert alert_type=Signal::derive(|| AlertType::Error)>
                <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
                <span>"Error! Task failed successfully."</span>
            </Alert>
        </div>
    }
}
```

</details>

### Alerts with Actions

<details>
<summary>View Code</summary>

```rust
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
fn ActionAlerts() -> impl IntoView {
    let (cookie_accepted, set_cookie_accepted) = signal(false);
    let (update_dismissed, set_update_dismissed) = signal(false);
    
    view! {
        <div class="space-y-4">
            {move || if !cookie_accepted.get() {
                view! {
                    <Alert>
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-current shrink-0 w-6 h-6">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                        </svg>
                        <div>
                            <h3 class="font-bold">"Cookie Notice"</h3>
                            <div class="text-xs">"We use cookies to improve your experience."</div>
                        </div>
                        <div class="flex-none">
                            <Button 
                                size=Signal::derive(|| ButtonSize::Small)
                                style=Signal::derive(|| ButtonStyle::Ghost)
                                on:click=move |_| set_cookie_accepted.set(true)
                            >
                                "Deny"
                            </Button>
                            <Button 
                                size=Signal::derive(|| ButtonSize::Small)
                                style=Signal::derive(|| ButtonStyle::Primary)
                                on:click=move |_| set_cookie_accepted.set(true)
                            >
                                "Accept"
                            </Button>
                        </div>
                    </Alert>
                }.into_any()
            } else {
                view! { <div></div> }.into_any()
            }}
            
            {move || if !update_dismissed.get() {
                view! {
                    <Alert alert_type=Signal::derive(|| AlertType::Info)>
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-current shrink-0 w-6 h-6">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                        </svg>
                        <div>
                            <h3 class="font-bold">"Update Available"</h3>
                            <div class="text-xs">"A new version of the app is available. Update now for the latest features."</div>
                        </div>
                        <div class="flex-none">
                            <Button 
                                size=Signal::derive(|| ButtonSize::Small)
                                style=Signal::derive(|| ButtonStyle::Ghost)
                                on:click=move |_| set_update_dismissed.set(true)
                            >
                                "Later"
                            </Button>
                            <Button 
                                size=Signal::derive(|| ButtonSize::Small)
                                style=Signal::derive(|| ButtonStyle::Primary)
                            >
                                "Update"
                            </Button>
                        </div>
                    </Alert>
                }.into_any()
            } else {
                view! { <div></div> }.into_any()
            }}
        </div>
    }
}
```

</details>

### Auto-dismiss Alert

<details>
<summary>View Code</summary>

```rust
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
fn AutoDismissAlert() -> impl IntoView {
    let (show_alert, set_show_alert) = signal(false);
    
    let show_temporary_alert = move |_| {
        set_show_alert.set(true);
        
        // Auto-dismiss after 3 seconds
        set_timeout(move || {
            set_show_alert.set(false);
        }, Duration::from_secs(3));
    };
    
    view! {
        <div class="space-y-4">
            <Button 
                style=Signal::derive(|| ButtonStyle::Primary)
                on:click=show_temporary_alert
            >
                "Show Temporary Alert"
            </Button>
            
            {move || if show_alert.get() {
                view! {
                    <Alert alert_type=Signal::derive(|| AlertType::Success)>
                        <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                        </svg>
                        <span>"This alert will disappear in 3 seconds!"</span>
                    </Alert>
                }.into_any()
            } else {
                view! { <div></div> }.into_any()
            }}
        </div>
    }
}
```

</details>

### Alert List Manager

<details>
<summary>View Code</summary>

```rust
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[derive(Clone, Debug)]
struct AlertMessage {
    id: u32,
    message: String,
    alert_type: AlertType,
}

#[component]
fn AlertManager() -> impl IntoView {
    let (alerts, set_alerts) = signal(Vec::<AlertMessage>::new());
    let (next_id, set_next_id) = signal(1u32);
    
    let add_alert = move |alert_type: AlertType, message: &str| {
        let id = next_id.get();
        set_next_id.update(|id| *id += 1);
        
        set_alerts.update(|alerts| {
            alerts.push(AlertMessage {
                id,
                message: message.to_string(),
                alert_type,
            });
        });
        
        // Auto-remove after 5 seconds
        set_timeout(move || {
            set_alerts.update(|alerts| {
                alerts.retain(|alert| alert.id != id);
            });
        }, Duration::from_secs(5));
    };
    
    let remove_alert = move |id: u32| {
        set_alerts.update(|alerts| {
            alerts.retain(|alert| alert.id != id);
        });
    };
    
    view! {
        <div class="space-y-4">
            <div class="flex flex-wrap gap-2">
                <Button 
                    color=Signal::derive(|| ButtonColor::Success)
                    on:click=move |_| add_alert(AlertType::Success, "Operation completed successfully!")
                >
                    "Add Success"
                </Button>
                <Button 
                    color=Signal::derive(|| ButtonColor::Warning)
                    on:click=move |_| add_alert(AlertType::Warning, "Please review your input.")
                >
                    "Add Warning"
                </Button>
                <Button 
                    color=Signal::derive(|| ButtonColor::Error)
                    on:click=move |_| add_alert(AlertType::Error, "An error occurred!")
                >
                    "Add Error"
                </Button>
                <Button 
                    color=Signal::derive(|| ButtonColor::Info)
                    on:click=move |_| add_alert(AlertType::Info, "Here's some information.")
                >
                    "Add Info"
                </Button>
            </div>
            
            <div class="space-y-2">
                {move || {
                    alerts.get().into_iter().map(|alert| {
                        let alert_id = alert.id;
                        view! {
                            <Alert 
                                key=alert_id
                                alert_type=Signal::derive(move || alert.alert_type.clone())
                                dismissible=Signal::derive(|| true)
                                on_dismiss=Callback::new(move |_| remove_alert(alert_id))
                            >
                                {alert.message}
                            </Alert>
                        }
                    }).collect::<Vec<_>>()
                }}
            </div>
        </div>
    }
}
```

</details>

## Accessibility

- Proper ARIA roles and attributes for screen readers
- Color and icon combinations for accessibility
- Keyboard navigation support for dismissible alerts
- Focus management when alerts appear/disappear

## Best Practices

1. Use appropriate alert types for different situations
2. Keep messages concise and actionable
3. Provide clear next steps in alert content
4. Use icons to reinforce the message type
5. Consider auto-dismiss for non-critical messages
6. Group related alerts appropriately
7. Don't overwhelm users with too many alerts at once
