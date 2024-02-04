use esp32_rust_example::{wifi::Wifi, *};
use esp_idf_svc::hal::task::block_on;

use anyhow::Ok;
use esp_idf_sys::esp_app_desc;
use log::info;

esp_app_desc!();

const WIFI_SSID: &str = env!("WIFI_SSID");

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();

    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    log::info!("WIFI_SSID: {}", WIFI_SSID);

    log::info!("Starting server...");

    let mut wifi = Wifi::new()?;

    block_on(run(&mut wifi))?;

    info!("Shutting down in 5s...");

    std::thread::sleep(core::time::Duration::from_secs(5));

    Ok(())
}

async fn run(wifi: &mut Wifi<'static>) -> anyhow::Result<()> {
    wifi.configure().await?;
    wifi.connect().await?;

    server::server().await?;

    /*     loop {
            tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
        }

        wifi.disconnect().await?;
    */
    Ok(())
}
