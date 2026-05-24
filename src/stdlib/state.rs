use leptos::prelude::*;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct GlobalState {
    pub user_name: RwSignal<String>,
    pub is_authenticated: RwSignal<bool>,
    pub notifications: RwSignal<Vec<Notification>>,
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

impl GlobalState {
    pub fn new() -> Self {
        Self {
            user_name: RwSignal::new("Guest".to_string()),
            is_authenticated: RwSignal::new(false),
            notifications: RwSignal::new(Vec::new()),
        }
    }

    pub fn add_notification(&self, message: String, type_: NotificationType) {
        self.notifications.update(|n| {
            n.push(Notification {
                id: n.len(),
                message,
                type_,
            });
        });
    }
}

pub fn provide_global_state() {
    provide_context(GlobalState::new());
}

pub fn use_global_state() -> GlobalState {
    use_context::<GlobalState>().expect("GlobalState not provided in context")
}
