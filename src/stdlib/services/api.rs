use reqwest::Client;
use std::sync::Arc;
use crate::stdlib::services::errors::AppError;

#[allow(dead_code)]
pub struct ApiClient {
    client: Client,
    base_url: String,
}

#[allow(dead_code)]
impl ApiClient {
    pub fn new(base_url: &str) -> Arc<Self> {
        Arc::new(Self {
            client: Client::new(),
            base_url: base_url.to_string(),
        })
    }

    pub async fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T, AppError> {
        let url = format!("{}{}", self.base_url, path);
        let res = self.client.get(url)
            .send()
            .await
            .map_err(|e| AppError::Network(e.to_string()))?;

        if res.status().is_success() {
            res.json::<T>().await.map_err(|e| AppError::Serialization(e.to_string()))
        } else {
            Err(AppError::Internal(format!("HTTP {}", res.status())))
        }
    }

    pub async fn post<T: serde::Serialize, R: serde::de::DeserializeOwned>(&self, path: &str, body: &T) -> Result<R, AppError> {
        let url = format!("{}{}", self.base_url, path);
        let res = self.client.post(url)
            .json(body)
            .send()
            .await
            .map_err(|e| AppError::Network(e.to_string()))?;

        if res.status().is_success() {
            res.json::<R>().await.map_err(|e| AppError::Serialization(e.to_string()))
        } else {
            Err(AppError::Internal(format!("HTTP {}", res.status())))
        }
    }
}
