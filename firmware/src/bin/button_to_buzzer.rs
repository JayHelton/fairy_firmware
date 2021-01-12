#![no_main]
#![no_std]

use embedded_hal::digital::v2::{InputPin, OutputPin};
use firmware as _; // global logger + panicking-behavior + memory layout
use hal::{self, peripherals::gpio::pins::Port0Pins, peripherals::gpio::Level};

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello, world!");
    let board = hal::peripherals::Peripherals::take().unwrap();
    let pins = Port0Pins::new(board.port_0);
    let mut led_1 = pins.p0_13.into_push_pull_output(Level::Low);
    let mut led_2 = pins.p0_3.into_push_pull_output(Level::High);
    let button_1 = pins.p0_11.into_pullup_input();
    loop {
        if button_1.is_low().unwrap() {
            led_1.set_low().unwrap();
            led_2.set_low().unwrap();
        } else {
            led_1.set_high().unwrap();
            led_2.set_high().unwrap();
        }
    }
}
