#![no_main]
#![no_std]

use firmware as _; // global logger + panicking-behavior + memory layout
use hal::it_works;
#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::debug!("{:?}", it_works());

    firmware::exit()
}
