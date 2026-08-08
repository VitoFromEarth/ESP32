#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Level, Output, OutputConfig},
    main,
    time::Duration,
};
use esp_println::{print, println};

const DOT_DURATION_MS: u64 = 200;
const DASH_DURATION_MS: u64 = 600;
const ELEMENT_PAUSE_MS: u64 = 200;
const LETTER_PAUSE_MS: u64 = 600;
const WORD_PAUSE_MS: u64 = 1400;

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    // Конфігурація GPIO 2 (вбудований LED) та GPIO 4 (зовнішній LED для Wokwi)
    let mut led = Output::new(peripherals.GPIO2, Level::Low, OutputConfig::default());
    let mut ext_led = Output::new(peripherals.GPIO4, Level::Low, OutputConfig::default());

    println!("\r\n=== ESP32 Morse Code SOS Signal (Rust) ===");
    println!("\rКрапка (.): 200 мс | Тире (-): 600 мс");
    println!("\rПочаток передачі...\r\n");

    let send_dot = |l: &mut Output, el: &mut Output| {
        print!(".");
        l.set_high();
        el.set_high();
        delay.delay(Duration::from_millis(DOT_DURATION_MS));
        l.set_low();
        el.set_low();
        delay.delay(Duration::from_millis(ELEMENT_PAUSE_MS));
    };

    let send_dash = |l: &mut Output, el: &mut Output| {
        print!("-");
        l.set_high();
        el.set_high();
        delay.delay(Duration::from_millis(DASH_DURATION_MS));
        l.set_low();
        el.set_low();
        delay.delay(Duration::from_millis(ELEMENT_PAUSE_MS));
    };

    let send_s = |l: &mut Output, el: &mut Output| {
        send_dot(l, el);
        send_dot(l, el);
        send_dot(l, el);
        delay.delay(Duration::from_millis(LETTER_PAUSE_MS - ELEMENT_PAUSE_MS));
        print!(" ");
    };

    let send_o = |l: &mut Output, el: &mut Output| {
        send_dash(l, el);
        send_dash(l, el);
        send_dash(l, el);
        delay.delay(Duration::from_millis(LETTER_PAUSE_MS - ELEMENT_PAUSE_MS));
        print!(" ");
    };

    loop {
        print!("\rSOS: ");
        send_s(&mut led, &mut ext_led);
        send_o(&mut led, &mut ext_led);
        send_s(&mut led, &mut ext_led);
        println!(" [Передано]\r");

        delay.delay(Duration::from_millis(
            WORD_PAUSE_MS - (LETTER_PAUSE_MS - ELEMENT_PAUSE_MS),
        ));
    }
}

