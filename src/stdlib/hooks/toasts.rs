use leptos::prelude::*;

#[derive(Clone, Debug)]
pub struct ToastNotification {
    pub id: u64,
    pub message: String,
    pub toast_type: String, // "success", "error", "info"
}

pub fn use_toasts() -> (ReadSignal<Vec<ToastNotification>>, WriteSignal<Vec<ToastNotification>>) {
    signal(Vec::<ToastNotification>::new())
}

#[allow(dead_code)]
pub fn add_toast(set_toasts: &WriteSignal<Vec<ToastNotification>>, message: String, toast_type: String) {
    set_toasts.update(|toasts| {
        toasts.push(ToastNotification {
            id: js_sys::Math::random() as u64,
            message,
            toast_type,
        });
    });
}
