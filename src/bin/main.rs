#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use embedded_graphics::image::Image;
use embedded_graphics::mono_font::iso_8859_10::FONT_6X10;
use embedded_graphics::prelude::Point;
use embedded_graphics::{
    mono_font::{iso_8859_3::FONT_10X20, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::i2c::master::{Config, I2c};
use esp_hal::main;
use esp_hal::time::{Duration, Instant};
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
use tinybmp::Bmp;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    // generator version: 0.5.0

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // on-board oled init
    let i2c = I2c::new(peripherals.I2C0, Config::default())
        .unwrap()
        .with_scl(peripherals.GPIO6)
        .with_sda(peripherals.GPIO5);
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize72x40, DisplayRotation::Rotate180)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    // define text styles

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_10X20)
        .text_color(BinaryColor::On)
        .build();
    let text_style2 = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    // display bmp
    const LOGO_DATA: &[u8] = include_bytes!("../../assets/logo-blk.bmp");
    const FERRIS_DATA: &[u8] = include_bytes!("../../assets/ferris-w.bmp");

    loop {
        // display text
        let _ = display.clear(BinaryColor::Off);

        Text::with_baseline("powered", Point::zero(), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();
        Text::with_baseline("  by", Point { x: 5, y: 20 }, text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();

        let _ = display.flush();

        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(3000) {}

        // display image
        let _ = display.clear(BinaryColor::On);

        // 1. Parse the embedded BMP data
        let bmp = Bmp::from_slice(LOGO_DATA).unwrap();

        // 2. Create the Image object (start drawing at top-left corner)
        let image = Image::new(&bmp, Point { x: 16, y: 0 });

        // 3. Draw the image to the display buffer
        image.draw(&mut display).unwrap();

        let _ = display.flush();

        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(5000) {}

        // display text2
        let _ = display.clear(BinaryColor::Off);

        Text::with_baseline("say \"Hello\"", Point { x: 0, y: 5 }, text_style2, Baseline::Top)
            .draw(&mut display)
            .unwrap();
        Text::with_baseline(
            "to Ferris!",
            Point { x: 0, y: 20 },
            text_style2,
            Baseline::Top,
        )
        .draw(&mut display)
        .unwrap();

        let _ = display.flush();

        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(3000) {}

        // display image
        let _ = display.clear(BinaryColor::Off);

        // 1. Parse the embedded BMP data
        let bmp = Bmp::from_slice(FERRIS_DATA).unwrap();

        // 2. Create the Image object (start drawing at top-left corner)
        let image = Image::new(&bmp, Point { x: 6, y: 0 });

        // 3. Draw the image to the display buffer
        image.draw(&mut display).unwrap();

        let _ = display.flush();

        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(5000) {}
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-rc.0/examples/src/bin
}
