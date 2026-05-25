use leptos::prelude::*;
use gloo_storage::{LocalStorage, Storage};

/// Trait to abstract storage operations for testing
pub trait StorageBackend<T> {
    fn get(&self, key: &str) -> Result<T, String>;
    fn set(&self, key: &str, value: T) -> Result<(), String>;
}

pub struct LocalStorageBackend;

impl<T> StorageBackend<T> for LocalStorageBackend 
where T: Clone + serde::Serialize + serde::de::DeserializeOwned 
{
    fn get(&self, key: &str) -> Result<T, String> {
        LocalStorage::get(key).map_err(|e| e.to_string())
    }
    fn set(&self, key: &str, value: T) -> Result<(), String> {
        LocalStorage::set(key, value).map_err(|e| e.to_string())
    }
}

#[allow(dead_code)]
pub fn use_local_storage<T, B>(key: &'static str, initial_value: T, backend: B) -> (ReadSignal<T>, WriteSignal<T>)
where
    T: Clone + serde::Serialize + serde::de::DeserializeOwned + 'static + Send + Sync,
    B: StorageBackend<T> + 'static,
{
    let initial = backend.get(key).unwrap_or_else(|_| initial_value.clone());

    let (get, set) = signal(initial);

    Effect::new(move |_| {
        let val = get.with(|v| v.clone());
        if let Err(e) = backend.set(key, val) {
            tracing::error!("Storage save failed: {}", e);
        }
    });

    (get, set)
}

/// Helper for the actual application usage
pub fn use_local_storage_default<T>(key: &'static str, initial_value: T) -> (ReadSignal<T>, WriteSignal<T>)
where
    T: Clone + serde::Serialize + serde::de::DeserializeOwned + 'static + Send + Sync,
{
    use_local_storage(key, initial_value, LocalStorageBackend)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockStorage<T> {
        data: std::cell::RefCell<std::collections::HashMap<String, T>>,
    }

    impl<T: Clone + serde::Serialize + serde::de::DeserializeOwned + 'static> StorageBackend<T> for MockStorage<T> {
        fn get(&self, key: &str) -> Result<T, String> {
            self.data.borrow().get(key).cloned().ok_or_else(|| "Not found".to_string())
        }
        fn set(&self, key: &str, value: T) -> Result<(), String> {
            self.data.borrow_mut().insert(key.to_string(), value);
            Ok(())
        }
    }

    #[test]
    fn test_storage_backend_mock() {
        let storage = MockStorage {
            data: std::cell::RefCell::new(std::collections::HashMap::new()),
        };
        
        storage.set("test_key", 10).unwrap();
        assert_eq!(storage.get("test_key").unwrap(), 10);
    }
}
