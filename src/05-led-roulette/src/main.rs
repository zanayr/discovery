#![deny(unsafe_code)]
#![no_main]
#![no_std]

use volatile::Volatile;
use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

    let mut ms = 50_u8;
    let v_ms = Volatile::new(&mut ms);

    loop {
        for curr in 0..8 {
            let next = (curr + 1) % 8;

            leds[next].on().ok();
            delay.delay_ms(v_ms.read());

            leds[curr].off().ok();
            delay.delay_ms(v_ms.read());
        }
    }
}