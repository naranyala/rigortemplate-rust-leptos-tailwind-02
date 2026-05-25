use leptos::prelude::*;

pub fn provide_global_state() {
    provide_context(RwSignal::new("Guest".to_string()));
    provide_context(RwSignal::new(false));
    provide_context(RwSignal::new(Vec::<Notification>::new()));
}

pub fn use_user_name() -> RwSignal<String> {
    use_context::<RwSignal<String>>().expect("user_name not provided")
}

pub fn use_is_authenticated() -> RwSignal<bool> {
    use_context::<RwSignal<bool>>().expect("is_authenticated not provided")
}

pub fn use_notifications() -> RwSignal<Vec<Notification>> {
    use_context::<RwSignal<Vec<Notification>>>().expect("notifications not provided")
}

#[derive(Clone, Debug)]
pub struct Notification {
    pub id: usize,
    pub message: String,
    pub type_: NotificationType,
}

#[derive(Clone, Debug)]
pub enum NotificationType {
    Info,
    Success,
    Warning,
    Error,
}

pub fn add_notification(message: String, type_: NotificationType) {
    use_notifications().update(|n| {
        n.push(Notification {
            id: n.len(),
            message,
            type_,
        });
    });
}
