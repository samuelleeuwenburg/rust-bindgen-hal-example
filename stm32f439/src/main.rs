#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;

// import our homebrew C HAL wrapper crate
use stmlib::{clock_enable, hal_init, led_init, toggle_pin};

// GPIO B
const GPIOB: usize = 0x40020400;

// user led LD1
const LED: usize = 1;

#[entry]
fn main() -> ! {
    hal_init();

    clock_enable();

    led_init(GPIOB, LED);

    loop {
        toggle_pin(GPIOB, LED);
    }
}
