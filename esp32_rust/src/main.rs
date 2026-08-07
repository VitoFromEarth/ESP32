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
