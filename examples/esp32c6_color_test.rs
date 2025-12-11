//! RGB color cycling test for ESP32-C6
//!
//! Tests the RGB backlight by cycling through different colors.
//! Great for verifying backlight functionality on both v4 and v5 hardware.
//!
//! # Usage
//! ```bash
//! cargo run --example esp32c6_color_test --features esp32c6-examples --release
//! ```

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{clock::CpuClock, delay::Delay, i2c::master::{Config, I2c}};
use esp_println::println;
use grove_lcd_rgb::GroveLcd;

// This creates a default app-descriptor required by the esp-idf bootloader.
esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal::main]
fn main() -> ! {
    println!("\n=== RGB Color Test ===\n");

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    let delay = Delay::new();

    // Heap allocation
    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 65536);
    
    let i2c = I2c::new(peripherals.I2C0, Config::default())
        .expect("Failed to create I2C")
        .with_sda(peripherals.GPIO6)
        .with_scl(peripherals.GPIO7);
    
    println!("Initializing LCD...");
    let mut lcd = GroveLcd::new(i2c, delay);
    
    match lcd.begin(16, 2) {
        Ok(_) => println!("✓ LCD initialized!"),
        Err(e) => {
            println!("✗ Failed to initialize: {:?}", e);
            loop {}
        }
    }
    
    lcd.clear().ok();
    lcd.print("RGB Color Test").ok();
    lcd.set_cursor(0, 1).ok();
    lcd.print("Cycling...").ok();
    
    println!("Cycling through colors...\n");
    
    let colors = [
        (255, 0, 0, "Red"),
        (255, 128, 0, "Orange"),
        (255, 255, 0, "Yellow"),
        (0, 255, 0, "Green"),
        (0, 255, 255, "Cyan"),
        (0, 0, 255, "Blue"),
        (128, 0, 255, "Purple"),
        (255, 0, 255, "Magenta"),
        (255, 255, 255, "White"),
        (128, 128, 128, "Gray"),
    ];
    
    let delay = Delay::new();
    let mut index = 0;
    
    loop {
        let (r, g, b, name) = colors[index];
        println!("Setting color: {} (R:{}, G:{}, B:{})", name, r, g, b);
        lcd.set_rgb(r, g, b).ok();
        
        // Update display with color name
        lcd.set_cursor(0, 1).ok();
        lcd.print("                ").ok(); // Clear line
        lcd.set_cursor(0, 1).ok();
        lcd.print(name).ok();
        
        delay.delay_millis(1500);
        index = (index + 1) % colors.len();
    }
}