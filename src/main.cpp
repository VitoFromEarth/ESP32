#include <Arduino.h>

#define LED 2
#define EXT_LED 4

const int DOT_DURATION = 200;
const int DASH_DURATION = 600;
const int ELEMENT_PAUSE = 200;
const int LETTER_PAUSE = 600;
const int WORD_PAUSE = 1400;

void setLeds(bool state) {
  digitalWrite(LED, state ? HIGH : LOW);
  digitalWrite(EXT_LED, state ? HIGH : LOW);
}

// Передача сигналу "Крапка" (.)
void sendDot() {
  Serial.print('.');
  setLeds(true);
  delay(DOT_DURATION);
  setLeds(false);
  delay(ELEMENT_PAUSE);
}

// Передача сигналу "Тире" (-)
void sendDash() {
  Serial.print('-');
  setLeds(true);
  delay(DASH_DURATION);
  setLeds(false);
  delay(ELEMENT_PAUSE);
}

// Передача літери 'S' (... — 3 крапки)
void sendS() {
  sendDot();
  sendDot();
  sendDot();
  delay(LETTER_PAUSE - ELEMENT_PAUSE); // Пауза між літерами
  Serial.print(' ');
}

// Передача літери 'O' (--- — 3 тире)
void sendO() {
  sendDash();
  sendDash();
  sendDash();
  delay(LETTER_PAUSE - ELEMENT_PAUSE); // Пауза між літерами
  Serial.print(' ');
}

// Повний цикл сигналу SOS (... --- ...)
void sendSOS() {
  Serial.print("SOS: ");
  sendS(); // S: ...
  sendO(); // O: ---
  sendS(); // S: ...
  Serial.println(" [Передано]");

  // Пауза між повтореннями сигналу SOS
  delay(WORD_PAUSE - (LETTER_PAUSE - ELEMENT_PAUSE));
}

void setup() {
  Serial.begin(115200);
  pinMode(LED, OUTPUT);
  pinMode(EXT_LED, OUTPUT);

  setLeds(false);

  Serial.println("\n=== ESP32 Morse Code SOS Signal Initialized ===");
  Serial.println("Крапка (.): 200 мс | Тире (-): 600 мс");
  Serial.println("Початок передачі через 1 секунду...\n");
  delay(1000);
}

void loop() { sendSOS(); }
