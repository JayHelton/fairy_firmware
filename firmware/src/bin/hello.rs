#![no_main]
#![no_std]

use embedded_hal::digital::v2::OutputPin;
use firmware as _; // global logger + panicking-behavior + memory layout
use hal::{self, pa::gpio::pins::Port0Parts, pa::gpio::Level};

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello, world!");
    let board = hal::pa::Peripherals::take().unwrap();
    let pins = Port0Parts::new(board.port_0);
    let mut led_1 = pins.p0_13.into_push_pull_output(Level::Low);
    loop {
        led_1.set_low().unwrap();
    }
}
