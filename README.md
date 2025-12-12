# Grove LCD RGB Backlight - Rust Driver

[![Crates.io](https://img.shields.io/crates/v/grove-lcd-rgb.svg)](https://crates.io/crates/grove-lcd-rgb)
[![Documentation](https://docs.rs/grove-lcd-rgb/badge.svg)](https://docs.rs/grove-lcd-rgb)
[![License](https://img.shields.io/crates/l/grove-lcd-rgb.svg)](https://github.com/maththedude/grove-lcd-rgb#license)

A platform-agnostic Rust driver for the Grove LCD RGB Backlight display using `embedded-hal` traits. Provides full control over the 16x2 character LCD with RGB backlight via I2C.

## Features

- ✅ Full LCD control (clear, home, cursor positioning)
- ✅ RGB backlight color control (16.7 million colors)
- ✅ Custom character support (up to 8 custom chars)
- ✅ Display scrolling and text direction control
- ✅ Cursor visibility and blink control
- ✅ Automatic v4/v5 hardware detection
- ✅ Platform-agnostic using `embedded-hal` 1.0
- ✅ `no_std` compatible
- ✅ Comprehensive examples for ESP32-C6

## Hardware Support

The Grove LCD RGB Backlight uses two I2C devices:
- **LCD Controller**: Address 0x3E (HD44780-compatible)
- **RGB Controller v4**: Address 0x62 (PCA9632)
- **RGB Controller v5**: Address 0x30 (SGM31323)

The driver automatically detects which version you have and uses the appropriate configuration.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
grove-lcd-rgb = { git = "https://github.com/maththedude/grove-lcd-rgb" }
embedded-hal = "1.0"
```

Or from crates.io (once published):

```toml
[dependencies]
grove-lcd-rgb = "0.1"
```

## Quick Start

```rust
use grove_lcd_rgb::GroveLcd;

// Create LCD instance with your I2C peripheral and delay provider
let mut lcd = GroveLcd::new(i2c, delay);

// Initialize LCD (16 columns, 2 rows)
lcd.begin(16, 2)?;

// Set backlight color to green
lcd.set_rgb(0, 255, 0)?;

// Display text
lcd.print("Hello, World!")?;

// Set cursor to second line
lcd.set_cursor(0, 1)?;
lcd.print("Grove LCD")?;
```

## ESP32-C6 Example

```rust
#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{delay::Delay, i2c::master::{Config, I2c}};
use grove_lcd_rgb::GroveLcd;

esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 65536);

#[esp_hal::main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    
    let i2c = I2c::new(peripherals.I2C0, Config::default())
        .unwrap()
        .with_sda(peripherals.GPIO6)
        .with_scl(peripherals.GPIO7);
    
    let delay = Delay::new();
    let mut lcd = GroveLcd::new(i2c, delay);
    
    lcd.begin(16, 2).unwrap();
    lcd.set_rgb(0, 255, 0).unwrap();
    lcd.print("Hello, ESP32!").unwrap();
    
    loop {}
}
```

## Examples

The crate includes several examples for ESP32-C6. To run them:

```bash
# Basic "Hello World" example
cargo run --example esp32c6_basic --release

# I2C device scanner
cargo run --example esp32c6_i2c_scan --release

# Color cycling demo
cargo run --example esp32c6_color_test --release

# Counter with color changes
cargo run --example esp32c6_counter --release

# Full thermostat example
cargo run --example esp32c6_thermostat --release

# Advanced diagnostic tool
cargo run --example esp32c6_diagnostic --release
```

## API Overview

### Initialization

```rust
lcd.begin(cols: u8, rows: u8) -> Result<(), LcdError<E>>
```

### Display Control

```rust
lcd.clear()                               // Clear display
lcd.home()                                // Return cursor to (0,0)
lcd.set_cursor(col: u8, row: u8)         // Position cursor
lcd.display()                             // Turn display on
lcd.no_display()                          // Turn display off
```

### Backlight Control

```rust
lcd.set_rgb(r: u8, g: u8, b: u8)         // Set RGB color
lcd.backlight_white()                     // Set to white
lcd.backlight_off()                       // Turn off backlight
```

### Text Operations

```rust
lcd.print(s: &str)                        // Print string
lcd.write(value: u8)                      // Write single byte
```

### Cursor Control

```rust
lcd.cursor()                              // Show cursor
lcd.no_cursor()                           // Hide cursor
lcd.blink()                               // Blink cursor
lcd.no_blink()                            // Stop blinking
```

### Advanced Features

```rust
lcd.scroll_display_left()                // Scroll content left
lcd.scroll_display_right()               // Scroll content right
lcd.create_char(location, charmap)       // Create custom character
lcd.left_to_right()                      // Text flows left to right
lcd.right_to_left()                      // Text flows right to left
```

## Hardware Differences: v4 vs v5

| Feature | v4.0 | v5.0 |
|---------|------|------|
| RGB Chip | PCA9632 | SGM31323 |
| RGB I2C Address | 0x62 | 0x30 |
| RGB Registers | R:0x04, G:0x03, B:0x02 | R:0x06, G:0x07, B:0x08 |
| Voltage | 5V only | 3.3V/5V compatible |

**The driver automatically detects and handles both versions!**

## Wiring for ESP32-C6

```
Grove LCD          ESP32-C6
---------          --------
GND                GND
VCC                5V (or 3.3V for v5)
SDA                GPIO6 (configurable)
SCL                GPIO7 (configurable)
```

## Troubleshooting

### Display not working?

1. Run the I2C scanner example to verify devices are detected
2. Check power supply (3.3V or 5V)
3. Verify SDA/SCL connections
4. Ensure pull-up resistors are present (usually built into Grove modules)

### Colors not working on v5?

The driver should auto-detect v5 hardware. If colors don't work:
- Verify the RGB controller is detected at address 0x30
- Check that you're using the latest version of this crate
- Try the diagnostic example for detailed testing

### Need help?

- Check the [examples](examples/) directory
- Read the [API documentation](https://docs.rs/grove-lcd-rgb)
- Open an issue on [GitHub](https://github.com/maththedude/grove-lcd-rgb/issues)

## Using in Your Project

Add to your `Cargo.toml`:

```toml
[dependencies]
grove-lcd-rgb = { git = "https://github.com/maththedude/grove-lcd-rgb" }
# Or after publishing:
# grove-lcd-rgb = "0.1"
```

Then in your code:

```rust
use grove_lcd_rgb::GroveLcd;

let mut lcd = GroveLcd::new(i2c, delay);
lcd.begin(16, 2)?;
lcd.print("Working!")?;
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.


## Credits

Based on the [Seeed Studio Grove LCD RGB Backlight Arduino library](https://github.com/Seeed-Studio/Grove_LCD_RGB_Backlight).