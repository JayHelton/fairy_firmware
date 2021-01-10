#![allow(non_camel_case_types)]

pub mod gpio;
pub mod port_0;
pub struct Peripherals {
    pub port_0: port_0::Port0,
}

impl Peripherals {
    pub fn take() -> Option<Self> {
        Some(unsafe { Peripherals::steal() })
    }

    pub unsafe fn steal() -> Self {
        Peripherals {
            port_0: port_0::Port0::new(),
        }
    }
}
