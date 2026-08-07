#include <Arduino.h>

#define LED 2     // вбудований світлодіод
#define EXT_LED 4 // зовнішній світлодіод

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