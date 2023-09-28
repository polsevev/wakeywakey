use anyhow::Result;
use embedded_graphics::image::Image;
use embedded_graphics::image::ImageRaw;
use embedded_graphics::image::ImageRawLE;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    text::Text
};
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::Gpio0;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::prelude::Peripherals;
use esp_idf_hal::prelude::*;
use esp_idf_hal::spi;


use st7735_lcd;
use st7735_lcd::Orientation;

fn main() -> Result<()> {
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

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
    display.set_offset(0, 25);

    let image_raw: ImageRawLE<Rgb565> =
        ImageRaw::new(include_bytes!("../ferris.raw"), 86);
    let image = Image::new(&image_raw, Point::new(26, 8));
   

    
// Create a new character style
    let style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);

// Create a text at position (20, 30) and draw it using the previously defined style


    println!("lcd test have done.");
    let mut led_blink = PinDriver::output(peripherals.pins.gpio15).unwrap();

    loop {
        led_blink.set_high().unwrap();
        // we are sleeping here to make sure the watchdog isn't triggered
        image.draw(&mut display).unwrap();
        FreeRtos::delay_ms(1000);

        Text::new("Hello Rust!", Point::new(20, 30), style).draw(&mut display).unwrap();
        led_blink.set_low().unwrap();
        FreeRtos::delay_ms(1000);
    }
}
