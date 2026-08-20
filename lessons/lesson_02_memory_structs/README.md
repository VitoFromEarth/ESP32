# Lesson 2: Embedded Memory, Data Structures & Data Types (C++ & Rust)

Урок присвячений організації даних у структури, неблокуючому виконанню задач, моніторингу вільної пам'яті (Heap) та аналізу переповнення типів даних у C++ та Rust (`no_std`).

---

## Завдання

1. **Організація даних сенсора:** створення структури `SensorData` (температура, вологість, timestamp).
2. **Генерація випадкових значень:** температура (15..30 °C), вологість (30..65 %), timestamp (uptime в секундах).
3. **Неблокуючий вивід:** логування даних кожні 20 секунд без використання блокуючого `delay()`.
4. **Моніторинг пам'яті:** вивід `ESP.getFreeHeap()` кожні 60 секунд (C++) та демонстрація статичного виділення (Rust `no_std`).
5. **Теорія:** порівняльний аналіз integer overflow та областей пам'яті у [`MEMORY_AND_TYPES.md`](MEMORY_AND_TYPES.md).

---

## 📁 Структура проєкту

```
lessons/lesson_02_memory_structs/
├── README.md               # Опис уроку
├── MEMORY_AND_TYPES.md     # Аналіз пам'яті та переповнення типів
├── cpp/                    # PlatformIO C++ проєкт
│   ├── platformio.ini
│   ├── wokwi.toml
│   ├── diagram.json
│   └── src/main.cpp
└── rust/                   # Rust (no_std / esp-hal) проєкт
    ├── Cargo.toml
    ├── rust-toolchain.toml
    ├── build.rs
    ├── wokwi.toml
    ├── diagram.json
    └── src/main.rs
```

---

## 🚀 Компіляція та запуск

### C++ (PlatformIO)
```powershell
cd lessons/lesson_02_memory_structs/cpp
pio run
```
Для симуляції: відкрити `cpp/diagram.json` -> `F1` -> **Wokwi: Start Simulator**.

### Rust (`esp-hal`)
```powershell
cd lessons/lesson_02_memory_structs/rust
. C:\Users\vitof\export-esp.ps1
cargo build --release
```
Для симуляції: відкрити `rust/diagram.json` -> `F1` -> **Wokwi: Start Simulator**.

---

## Чеклист готовності

- [x] Вивід даних сенсора кожні 20 секунд (C++ та Rust)
- [x] Рандомізовані значення температури та вологості
- [x] Щохвилинний моніторинг стабільності вільної пам'яті
- [x] Пояснення overflow та областей пам'яті у [`MEMORY_AND_TYPES.md`](MEMORY_AND_TYPES.md)
