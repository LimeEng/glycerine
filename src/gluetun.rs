use anyhow::Context;
use reqwest::Client;
use serde::Deserialize;
use url::Url;

const API_PORTFORWARD_PATH: &str = "/v1/portforward";

#[derive(Deserialize)]
struct PortResponse {
    port: u16,
}

pub struct Gluetun {
    client: Client,
    url: Url,
}

impl Gluetun {
    pub fn new(url: Url) -> anyhow::Result<Self> {
        let client = Client::new();
        Ok(Self { client, url })
    }
}

impl Gluetun {
    pub async fn fetch_port(&self) -> anyhow::Result<u16> {
        let portforward_url = self.url.join(API_PORTFORWARD_PATH).context("invalid URL")?;

        self.client
            .get(portforward_url)
            .send()
            .await
            .context("failed to connect to Gluetun API")?
            .json::<PortResponse>()
            .await
            .context("Gluetun API error")
            .map(|data| data.port)
    }
}
