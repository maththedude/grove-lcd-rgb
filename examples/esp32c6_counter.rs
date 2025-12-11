//! Counter example with color cycling for ESP32-C6
//!
//! Displays a counter that increments every 100ms and cycles through
//! different backlight colors.
//!
//! # Usage
//! ```bash
//! cargo run --example esp32c6_counter --release
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
    
    let i2c = I2c::new(peripherals.I2C0, Config::default())
        .unwrap()
        .with_sda(peripherals.GPIO6)
        .with_scl(peripherals.GPIO7);
    
    let mut lcd = GroveLcd::new(i2c, delay);
    
    lcd.begin(16, 2).unwrap();
    lcd.set_cursor(0, 0).unwrap();
    lcd.print("Counter:").unwrap();
    
    let mut counter = 0u32;
    
    loop {
        // Update counter on second line
        lcd.set_cursor(0, 1).unwrap();
        
        // Format counter (simple no_std approach)
        let mut buffer = [0u8; 16];
        let mut pos = 0;
        let mut n = counter;
        
        if n == 0 {
            buffer[pos] = b'0';
            pos += 1;
        } else {
            let mut divisor = 1_000_000_000;
            let mut started = false;
            
            while divisor > 0 {
                let digit = (n / divisor) as u8;
                if digit > 0 || started {
                    buffer[pos] = b'0' + digit;
                    pos += 1;
                    started = true;
                }
                n %= divisor;
                divisor /= 10;
            }
        }
        
        // Print counter
        for i in 0..pos {
            lcd.write(buffer[i]).unwrap();
        }
        
        // Clear rest of line
        for _ in pos..16 {
            lcd.write(b' ').unwrap();
        }
        
        // Change color every 10 counts
        let phase = (counter / 10) % 6;
        match phase {
            0 => lcd.set_rgb(255, 0, 0).unwrap(),     // Red
            1 => lcd.set_rgb(255, 255, 0).unwrap(),   // Yellow
            2 => lcd.set_rgb(0, 255, 0).unwrap(),     // Green
            3 => lcd.set_rgb(0, 255, 255).unwrap(),   // Cyan
            4 => lcd.set_rgb(0, 0, 255).unwrap(),     // Blue
            5 => lcd.set_rgb(255, 0, 255).unwrap(),   // Magenta
            _ => {}
        }
        
        counter += 1;
        delay.delay_millis(100);
    }
}