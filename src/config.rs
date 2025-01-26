use std::env;
use url::Url;

#[derive(Debug)]
pub struct Config {
    pub gluetun_url: Url,
    pub poll_interval: u64,
    pub qbit_url: Url,
    pub qbit_username: String,
    pub qbit_password: String,
}

impl Config {
    #[must_use]
    pub fn load() -> Self {
        let gluetun_url = env::var("GLUETUN_URL")
            .expect("GLUETUN_URL must be specified")
            .parse()
            .expect("GLUETUN_URL must be a valid URL");
        let poll_interval = env::var("POLL_INTERVAL")
            .expect("POLL_INTERVAL must be specified")
            .parse()
            .expect("POLL_INTERVAL must be a number");
        let qbit_url = env::var("QBIT_URL")
            .expect("QBIT_URL must be specified")
            .parse()
            .expect("QBIT_URL must be a valid URL");
        let qbit_username = env::var("QBIT_USERNAME").expect("QBIT_USERNAME must be specified");
        let qbit_password = env::var("QBIT_PASSWORD").expect("QBIT_PASSWORD must be specified");
        Self {
            gluetun_url,
            poll_interval,
            qbit_url,
            qbit_username,
            qbit_password,
        }
    }
}
