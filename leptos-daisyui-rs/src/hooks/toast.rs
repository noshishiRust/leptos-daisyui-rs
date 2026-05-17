//! # ToastProvider + use_toast
//!
//! Notification queue management with auto-dismiss timers.

use leptos::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

/// Severity type for toast notifications.
#[derive(Clone, Debug, Default)]
pub enum ToastType {
    /// Informational message (default).
    #[default]
    Info,
    /// Success message.
    Success,
    /// Warning message.
    Warning,
    /// Error message.
    Error,
}

impl ToastType {
    /// Returns the daisyUI alert CSS class for this toast type.
    pub fn alert_class(&self) -> &'static str {
        match self {
            ToastType::Info => "alert-info",
            ToastType::Success => "alert-success",
            ToastType::Warning => "alert-warning",
            ToastType::Error => "alert-error",
        }
    }
}

/// A single toast notification in the queue.
#[derive(Clone, Debug)]
pub struct ToastEntry {
    /// Unique identifier for this toast.
    pub id: usize,
    /// Message content.
    pub message: String,
    /// Toast severity type.
    pub toast_type: ToastType,
    /// Optional auto-dismiss duration. `None` = no auto-dismiss.
    pub duration: Option<Duration>,
}

/// Reactive toast state provided by [`ToastProvider`].
///
/// The provider manages the notification queue. Rendering is the user's responsibility
/// — use the existing `Toast` and `Alert` components to display notifications.
///
/// ## Example
///
/// ```rust,ignore
/// let toast = use_toast();
/// toast.add("Saved!".to_string());
/// toast.add_with_duration(ToastType::Success, "Saved!".into(), Duration::from_secs(3));
/// ```
#[derive(Clone)]
pub struct ToastContext {
    /// The queue of active toasts.
    pub toasts: RwSignal<Vec<ToastEntry>>,
    /// Add a simple info toast with default 5s auto-dismiss.
    pub add: ArcLocalCallback<String>,
    /// Add a typed toast with default 5s auto-dismiss.
    pub add_typed: ArcLocalCallback<(ToastType, String)>,
    /// Add a toast with custom auto-dismiss duration.
    pub add_with_duration: ArcLocalCallback<(ToastType, String, Duration)>,
    /// Remove a toast by ID.
    pub dismiss: ArcLocalCallback<usize>,
    /// Remove all toasts.
    pub clear: ArcLocalCallback<()>,
}

static TOAST_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Provider component that manages a toast notification queue.
///
/// Place this near the root of your app. It provides toast state to
/// descendants via context. Rendering is the user's responsibility.
///
/// ## Example
///
/// ```rust,ignore
/// view! {
///     <ToastProvider>
///         <App />
///     </ToastProvider>
/// }
///
/// // In a descendant component:
/// let toast = use_toast();
/// toast.add("Hello!".to_string());
/// ```
#[component]
pub fn ToastProvider(children: Children) -> impl IntoView {
    let toasts = RwSignal::new(Vec::<ToastEntry>::new());

    let dismiss: ArcLocalCallback<usize> = ArcLocalCallback::new(move |id: usize| {
        toasts.write().retain(|t| t.id != id);
    });

    let toasts_for_add = toasts;
    let dismiss_for_timer = dismiss.clone();

    let add_with_duration: ArcLocalCallback<(ToastType, String, Duration)> =
        ArcLocalCallback::new(move |(toast_type, message, duration)| {
            let id = TOAST_COUNTER.fetch_add(1, Ordering::Relaxed);
            let entry = ToastEntry {
                id,
                message,
                toast_type,
                duration: Some(duration),
            };
            toasts_for_add.write().push(entry);

            // Schedule auto-dismiss
            let dismiss = dismiss_for_timer.clone();
            set_timeout(
                move || {
                    dismiss.invoke(id);
                },
                duration,
            );
        });

    let add_with_duration_for_typed = add_with_duration.clone();

    let add_typed: ArcLocalCallback<(ToastType, String)> =
        ArcLocalCallback::new(move |(toast_type, message)| {
            add_with_duration_for_typed.invoke((toast_type, message, Duration::from_secs(5)));
        });

    let add_typed_for_simple = add_typed.clone();

    let add: ArcLocalCallback<String> = ArcLocalCallback::new(move |message| {
        add_typed_for_simple.invoke((ToastType::Info, message));
    });

    let toasts_for_clear = toasts;
    let clear: ArcLocalCallback<()> = ArcLocalCallback::new(move |()| {
        toasts_for_clear.write().clear();
    });

    provide_context(ToastContext {
        toasts,
        add,
        add_typed,
        add_with_duration,
        dismiss,
        clear,
    });

    children()
}

/// Hook to access toast functionality from a [`ToastProvider`] ancestor.
///
/// ## Example
///
/// ```rust,ignore
/// let toast = use_toast();
/// toast.add_typed.invoke((ToastType::Success, "Saved!".to_string()));
/// ```
pub fn use_toast() -> ToastContext {
    use_context::<ToastContext>().expect("use_toast requires a ToastProvider ancestor")
}

/// A clonable closure that can be invoked from synchronous code.
///
/// Similar to Leptos's `Callback` but with an `invoke` method for direct calls.
#[derive(Clone)]
pub struct ArcLocalCallback<T> {
    f: Arc<dyn Fn(T) + Send + Sync>,
}

impl<T> ArcLocalCallback<T> {
    /// Create a new callback from a closure.
    pub fn new(f: impl Fn(T) + Send + Sync + 'static) -> Self {
        Self {
            f: Arc::new(f),
        }
    }

    /// Invoke the callback.
    pub fn invoke(&self, arg: T) {
        (self.f)(arg);
    }
}
