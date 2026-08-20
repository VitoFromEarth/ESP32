# ESP32 Lessons & Examples (C++ & Rust)

Welcome to the ESP32 firmware development curriculum. This repository contains step-by-step hands-on lessons written in both **C++ (PlatformIO / Arduino)** and **Rust (`no_std` / `esp-hal`)** for the **ESP32 Dev Module**.

---

## 📚 Repository Structure & Lessons Index

Each lesson is self-contained in the `lessons/` directory, containing dedicated subfolders for C++ (`cpp/`) and Rust (`rust/`), along with full Wokwi simulator setup (`diagram.json`, `wokwi.toml`) and documentation.

```
ESP32/
├── lessons/
│   └── lesson_01_sos/           # Lesson 1: SOS Morse Code & Dual LED
│       ├── README.md            # Lesson details and explanation
│       ├── cpp/                 # PlatformIO C++ implementation
│       └── rust/                # esp-hal Rust implementation
│
└── README.md                    # Curriculum Overview & Quick Start
```

### Curriculum Roadmap

| # | Lesson Name | Description | C++ Setup | Rust Setup |
|---|---|---|---|---|
| 01 | [Lesson 01: SOS Morse Code](lessons/lesson_01_sos/README.md) | Blinks onboard & external LEDs to send SOS in Morse code via GPIO2/GPIO4 with serial output | [`cpp/`](lessons/lesson_01_sos/cpp) | [`rust/`](lessons/lesson_01_sos/rust) |

---

## 🛠️ Prerequisites & Setup

### Requirements:
1. **PlatformIO** (for C++ development)
2. **Rust & Xtensa Toolchain** (for Rust development):
   - `espup` installer
   - `cargo-espflash`
   - Xtensa toolchain (`esp` channel)

### Setting Environment Variables (Rust)
In PowerShell, source the toolchain variables prior to building Rust projects:
```powershell
. C:\Users\vitof\export-esp.ps1
```

---

## 💻 Running a Lesson

### Running C++ (PlatformIO)
```powershell
cd lessons/lesson_01_sos/cpp
pio run
```
To run Wokwi simulator in VS Code:
1. Open `lessons/lesson_01_sos/cpp/diagram.json`.
2. Press `Ctrl+Shift+P` (or `F1`) and select **Wokwi: Start Simulator**.

---

### Running Rust (`esp-hal`)
```powershell
cd lessons/lesson_01_sos/rust
. C:\Users\vitof\export-esp.ps1
cargo build --release
```
To run Wokwi simulator in VS Code:
1. Open `lessons/lesson_01_sos/rust/diagram.json`.
2. Press `Ctrl+Shift+P` (or `F1`) and select **Wokwi: Start Simulator**.

To flash directly to hardware:
```powershell
cargo espflash flash --release --monitor
```

---

## ➕ Adding a New Lesson

To add a new lesson (e.g., `lesson_02_button`):
1. Create a new directory under `lessons/` (e.g. `lessons/lesson_02_button/`).
2. Add `cpp/` with `platformio.ini`, `wokwi.toml`, `diagram.json`, and `src/main.cpp`.
3. Add `rust/` with `Cargo.toml`, `rust-toolchain.toml`, `build.rs`, `.cargo/config.toml`, `wokwi.toml`, `diagram.json`, and `src/main.rs`.
4. Add a `README.md` inside the lesson folder explaining the objective.
5. Update the table in the main [`README.md`](file:///c:/Users/vitof/Documents/PlatformIO/Projects/ESP32/README.md).
