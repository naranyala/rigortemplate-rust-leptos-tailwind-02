use leptos::prelude::*;
use std::sync::Arc;
use crate::core::api::ApiClient;

pub fn provide_services() {
    let api_client = ApiClient::new("https://api.example.com");
    provide_context(api_client);
}

pub fn use_api() -> Arc<ApiClient> {
    use_context::<Arc<ApiClient>>().expect("ApiClient not provided")
}
