use reqwest::Client;
use serde::Deserialize;
use url::Url;

#[derive(Deserialize)]
struct PortResponse {
    port: u16,
}

pub async fn fetch_port(client: &Client, gluetun_url: &Url) -> Option<u16> {
    let target = gluetun_url
        .join("/v1/openvpn/portforwarded")
        .expect("Invalid URL");

    let result = client.get(target).send().await;

    if let Ok(response) = result {
        let data = response.json::<PortResponse>().await;
        if let Ok(data) = data {
            Some(data.port)
        } else {
            tracing::error!("Gluetun API error");
            None
        }
    } else {
        tracing::error!("Failed to connect to Gluetun API");
        None
    }
}
