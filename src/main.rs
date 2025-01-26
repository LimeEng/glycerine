use glycerine::{gluetun, Config, QBit};
use reqwest::Client;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .pretty()
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Setting default subscriber failed");

    let config = Config::load();

    let qbit = QBit::new(config.qbit_url, config.qbit_username, config.qbit_password);
    let client = Client::new();

    let warmup_time = 15;
    tracing::info!("Polling every {} seconds", config.poll_interval);
    tracing::info!("Polling begins in {warmup_time} seconds");

    // Let the services start up
    tokio::time::sleep(tokio::time::Duration::from_secs(warmup_time)).await;

    let mut last_known_port: Option<u16> = None;
    let mut last_update_failed = false;
    loop {
        let port = gluetun::fetch_port(&client, &config.gluetun_url).await;
        if let Some(port) = port {
            let port_changed = last_known_port != Some(port);
            if port_changed || last_update_failed {
                tracing::info!("Detected port change: {port}");
                last_known_port = Some(port);
                let result = qbit.update(port).await;
                match result {
                    Ok(success) => last_update_failed = !success,
                    Err(e) => {
                        tracing::error!("Error: {e}");
                        last_update_failed = true;
                    }
                }
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(config.poll_interval)).await;
    }
}
