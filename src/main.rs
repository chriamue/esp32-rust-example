use esp32_rust_example::{wifi::WifiConnectFix, *};
use esp_idf_svc::{eventloop::EspSystemEventLoop, hal::peripherals::Peripherals, wifi::WifiEvent};

use anyhow::Ok;
use esp_idf_sys::esp_app_desc;
use log::info;

esp_app_desc!();

const WIFI_SSID: &str = env!("WIFI_SSID");

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();

    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;
    let Peripherals {
        modem,
        pins,
        i2c1,
        uart1,
        ..
    } = peripherals;

    log::info!("Hello, world!");

    log::info!("WIFI_SSID: {}", WIFI_SSID);

    log::info!("Starting server...");

    let mut blocking_wifi = wifi::wifi(modem, sysloop.clone())?;

    blocking_wifi.connect_with_retry()?;

    info!("Connected to wifi");

    info!("Shutting down in 5s...");

    std::thread::sleep(core::time::Duration::from_secs(5));

    Ok(())
}
