use esp32_rust_example::{display::Display, wifi::WifiConnectFix, *};
use esp_idf_hal::{
    delay::FreeRtos,
    gpio::PinDriver,
    i2c::{I2cConfig, I2cDriver},
    prelude::FromValueType,
};
use esp_idf_svc::{eventloop::EspSystemEventLoop, hal::peripherals::Peripherals, wifi::WifiEvent};

use anyhow::Ok;
use esp_idf_sys::esp_app_desc;
use log::info;
use ssd1306::{
    mode::DisplayConfig, rotation::DisplayRotation, size::DisplaySize128x64, I2CDisplayInterface,
    Ssd1306,
};

esp_app_desc!();

const WIFI_SSID: &str = env!("WIFI_SSID");

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();

    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;

    //let display = display::Display::new();
    //display.run(&peripherals, "Hello, world!")?;

    let Peripherals {
        modem,
        pins,
        i2c0,
        i2c1,
        uart1,
        ..
    } = peripherals;

    let mut led = PinDriver::output(pins.gpio35)?;

    let rst = pins.gpio21;
    let sda = pins.gpio17;
    let scl = pins.gpio18;

    let mut oled_reset = PinDriver::output(rst)?;

    oled_reset.set_high()?;
    FreeRtos::delay_ms(1);
    oled_reset.set_low()?;
    FreeRtos::delay_ms(10);
    oled_reset.set_high()?;

    let config = I2cConfig::new().baudrate(400.kHz().into());
    let i2c = I2cDriver::new(i2c0, sda, scl, &config)?;

    let interface = I2CDisplayInterface::new(i2c);

    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display
        .init()
        .map_err(|e| anyhow::anyhow!("Display error: {:?}", e))?;

    //////////////////

    log::info!("Hello, world!");

    Display::run(&mut display, "Hello, world!")?;

    log::info!("WIFI_SSID: {}", WIFI_SSID);

    Display::run(&mut display, WIFI_SSID)?;

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

    drop(peripherals);
    drop(display);
    drop(blocking_wifi);
    drop(server);

    drop(peripherals); // i2c0 still used until display is dropped
    Ok(())
}
