use glycerine::{Config, Gluetun, QBit};
use std::time::Duration;
use tokio::time::sleep;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .pretty()
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Setting default subscriber failed");

    let config = Config::load()?;
    let qbit = QBit::new(
        config.qbit_url.clone(),
        config.qbit_username.clone(),
        config.qbit_password.clone(),
    )?;
    let gluetun = Gluetun::new(config.gluetun_url.clone())?;

    tracing::info!("Polling every {} seconds", config.poll_interval_seconds);
    tracing::info!("Polling begins in 15 seconds");
    sleep(Duration::from_secs(15)).await;

    poll_loop(config, qbit, gluetun).await;
    Ok(())
}

async fn poll_loop(config: Config, qbit: QBit, gluetun: Gluetun) {
    let mut last_known_port = None;
    let mut last_update_failed = false;
    loop {
        if let Err(error) = poll(
            &qbit,
            &gluetun,
            &mut last_known_port,
            &mut last_update_failed,
        )
        .await
        {
            tracing::error!("Poll error: {error}");
        }
        sleep(Duration::from_secs(config.poll_interval_seconds)).await;
    }
}

async fn poll(
    qbit: &QBit,
    gluetun: &Gluetun,
    last_known_port: &mut Option<u16>,
    last_update_failed: &mut bool,
) -> anyhow::Result<()> {
    let port = gluetun.fetch_port().await?;

    let port_changed = *last_known_port != Some(port);
    if !(port_changed || *last_update_failed) {
        return Ok(());
    }

    tracing::info!("Detected port change: {port}");
    *last_known_port = Some(port);

    match qbit.update(port).await {
        Ok(()) => {
            tracing::info!("Updated qbittorrent port to {port}");
            *last_update_failed = false;
            Ok(())
        }
        Err(error) => {
            *last_update_failed = true;
            Err(error)
        }
    }
}
