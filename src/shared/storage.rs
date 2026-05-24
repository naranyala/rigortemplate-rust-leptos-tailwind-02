use leptos::prelude::*;
use gloo_storage::{LocalStorage, Storage};

#[allow(dead_code)]
pub fn use_local_storage<T>(key: &'static str, initial_value: T) -> (ReadSignal<T>, WriteSignal<T>)
where
    T: Clone + serde::Serialize + serde::de::DeserializeOwned + 'static + Send + Sync,
{
    // Gracefully handle local storage unavailability
    let initial = LocalStorage::get(key).unwrap_or_else(|_| initial_value.clone());

    let (get, set) = signal(initial);

    Effect::new(move |_| {
        let val = get.with(|v| v.clone());
        if let Err(e) = LocalStorage::set(key, val) {
            tracing::error!("LocalStorage save failed: {}", e);
        }
    });

    (get, set)
}
