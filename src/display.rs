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
    gpio::{InputPin, Output, OutputPin, Pin, PinDriver},
    i2c::{I2c, I2cConfig, I2cDriver},
    peripheral::Peripheral,
    prelude::FromValueType,
};
use ssd1306::{
    mode::{BufferedGraphicsMode, DisplayConfig},
    prelude::I2CInterface,
    rotation::DisplayRotation,
    size::DisplaySize128x64,
    I2CDisplayInterface, Ssd1306,
};

pub struct Display<'a, T: OutputPin>
where
    T: OutputPin,
{
    display: Ssd1306<
        I2CInterface<I2cDriver<'a>>,
        DisplaySize128x64,
        BufferedGraphicsMode<DisplaySize128x64>,
    >,
    reset: PinDriver<'a, T, Output>,
}

pub type DisplayType = Ssd1306<
    I2CInterface<I2cDriver<'static>>,
    DisplaySize128x64,
    BufferedGraphicsMode<DisplaySize128x64>,
>;

impl<'a, T: Pin> Display<'a, T>
where
    T: OutputPin,
{
    pub fn new<I2C: I2c>(
        i2c: impl Peripheral<P = I2C> + 'a,
        rst: impl Peripheral<P = T> + 'a,
        sda: impl Peripheral<P = impl InputPin + OutputPin> + 'a,
        scl: impl Peripheral<P = impl InputPin + OutputPin> + 'a,
    ) -> Result<Display<'a, T>> {
        // lifetime is important here
        // or the display will turn off
        let reset: PinDriver<'a, T, Output> = PinDriver::output(rst).unwrap();

        let config = I2cConfig::new().baudrate(400.kHz().into());

        let i2c_driver: I2cDriver<'a> = I2cDriver::new(i2c, sda, scl, &config).unwrap();
        let interface: I2CInterface<I2cDriver<'a>> = I2CDisplayInterface::new(i2c_driver);

        let display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode();

        Ok(Self { display, reset })
    }

    pub fn init(&mut self) -> Result<()> {
        self.reset.set_high().unwrap();
        self.display
            .init()
            .map_err(|e| anyhow::anyhow!("Display error: {:?}", e))?;
        Ok(())
    }

    pub fn reset(&mut self) -> Result<()> {
        self.reset.set_high().unwrap();
        FreeRtos::delay_ms(1);
        self.reset.set_low().unwrap();
        FreeRtos::delay_ms(10);
        self.reset.set_high().unwrap();
        self.init()?;
        Ok(())
    }

    pub fn print(&mut self, text: &str) -> Result<()> {
        self.display.clear_buffer();
        let text_style = MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(BinaryColor::On)
            .build();

        Text::with_baseline(text, Point::new(0, 16), text_style, Baseline::Top)
            .draw(&mut self.display)
            .map_err(|e| anyhow::anyhow!("Txt2 error: {:?}", e))?;

        self.display
            .flush()
            .map_err(|e| anyhow::anyhow!("Flush error: {:?}", e))?;

        Ok(())
    }
}
