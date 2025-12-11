//! Advanced I2C diagnostic tool for ESP32-C6
//!
//! Performs comprehensive testing of the Grove LCD including:
//! - Device detection with retries
//! - LCD initialization sequence testing
//! - RGB controller testing for both v4 and v5
//! - Color cycling tests
//!
//! # Usage
//! ```bash
//! cargo run --example esp32c6_diagnostic --features esp32c6-examples --release
//! ```

#![no_std]
#![no_main]

use defmt::*;
use {esp_backtrace as _, esp_println as _};
use esp_hal::{clock::CpuClock, delay::Delay, i2c::master::{Config, I2c}};

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal::main]
fn main() -> ! {
    println!("\n╔═══════════════════════════════════════╗");
    println!("║  Grove LCD I2C Deep Diagnostic Tool   ║");
    println!("╚═══════════════════════════════════════╝\n");
    
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    let mut delay = Delay::new();

    // Heap allocation
    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 65536);
    
    println!("═══════════════════════════════════════");
    println!("Testing at default speed (100kHz)");
    println!("═══════════════════════════════════════");
    
    let mut i2c = I2c::new(peripherals.I2C0, Config::default())
        .expect("Failed to create I2C")
        .with_sda(peripherals.GPIO6)
        .with_scl(peripherals.GPIO7);
    
    let mut devices_found = 0;
    
    for addr in 0x00..=0x7F {
        if i2c.write(addr, &[]).is_ok() {
            devices_found += 1;
            println!("  Found: 0x{:02X}", addr);
            
            let device = match addr {
                0x3E => " ← LCD Text Controller",
                0x62 => " ← RGB Controller (v4)",
                0x30 => " ← RGB Controller (v5)",
                0x20 | 0x27 => " ← PCF8574 (common LCD adapter)",
                _ => "",
            };
            println!("  Found: 0x{:02X}{}", addr, device);
            
            delay.delay_millis(5);
        }
    }
    
    if devices_found == 0 {
        println!("  No devices found at this speed");
    }
    
    println!("");
    
    println!("\n╔═══════════════════════════════════════╗");
    println!("║        Detailed Device Analysis       ║");
    println!("╚═══════════════════════════════════════╝\n");
    
    // Detailed scan with retries
    println!("Scanning with 3 retry attempts per address...\n");
    
    for addr in [0x3E, 0x62, 0x30] {
        println!("Testing 0x{:02X}: ", addr);
        
        let mut success_count = 0;
        for _attempt in 1..=3 {
            delay.delay_millis(10);
            if i2c.write(addr, &[]).is_ok() {
                success_count += 1;
                println!("✓");
            } else {
                println!("✗");
            }
        }
        
        if success_count > 0 {
            println!(" → Device FOUND ({}/3 attempts)", success_count);
            
            match addr {
                0x3E => analyze_lcd_controller(&mut i2c, &mut delay),
                0x62 => analyze_rgb_controller_v4(&mut i2c, &mut delay),
                0x30 => analyze_rgb_controller_v5(&mut i2c, &mut delay),
                _ => {}
            }
        } else {
            println!(" → Not found");
        }
    }
    
    println!("\n╔═══════════════════════════════════════╗");
    println!("║            Recommendations            ║");
    println!("╚═══════════════════════════════════════╝\n");
    
    println!("If you see devices found but they don't work:");
    println!("  1. Check power supply voltage and current capability");
    println!("  2. Measure actual voltage at LCD VCC pin");
    println!("  3. Try adding 100nF capacitor across VCC and GND");
    println!("  4. Check for cold solder joints on Grove connector");
    println!("  5. Try shorter I2C wires (< 20cm)");
    println!("  6. Ensure common ground between ESP32 and LCD");
    
    loop {}
}

