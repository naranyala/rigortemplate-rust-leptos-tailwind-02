use reqwest::Client;
use std::sync::Arc;
use crate::core::errors::AppError;

pub struct ApiClient {
    client: Client,
    base_url: String,
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
    struct TestData {
        id: u32,
        name: String,
    }

    #[tokio::test]
    async fn test_api_client_get_success() {
        let mut server = Server::new_async().await;
        let mock = server.mock("GET", "/test")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id": 1, "name": "Test"}"#)
            .create_async().await;

        let client = ApiClient::new(&server.url());
        let res: Result<TestData, AppError> = client.get("/test").await;
        
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TestData { id: 1, name: "Test".to_string() });
        mock.assert();
    }

    #[tokio::test]
    async fn test_api_client_get_failure() {
        let mut server = Server::new_async().await;
        let mock = server.mock("GET", "/fail")
            .with_status(500)
            .create_async().await;

        let client = ApiClient::new(&server.url());
        let res: Result<TestData, AppError> = client.get("/fail").await;
        
        assert!(res.is_err());
        mock.assert();
    }

    #[tokio::test]
    async fn test_api_client_post_success() {
        let mut server = Server::new_async().await;
        let mock = server.mock("POST", "/test")
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id": 2, "name": "Posted"}"#)
            .create_async().await;

        let client = ApiClient::new(&server.url());
        let body = TestData { id: 2, name: "Posted".to_string() };
        let res: Result<TestData, AppError> = client.post("/test", &body).await;
        
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TestData { id: 2, name: "Posted".to_string() });
        mock.assert();
    }
}
