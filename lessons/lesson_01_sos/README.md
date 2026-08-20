# Lesson 1: SOS Morse Code & Dual LED (C++ & Rust)

This lesson demonstrates controlling GPIO pins on the ESP32 to transmit an **SOS Morse Code signal** (`... --- ...`) synchronously on both the internal board LED (GPIO 2) and an external LED (GPIO 4), while outputting Morse characters to the serial console at 115200 baud.

---

## 🎯 Objectives
- Configure GPIO pins as outputs (`GPIO2`, `GPIO4`).
- Implement accurate timing logic for Morse code elements:
  - **Dot (`.`):** 200 ms
  - **Dash (`-`):** 600 ms
  - **Element Pause:** 200 ms
  - **Letter Pause:** 600 ms
  - **Word Pause:** 1400 ms
- Compare C++ (Arduino framework) vs Rust (`no_std`, `esp-hal`) implementations.

---

## 📁 Directory Structure

```
lessons/lesson_01_sos/
├── README.md               # Lesson documentation
├── cpp/                    # C++ (PlatformIO / Arduino)
│   ├── platformio.ini
│   ├── wokwi.toml
│   ├── diagram.json
│   └── src/main.cpp
└── rust/                   # Rust (no_std / esp-hal)
    ├── Cargo.toml
    ├── rust-toolchain.toml
    ├── build.rs
    ├── wokwi.toml
    ├── diagram.json
    └── src/main.rs
```

---

## 🚀 How to Build & Run

### C++ (PlatformIO)
1. Navigate to the C++ project folder:
   ```powershell
   cd lessons/lesson_01_sos/cpp
   ```
2. Build project using PlatformIO:
   ```powershell
   pio run
   ```
3. Run Wokwi simulation in VS Code by opening `diagram.json` and pressing `F1` -> **Wokwi: Start Simulator**.

---

### Rust (`esp-hal`)
1. Navigate to the Rust project folder:
   ```powershell
   cd lessons/lesson_01_sos/rust
   ```
2. Source the Espressif toolchain environment (if on Windows):
   ```powershell
   . C:\Users\vitof\export-esp.ps1
   ```
3. Build the release binary:
   ```powershell
   cargo build --release
   ```
4. Run Wokwi simulation in VS Code by opening `rust/diagram.json` and starting Wokwi Simulator, or flash to hardware:
   ```powershell
   cargo espflash flash --release --monitor
   ```
