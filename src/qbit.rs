use anyhow::{Context, bail};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use url::Url;

const API_LOGIN_PATH: &str = "/api/v2/auth/login";
const API_SET_PREFERENCES_PATH: &str = "/api/v2/app/setPreferences";
const API_GET_PREFERENCES_PATH: &str = "/api/v2/app/preferences";

#[derive(Debug)]
pub struct QBit {
    client: Client,
    url: Url,
    username: String,
    password: String,
}

impl QBit {
    pub fn new(url: Url, username: String, password: String) -> anyhow::Result<Self> {
        let client = Client::builder()
            .cookie_store(true)
            .build()
            .context("failed to construct HTTP client")?;
        Ok(Self {
            client,
            url,
            username,
            password,
        })
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
    async fn login(&self) -> anyhow::Result<()> {
        let target = self.url.join(API_LOGIN_PATH)?;

        let params = [
            ("username", self.username.as_str()),
            ("password", self.password.as_str()),
        ];

        let response = self.client.post(target).form(&params).send().await?;
        if response.status().is_success() {
            Ok(())
        } else {
            bail!("failed to login to qbittorrent")
        }
    }

    async fn update_port(&self, port: u16) -> anyhow::Result<()> {
        let target = self.url.join(API_SET_PREFERENCES_PATH)?;

        let json = serde_json::to_string(&PreferencePayload { listen_port: port })?;

        let response = self
            .client
            .post(target)
            .form(&[("json", json)])
            .send()
            .await?;
        if response.status().is_success() {
            Ok(())
        } else {
            bail!("failed to update port of qbittorrent")
        }
    }

    async fn get_port(&self) -> anyhow::Result<u16> {
        let target = self.url.join(API_GET_PREFERENCES_PATH)?;

        let response = self.client.get(target).send().await?;
        let preferences: Preferences = response.json().await?;

        Ok(preferences.listen_port)
    }

    pub async fn update(&self, port: u16) -> anyhow::Result<()> {
        let mut update_result = self.update_port(port).await.is_ok();
        if !update_result {
            let () = self.login().await?;
            tracing::info!("Logged in to qbittorrent");
            update_result = self.update_port(port).await.is_ok();
        }

        if update_result {
            let response = self.get_port().await?;
            if response == port {
                return Ok(());
            }
        }

        bail!("failed to update qbittorrent port to {port}")
    }
}
