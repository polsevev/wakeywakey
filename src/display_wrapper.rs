use embedded_graphics::{pixelcolor::Rgb565, prelude::RgbColor};
use esp_idf_hal::{prelude::Peripherals, gpio::{Gpio0, PinDriver}, delay::FreeRtos};
use st7735_lcd::{Orientation, ST7735};

use esp_idf_hal::prelude::*;



pub struct DisplayWrapper<A,B,C>{
    display: ST7735<A,B,C>

}

impl DisplayWrapper<A,B,C>{

    pub fn new() -> anyhow::Result<DisplayWrapper<A,B,C>>{
        let peripherals = Peripherals::take().unwrap();             
        let spi = peripherals.spi2;
        let sclk = peripherals.pins.gpio6;
        let sdo = peripherals.pins.gpio7;
        let sdi = Option::<Gpio0>::None;
        let cs = peripherals.pins.gpio10;
        let driver_config = Default::default();
        let spi_config = spi::SpiConfig::new().baudrate(16.MHz().into()).data_mode(embedded_hal::spi::MODE_3);
        let spi =
            spi::SpiDeviceDriver::new_single(spi, sclk, sdo, sdi, Some(cs), &driver_config, &spi_config)?;

        let rst = PinDriver::output(peripherals.pins.gpio3)?;
        let dc = PinDriver::output(peripherals.pins.gpio4)?;

        let rgb = true;
        let inverted = false;
        let width = 128;
        let height = 160;

        let mut delay = FreeRtos;

        let mut display = st7735_lcd::ST7735::new(spi, dc, rst, rgb, inverted, width, height);

        display.init(&mut delay).unwrap();
        display.clear(Rgb565::BLACK).unwrap();
        display
            .set_orientation(&Orientation::LandscapeSwapped)
            .unwrap();
        display.set_offset(0, 0);

        Ok(DisplayWrapper{display})
    }
}
