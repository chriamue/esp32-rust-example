// source: https://github.com/andy31415/rust-esp32-c3-demos/blob/oled_wifi/src/main.rs
// https://randomnerdtutorials.com/esp32-ssd1306-oled-display-arduino-ide/
// https://www.roboter-bausatz.de/p/lora-esp32-entwicklungsboard-sx1278-mit-0.96-oled-display-v3-868mhz-915mhz

use anyhow::Result;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use esp_idf_hal::{
    delay::FreeRtos,
    gpio::{self, PinDriver},
    i2c::{I2cConfig, I2cDriver, I2C0},
    prelude::FromValueType,
};
use ssd1306::{mode::BufferedGraphicsMode, prelude::*, I2CDisplayInterface, Ssd1306};

pub struct Display {}

impl Display {
    pub fn new(
        i2c: I2C0,
        #[cfg(feature = "v2")]
        rst: gpio::Gpio16,
        #[cfg(feature = "v3")]
        rst: gpio::Gpio21,
        #[cfg(feature = "v2")]
        sda: gpio::Gpio4,
        #[cfg(feature = "v3")]
        sda: gpio::Gpio17,
        #[cfg(feature = "v2")]
        scl: gpio::Gpio15,
        #[cfg(feature = "v3")]
        scl: gpio::Gpio18,
    ) -> Result<
        Ssd1306<
            I2CInterface<I2cDriver<'static>>,
            DisplaySize128x64,
            BufferedGraphicsMode<DisplaySize128x64>,
        >,
    > {
        // important to set the pin high, otherwise the display won't work

        let mut oled_reset = PinDriver::output(rst)?;

        oled_reset.set_high()?;
        FreeRtos::delay_ms(1);
        oled_reset.set_low()?;
        FreeRtos::delay_ms(10);
        oled_reset.set_high()?;

        let config = I2cConfig::new().baudrate(400.kHz().into());
        let i2c = I2cDriver::new(i2c, sda, scl, &config)?;

        let interface = I2CDisplayInterface::new(i2c);

        let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode();

        display
            .init()
            .map_err(|e| anyhow::anyhow!("Display error: {:?}", e))?;

        Ok(display)
    }

    pub fn run(
        display: &mut Ssd1306<
            I2CInterface<I2cDriver<'static>>,
            DisplaySize128x64,
            BufferedGraphicsMode<DisplaySize128x64>,
        >,
        text: &str,
    ) -> Result<()> {
        let text_style = MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(BinaryColor::On)
            .build();

        Text::with_baseline(
            &format!("Text: {}", text),
            Point::new(0, 16),
            text_style,
            Baseline::Top,
        )
        .draw(display)
        .map_err(|e| anyhow::anyhow!("Txt2 error: {:?}", e))?;

        println!("Displaying...");

        display
            .flush()
            .map_err(|e| anyhow::anyhow!("Flush error: {:?}", e))?;

        Ok(())
    }
}
