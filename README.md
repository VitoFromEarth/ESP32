# ESP32 LED Blink & Serial (C++ and Rust)

This repository contains both **C++ (PlatformIO / Arduino)** and **Rust (`no_std` / `esp-hal`)** implementations of the same firmware for the **ESP32 Dev Module**:

- **Serial Output:** Prints `"Hello World"` at `115200` baud.
- **Internal LED:** Blinks onboard LED on **GPIO 2** every 100 ms.
- **External LED:** Blinks external LED on **GPIO 4** every 100 ms.
- **Simulation:** Full Wokwi simulator setup included for both versions.

---

## Project Structure

```
ESP32/
├── diagram.json                 # Wokwi circuit diagram (C++ version)
├── platformio.ini               # PlatformIO configuration (C++ version)
├── wokwi.toml                   # Wokwi simulation config (C++ version)
├── src/
│   └── main.cpp                 # C++ implementation
│
├── esp32_rust/                  # Rust project directory
│   ├── .cargo/
│   │   └── config.toml          # Cargo build flags & Xtensa target settings
│   ├── Cargo.toml               # Rust dependencies (esp-hal, esp-backtrace, etc.)
│   ├── diagram.json             # Wokwi circuit diagram (Rust version)
│   ├── wokwi.toml               # Wokwi simulation config pointing to Rust ELF
│   └── src/
│       └── main.rs              # Rust no_std implementation
│
└── README.md                    # This documentation
```

---

## 1. Prerequisites & Required Tools

Because the classic ESP32 uses the **Xtensa** architecture (not RISC-V), specialized tools and a toolchain are required.

### What is already installed on this machine:
1. **Rust & Cargo** (`rustup`)
2. **`espup`** (`v0.17.1`) - The official Espressif toolchain installer.
3. **`cargo-espflash`** (`v4.5.0`) - Tool to flash and monitor ESP32 over serial.
4. **`cargo-generate`** (`v0.23.14`) - Project template generator.
5. **Xtensa Toolchain (`esp`)** - Installed via `espup` into `rustup`.

### Installing from scratch on a new machine:
If setting up on another computer, run:
```powershell
# 1. Install cargo helper tools
cargo install espup cargo-generate espflash

# 2. Install the Xtensa toolchain for ESP32
espup install -d x86_64-pc-windows-msvc -t esp32
```

---

## 2. Setting Environment Variables

Every new PowerShell terminal session requires the Espressif LLVM and Clang paths in its environment.

Run the generated export script:
```powershell
. C:\Users\vitof\export-esp.ps1
```

*(This sets `$Env:LIBCLANG_PATH` and adds the Xtensa toolchain binaries to your `$Env:PATH`.)*

---

## 3. Building & Running the Rust Version

### Step A: Build the Project
Open a terminal in the `esp32_rust` directory:
```powershell
cd esp32_rust
. C:\Users\vitof\export-esp.ps1
cargo build --release
```

The compiled binary will be generated at:
```
esp32_rust/target/xtensa-esp32-none-elf/release/esp32_rust
```

---

### Step B: Run in Wokwi Simulator
The Rust project includes [wokwi.toml](file:///c:/Users/vitof/Documents/PlatformIO/Projects/ESP32/esp32_rust/wokwi.toml) configured with:
```toml
[wokwi]
version = 1
elf = "target/xtensa-esp32-none-elf/release/esp32_rust"
```

To simulate:
1. Open [esp32_rust/diagram.json](file:///c:/Users/vitof/Documents/PlatformIO/Projects/ESP32/esp32_rust/diagram.json) in VS Code.
2. Press `F1` (or `Ctrl+Shift+P`), type **`Wokwi: Start Simulator`**, and press Enter.
3. You will see both the onboard and external LEDs blinking in sync, with `"Hello World"` printing in the serial monitor.

---

### Step C: Flash to Physical Hardware
If you connect an ESP32 board via USB:
```powershell
cd esp32_rust
. C:\Users\vitof\export-esp.ps1
cargo espflash flash --release --monitor
```

---

## 4. Code Comparison

### C++ (`src/main.cpp`)
```cpp
#include <Arduino.h>

#define LED 2
#define EXT_LED 4

void setup() {
  Serial.begin(115200);
  pinMode(LED, OUTPUT);
  pinMode(EXT_LED, OUTPUT);
}

void loop() {
  Serial.println("Hello World");
  digitalWrite(LED, HIGH);
  digitalWrite(EXT_LED, HIGH);
  delay(100);
  digitalWrite(LED, LOW);
  digitalWrite(EXT_LED, LOW);
  delay(100);
}
```

### Rust (`esp32_rust/src/main.rs`)
```rust
#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Level, Output, OutputConfig},
    main,
    time::Duration,
};
use esp_println::println;

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    // Configure GPIO 2 (Internal LED) and GPIO 4 (External LED)
    let mut led = Output::new(peripherals.GPIO2, Level::Low, OutputConfig::default());
    let mut ext_led = Output::new(peripherals.GPIO4, Level::Low, OutputConfig::default());

    loop {
        println!("Hello World");

        led.set_high();
        ext_led.set_high();
        delay.delay(Duration::from_millis(100));

        led.set_low();
        ext_led.set_low();
        delay.delay(Duration::from_millis(100));
    }
}
```
