use leptos::prelude::*;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct ToastNotification {
    pub id: u64,
    pub message: String,
    pub toast_type: String,
}

#[derive(Clone, Debug)]
pub struct ToastContext {
    pub toasts: ReadSignal<Vec<ToastNotification>>,
    pub add_toast: Callback<(String, String)>,
}

pub fn provide_toast_context() {
    let (toasts, set_toasts) = signal(Vec::<ToastNotification>::new());

    let add_toast = Callback::new(move |(message, toast_type): (String, String)| {
        let id = js_sys::Math::random() as u64;
        set_toasts.update(|t| {
            t.push(ToastNotification { id, message, toast_type });
        });
        set_timeout(
            move || {
                set_toasts.update(|t| t.retain(|n| n.id != id));
            },
            Duration::from_secs(4),
        );
    });

    provide_context(ToastContext { toasts, add_toast });
}

pub fn use_toasts() -> ToastContext {
    use_context::<ToastContext>().expect("ToastContext not provided")
}
