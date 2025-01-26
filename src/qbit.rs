use reqwest::Client;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug)]
pub struct QBit {
    client: Client,
    url: Url,
    username: String,
    password: String,
}

impl QBit {
    #[must_use]
    pub fn new(url: Url, username: String, password: String) -> Self {
        let client = Client::builder().cookie_store(true).build().unwrap();
        Self {
            client,
            url,
            username,
            password,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct PreferencePayload {
    listen_port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
struct Preferences {
    listen_port: u16,
}

impl QBit {
    async fn login(&self) -> Result<bool, reqwest::Error> {
        let target = self.url.join("/api/v2/auth/login").unwrap();

        let params = [
            ("username", self.username.as_str()),
            ("password", self.password.as_str()),
        ];

        let response = self.client.post(target).form(&params).send().await?;
        Ok(response.status().is_success())
    }

    async fn update_port(&self, port: u16) -> Result<bool, reqwest::Error> {
        let target = self.url.join("/api/v2/app/setPreferences").unwrap();

        let json = serde_json::to_string(&PreferencePayload { listen_port: port }).unwrap();

        let response = self
            .client
            .post(target)
            .form(&[("json", json)])
            .send()
            .await?;
        Ok(response.status().is_success())
    }

    async fn get_port(&self) -> Result<u16, reqwest::Error> {
        let target = self.url.join("/api/v2/app/preferences").unwrap();

        let response = self.client.get(target).send().await?;
        let preferences: Preferences = response.json().await?;

        Ok(preferences.listen_port)
    }

    pub async fn update(&self, port: u16) -> Result<bool, reqwest::Error> {
        let mut update_result = self.update_port(port).await?;
        if !update_result {
            if self.login().await? {
                tracing::info!("Logged in to qbittorrent");
                update_result = self.update_port(port).await?;
            } else {
                tracing::error!("Failed to login to qbittorrent");
            }
        }
        if update_result {
            let response = self.get_port().await?;
            if response == port {
                tracing::info!("Updated qbittorrent port to {port}");
                return Ok(true);
            }
        }
        tracing::error!("Failed to update qbittorrent port to {port}");
        Ok(false)
    }
}
