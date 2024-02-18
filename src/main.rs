use esp32_rust_example::{display::Display, wifi::WifiConnectFix, *};
use esp_idf_hal::{delay::FreeRtos, gpio::PinDriver};
use esp_idf_svc::{eventloop::EspSystemEventLoop, hal::peripherals::Peripherals};
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
        modem, pins, i2c0, ..
    } = peripherals;

    #[cfg(feature = "esp32")]
    let mut led = PinDriver::output(pins.gpio25)?;
    #[cfg(feature = "esp32s3")]
    let mut led = PinDriver::output(pins.gpio35)?;

    #[cfg(feature = "esp32")]
    let rst = pins.gpio16;
    #[cfg(feature = "esp32s3")]
    let rst = pins.gpio21;

    #[cfg(feature = "esp32")]
    let sda = pins.gpio4;
    #[cfg(feature = "esp32s3")]
    let sda = pins.gpio17;

    #[cfg(feature = "esp32")]
    let scl = pins.gpio15;
    #[cfg(feature = "esp32s3")]
    let scl = pins.gpio18;

    let mut display = Display::new(i2c0, rst, sda, scl)?;

    display.init()?;
    display.reset()?;

    //////////////////

    log::info!("Hello, world!");

    Display::print(&mut display, "Hello, world!")?;

    log::info!("WIFI_SSID: {}", WIFI_SSID);

    Display::print(&mut display, &format!("SSID: {}", WIFI_SSID))?;

    log::info!("Starting server...");

    let mut blocking_wifi = wifi::wifi(modem, sysloop.clone())?;

    blocking_wifi.connect_with_retry()?;

    info!("Connected to wifi");

    info!("Shutting down in 5s...");

    let server = server::server();

    std::thread::sleep(core::time::Duration::from_secs(5));

    tokio::spawn(async move {
        server.await.unwrap();
    });

    loop {
        led.set_high()?;
        // we are sleeping here to make sure the watchdog isn't triggered
        FreeRtos::delay_ms(1000);

        led.set_low()?;
        FreeRtos::delay_ms(1000);
    }
}
