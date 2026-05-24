use leptos::prelude::*;
use std::sync::Arc;
use crate::stdlib::services::api::ApiClient;

/// ServiceProvider handles the initialization and provision of global services.
pub struct ServiceProvider;

impl ServiceProvider {
    pub fn provide_all() {
        // 1. Core API Client
        let api_client = ApiClient::new("https://api.example.com");
        provide_context(api_client);
        
        // 2. Other services could be added here
        // provide_context(AuthService::new(api_client.clone()));
    }
}

/// Helper to consume services
#[allow(dead_code)]
pub fn use_api() -> Arc<ApiClient> {
    use_context::<Arc<ApiClient>>().expect("ApiClient not provided")
}
