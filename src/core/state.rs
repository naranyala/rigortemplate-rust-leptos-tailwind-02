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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_global_state_initialization() {
        let state = GlobalState::new();
        assert_eq!(state.user_name.get(), "Guest");
        assert_eq!(state.is_authenticated.get(), false);
        assert!(state.notifications.get().is_empty());
    }

    #[test]
    fn test_add_notification() {
        let state = GlobalState::new();
        state.add_notification("Test message".to_string(), NotificationType::Info);
        
        let notifications = state.notifications.get();
        assert_eq!(notifications.len(), 1);
        assert_eq!(notifications[0].message, "Test message");
        assert!(matches!(notifications[0].type_, NotificationType::Info));
    }

    #[test]
    fn test_state_updates() {
        let state = GlobalState::new();
        state.user_name.set("Alice".to_string());
        state.is_authenticated.set(true);
        
        assert_eq!(state.user_name.get(), "Alice");
        assert_eq!(state.is_authenticated.get(), true);
    }
}
