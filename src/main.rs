#![no_std]
#![no_main]

use panic_halt as _;
use arduino_hal::{Peripherals, pins, delay_ms, entry};

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = pins!(dp);

    let mut led = pins.d13.into_output();

    loop {
        led.toggle();
        delay_ms(1000);
    }
}
