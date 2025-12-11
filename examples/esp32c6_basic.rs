//! Basic "Hello World" example for ESP32-C6
//!
//! Displays a simple message on the LCD with a green backlight.
//!
//! # Wiring
//! - SDA: GPIO6
//! - SCL: GPIO7
//! - VCC: 3.3V or 5V
//! - GND: GND
//!
//! # Usage
//! ```bash
//! cargo run --example esp32c6_basic --release
//! ```


#![no_std]
#![no_main]

use {esp_backtrace as _, esp_println as _};
use esp_hal::{clock::CpuClock, delay::Delay, i2c::master::{Config, I2c}};
use grove_lcd_rgb::GroveLcd;

// This creates a default app-descriptor required by the esp-idf bootloader.
esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal::main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    let delay = Delay::new();
    
    // Heap allocation
    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 65536);
    
    // Create I2C on GPIO6 (SDA) and GPIO7 (SCL)
    let i2c = I2c::new(peripherals.I2C0, Config::default()).unwrap()
        .with_sda(peripherals.GPIO6)
        .with_scl(peripherals.GPIO7);
    
    // Create and initialize LCD
    let mut lcd = GroveLcd::new(i2c, delay);
    lcd.begin(16, 2).unwrap();
    
    // Set green backlight
    lcd.set_rgb(0, 255, 0).unwrap();
    
    // Display message
    lcd.set_cursor(0, 0).ok();
    lcd.print("Hello, World!").unwrap();
    
    lcd.set_cursor(0, 1).ok();
    lcd.print("Grove LCD Rust").unwrap();
    
    loop {}
}