fn analyze_lcd_controller(i2c: &mut I2c<'_, esp_hal::Blocking>, delay: &mut Delay) {
    println!("  → Analyzing LCD Controller (0x3E)...");
    
    let mut buf = [0u8; 1];
    match i2c.read(0x3E, &mut buf) {
        Ok(_) => println!("     ✓ Can read from LCD (data: 0x{:02X})", buf[0]),
        Err(_) => println!("     ⓘ Cannot read from LCD (write-only?)"),
    }
    
    println!("     Testing initialization sequence:");
    
    let commands = [
        (0x38, "Function Set (8-bit, 2-line, 5x8)"),
        (0x0C, "Display ON, Cursor OFF"),
        (0x01, "Clear Display"),
    ];
    
    for (cmd, desc) in commands {
        delay.delay_millis(2);
        let data = [0x80, cmd];
        match i2c.write(0x3E, &data) {
            Ok(_) => println!("       ✓ {}", desc),
            Err(_) => println!("       ✗ {} FAILED", desc),
        }
        
        if cmd == 0x01 {
            delay.delay_millis(2);
        }
    }
    
    delay.delay_millis(10);
    let char_data = [0x40, b'A'];
    match i2c.write(0x3E, &char_data) {
        Ok(_) => println!("       ✓ Wrote 'A' to display"),
        Err(_) => println!("       ✗ Failed to write character"),
    }
}

fn analyze_rgb_controller_v4(i2c: &mut I2c<'_, esp_hal::Blocking>, delay: &mut Delay) {
    println!("  → Analyzing RGB Controller v4 (0x62)...");
    
    let init_sequence = [
        (0x00, 0x00, "MODE1 register"),
        (0x01, 0x00, "MODE2 register"),
        (0x08, 0xFF, "Output state"),
    ];
    
    for (reg, val, desc) in init_sequence {
        delay.delay_millis(2);
        match i2c.write(0x62, &[reg, val]) {
            Ok(_) => println!("     ✓ Set {}", desc),
            Err(_) => println!("     ✗ Failed to set {}", desc),
        }
    }
    
    println!("     Testing RGB colors:");
    
    let colors = [
        (0xFF, 0x00, 0x00, "Red"),
        (0x00, 0xFF, 0x00, "Green"),
        (0x00, 0x00, 0xFF, "Blue"),
    ];
    
    for (r, g, b, name) in colors {
        delay.delay_millis(500);
        
        let mut all_ok = true;
        if i2c.write(0x62, &[0x04, r]).is_err() { all_ok = false; }
        if i2c.write(0x62, &[0x03, g]).is_err() { all_ok = false; }
        if i2c.write(0x62, &[0x02, b]).is_err() { all_ok = false; }
        
        if all_ok {
            println!("       ✓ {} (should be visible now)", name);
        } else {
            println!("       ✗ Failed to set {}", name);
        }
    }
}

fn analyze_rgb_controller_v5(i2c: &mut I2c<'_, esp_hal::Blocking>, delay: &mut Delay) {
    println!("  → Analyzing RGB Controller v5 (0x30)...");
    
    let init_sequence = [
        (0x00, 0x07, "Reset all"),
        (0x04, 0x15, "Set PWM - all LEDs always on"),
    ];
    
    for (reg, val, desc) in init_sequence {
        delay.delay_millis(5);
        match i2c.write(0x30, &[reg, val]) {
            Ok(_) => println!("     ✓ {}", desc),
            Err(_) => println!("     ✗ Failed: {}", desc),
        }
    }
    
    delay.delay_millis(200);
    
    println!("     Testing RGB colors (v5 uses registers 0x06/0x07/0x08):");
    
    let colors = [
        (0xFF, 0x00, 0x00, "Red"),
        (0x00, 0xFF, 0x00, "Green"),
        (0x00, 0x00, 0xFF, "Blue"),
    ];
    
    for (r, g, b, name) in colors {
        delay.delay_millis(500);
        
        let mut all_ok = true;
        if i2c.write(0x30, &[0x06, r]).is_err() { all_ok = false; }
        if i2c.write(0x30, &[0x07, g]).is_err() { all_ok = false; }
        if i2c.write(0x30, &[0x08, b]).is_err() { all_ok = false; }
        
        if all_ok {
            println!("       ✓ {} (should be visible now)", name);
        } else {
            println!("       ✗ Failed to set {}", name);
        }
    }
}