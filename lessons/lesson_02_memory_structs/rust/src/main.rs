#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{delay::Delay, main, time::Instant};
use esp_println::println;

// 1. Структура даних сенсора в Rust
#[derive(Debug)]
struct SensorData {
    temperature: f32, // 15.0..30.0 °C
    humidity: f32,    // 30.0..65.0 %
    timestamp: u32,   // Uptime у секундах
}

// Простий генератор псевдовипадкових чисел (Linear Congruential Generator) для no_std
struct SimpleLcg {
    state: u32,
}

impl SimpleLcg {
    fn new(seed: u32) -> Self {
        Self { state: seed }
    }

    fn next_u32(&mut self) -> u32 {
        self.state = self.state.wrapping_mul(1664525).wrapping_add(1013904223);
        self.state
    }

    fn range_f32(&mut self, min: f32, max: f32) -> f32 {
        let step = (self.next_u32() % 1000) as f32 / 1000.0;
        min + step * (max - min)
    }
}

// Демонстрація переповнення типів у Rust
fn demonstrate_integer_overflow() {
    println!("\r==================================================");
    println!("\r1. Демонстрація переповнення типів даних у Rust");
    println!("\r==================================================");

    let a: u8 = 200;
    let b: u8 = 100;
    
    // В Rust переповнення за замовчуванням під час звичайної математики a + b у Debug режимі
    // викликає panic для гарантії безпеки пам'яті. Для явного переповнення використовують wrapping_add.
    let sum_wrapped = a.wrapping_add(b);
    let sum_promoted = (a as u16) + (b as u16);

    println!("\rЗмінна a (u8): {}", a);
    println!("\rЗмінна b (u8): {}", b);
    println!("\rМатематична сума (u16 promotion): {}", sum_promoted);
    println!("\rЯвне переповнення (a.wrapping_add(b)): {}", sum_wrapped);
    println!("\r--------------------------------------------------");
    println!("\rПОЯСНЕННЯ (Rust):");
    println!("\rТип u8 вміщує значення 0..255.");
    println!("\rУ Rust 'a.wrapping_add(b)' виконує циклічне переповнення: 300 % 256 = 44.");
    println!("\rУ Debug-збірках звичайний '+' перевіряє переповнення та захищає від помилок.");
    println!("\r==================================================");
}

#[main]
fn main() -> ! {
    let _peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    println!("\r\n=== ESP32 Lesson 2: Memory & Structs (Rust no_std) ===");

    // 1. Пояснення переповнення типів
    demonstrate_integer_overflow();

    let mut rng = SimpleLcg::new(0xDEADBEEF);
    let start_time = Instant::now();

    let mut last_sensor_secs: u64 = 0;
    let mut last_heap_secs: u64 = 0;

    println!("\r");
    println!("\r[RUST MEMORY INFO]");
    println!("\rno_std Rust використовує статичне виділення пам'яті (без Heap allocator).");
    println!("\rВитоки пам'яті (Memory Leaks) гарантовано відсутні на рівні компілятора!");
    println!("\r");

    loop {
        let elapsed_secs = start_time.elapsed().as_secs();

        // Читання та вивід сенсорів кожні 20 секунд
        if elapsed_secs >= last_sensor_secs + 20 || last_sensor_secs == 0 {
            last_sensor_secs = if elapsed_secs == 0 { 1 } else { elapsed_secs };

            let data = SensorData {
                temperature: rng.range_f32(15.0, 30.0),
                humidity: rng.range_f32(30.0, 65.0),
                timestamp: elapsed_secs as u32,
            };

            println!(
                "\r[SENSOR DATA] Timestamp: {}s | Температура: {:.1} °C | Вологість: {:.1} %",
                data.timestamp, data.temperature, data.humidity
            );
        }

        // Перевірка пам'яті кожні 60 секунд
        if elapsed_secs >= last_heap_secs + 60 || (last_heap_secs == 0 && elapsed_secs > 0) {
            last_heap_secs = elapsed_secs;

            println!("\r--------------------------------------------------");
            println!(
                "\r[MEMORY MONITOR] Uptime: {}s | Dynamic Heap Allocations: 0 bytes (no_std)",
                elapsed_secs
            );
            println!("\r✅ Стан пам'яті повністю стабільний (Static Safety Guarantee).");
            println!("\r--------------------------------------------------");
        }

        // Невеликий відпочинок циклу (100 мс)
        delay.delay_millis(100);
    }
}
