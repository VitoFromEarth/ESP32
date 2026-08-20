#include <Arduino.h>

// 1. Структура даних сенсора
struct SensorData {
  float temperature;   // Температура у °C (15.0 - 30.0)
  float humidity;      // Вологість у % (30.0 - 65.0)
  uint32_t timestamp;  // Час роботи системи в секундах (uptime)
};

// Періодичні інтервали для задач (в мілісекундах)
const unsigned long SENSOR_READ_INTERVAL = 20000;  // 20 секунд
const unsigned long HEAP_MONITOR_INTERVAL = 60000; // 60 секунд (1 хвилина)

// Змінні таймерів для неблокуючого виконання (non-blocking timing)
unsigned long lastSensorReadTime = 0;
unsigned long lastHeapMonitorTime = 0;

// Змінна для відстеження початкового значення вільної пам'яті
uint32_t initialFreeHeap = 0;

// Функція для генерації випадкових даних сенсора
SensorData readSensorData() {
  SensorData data;
  // Генерація температури в діапазоні 15.0 ... 30.0 °C
  data.temperature = random(150, 301) / 10.0f;
  // Генерація вологості в діапазоні 30.0 ... 65.0 %
  data.humidity = random(300, 651) / 10.0f;
  // Timestamp як поточний час роботи програми у секундах від старту
  data.timestamp = millis() / 1000;
  return data;
}

// Функція для демонстрації переповнення типів даних (Overflow)
void demonstrateIntegerOverflow() {
  Serial.println("==================================================");
  Serial.println("1. Демонстрація переповнення типів даних (Integer Overflow)");
  Serial.println("==================================================");

  uint8_t a = 200;
  uint8_t b = 100;
  uint8_t sum = a + b;

  Serial.print("Змінна a (uint8_t): "); Serial.println(a);
  Serial.print("Змінна b (uint8_t): "); Serial.println(b);
  Serial.print("Математична сума (a + b): "); Serial.println((int)a + (int)b);
  Serial.print("Результат у sum (uint8_t): "); Serial.println(sum);
  Serial.println("--------------------------------------------------");
  Serial.println("ПОЯСНЕННЯ:");
  Serial.println("Тип uint8_t приймає значення від 0 до 255 (2^8 = 256 станів).");
  Serial.println("При додаванні 200 + 100 виникає overflow: 300 % 256 = 44.");
  Serial.println("==================================================\n");
}

// Вивід даних сенсора
void logSensorData() {
  SensorData currentData = readSensorData();
  Serial.print("[SENSOR DATA] Timestamp: ");
  Serial.print(currentData.timestamp);
  Serial.print("s | Температура: ");
  Serial.print(currentData.temperature, 1);
  Serial.print(" °C | Вологість: ");
  Serial.print(currentData.humidity, 1);
  Serial.println(" %");
}

// Моніторинг вільної пам'яті (Heap)
void logHeapMemory() {
  uint32_t currentFreeHeap = ESP.getFreeHeap();
  int32_t heapDiff = (int32_t)currentFreeHeap - (int32_t)initialFreeHeap;

  Serial.println("--------------------------------------------------");
  Serial.print("[MEMORY MONITOR] Free Heap: ");
  Serial.print(currentFreeHeap);
  Serial.print(" bytes | Min Free Heap: ");
  Serial.print(ESP.getMinFreeHeap());
  Serial.print(" bytes | Delta: ");
  if (heapDiff >= 0) Serial.print("+");
  Serial.print(heapDiff);
  Serial.println(" bytes");

  if (heapDiff < 0) {
    Serial.println("⚠️ УВАГА: Вільна пам'ять зменшується! Можливий витік пам'яті (Memory Leak).");
  } else {
    Serial.println("✅ Стан пам'яті стабільний (Memory consumption is stable).");
  }
  Serial.println("--------------------------------------------------");
}

void setup() {
  Serial.begin(115200);
  delay(1000); // Час на ініціалізацію послідовного порту

  // Ініціалізація генератора випадкових чисел
  randomSeed(analogRead(34));

  Serial.println("\n=== ESP32 Lesson 2: Memory, Structs & Data Types ===");
  
  // 1. Пояснення переповнення типів
  demonstrateIntegerOverflow();

  initialFreeHeap = ESP.getFreeHeap();
  Serial.print("Початковий обсяг вільного Heap: ");
  Serial.print(initialFreeHeap);
  Serial.println(" bytes\n");

  Serial.println("Запуск неблокуючого моніторингу...");
  Serial.println("- Читання сенсора кожні 20 секунд");
  Serial.println("- Перевірка вільної пам'яті кожні 60 секунд\n");

  // Виведемо перше значення відразу при старті
  logSensorData();
  lastSensorReadTime = millis();
  lastHeapMonitorTime = millis();
}

void loop() {
  unsigned long currentMillis = millis();

  // Задача 1: Вивід даних сенсора кожні 20 секунд (20 000 мс)
  if (currentMillis - lastSensorReadTime >= SENSOR_READ_INTERVAL) {
    lastSensorReadTime = currentMillis;
    logSensorData();
  }

  // Задача 2: Моніторинг вільної пам'яті кожні 60 секунд (60 000 мс)
  if (currentMillis - lastHeapMonitorTime >= HEAP_MONITOR_INTERVAL) {
    lastHeapMonitorTime = currentMillis;
    logHeapMemory();
  }
}
