//! I2C bus scanner for ESP32-C6
//!
//! Scans the I2C bus and reports all devices found.
//! Useful for verifying LCD connections.
//!
//! # Usage
//! ```bash
//! cargo run --example esp32c6_i2c_scan --features esp32c6-examples --release
//! ```

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{clock::CpuClock, delay::Delay, i2c::master::{Config, I2c}};
use esp_println::println;

// This creates a default app-descriptor required by the esp-idf bootloader.
esp_bootloader_esp_idf::esp_app_desc!();


#[esp_hal::main]
fn main() -> ! {
    println!("\n=== I2C Scanner ===\n");
    
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    let delay = Delay::new();

   // Heap allocation
    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 65536);
    
    let mut i2c = I2c::new(peripherals.I2C0, Config::default())
        .expect("Failed to create I2C")
        .with_sda(peripherals.GPIO6)
        .with_scl(peripherals.GPIO7);
    
    println!("Scanning I2C bus on GPIO6 (SDA) and GPIO7 (SCL)...\n");
    
    let mut found = 0;
    
    for addr in 0x00..=0x7F {
        // Small delay before each attempt to let I2C bus settle
        delay.delay_millis(10);
        
        match i2c.write(addr, &[]) {
            Ok(_) => {
                println!("  Found device at: 0x{:02X}", addr);
                
                match addr {
                    0x3E => println!("    ^ LCD Text Controller"),
                    0x62 => println!("    ^ RGB Backlight (v4)"),
                    0x30 => println!("    ^ RGB Backlight (v5)"),
                    _ => {}
                }
                
                found += 1;
            }
            Err(_) => {
                // Address not responding - this is normal
            }
        }
    }
    
    println!("\nScan complete. Found {} device(s).", found);
    
    if found == 0 {
        println!("\nNo devices found. Check:");
        println!("  - Wiring (SDA/SCL)");
        println!("  - Power (VCC/GND)");
        println!("  - Pull-up resistors");
    }
    
    loop {}
}