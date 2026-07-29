use anyhow::Context;
use std::env;
use url::Url;

fn require_env<T>(
    name: &str,
    parse: impl FnOnce(String) -> anyhow::Result<T>,
) -> anyhow::Result<T> {
    let value = env::var(name).with_context(|| format!("{name} must be specified"))?;
    parse(value).with_context(|| format!("{name} must be a valid value"))
}

#[derive(Debug)]
pub struct Config {
    pub gluetun_url: Url,
    pub poll_interval_seconds: u64,
    pub qbit_url: Url,
    pub qbit_username: String,
    pub qbit_password: String,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let gluetun_url = require_env("GLUETUN_URL", |value| Ok(value.parse::<Url>()?))?;
        let poll_interval_seconds =
            require_env("POLL_INTERVAL_SECONDS", |value| Ok(value.parse::<u64>()?))?;
        let qbit_url = require_env("QBIT_URL", |value| Ok(value.parse::<Url>()?))?;
        let qbit_username = require_env("QBIT_USERNAME", Ok)?;
        let qbit_password = require_env("QBIT_PASSWORD", Ok)?;
        Ok(Self {
            gluetun_url,
            poll_interval_seconds,
            qbit_url,
            qbit_username,
            qbit_password,
        })
    }
}
