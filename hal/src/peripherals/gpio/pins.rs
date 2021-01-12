use super::{Disconnected, Pin, Port};
use crate::peripherals::port_0::Port0;

pub struct Port0Pins {
    pub p0_3: Pin<Disconnected>,
    pub p0_11: Pin<Disconnected>,
    pub p0_13: Pin<Disconnected>,
}

impl Port0Pins {
    pub fn new(_gpio: Port0) -> Self {
        Self {
            p0_3: Pin::new(Port::Port0, 3),
            p0_11: Pin::new(Port::Port0, 11),
            p0_13: Pin::new(Port::Port0, 13),
        }
    }
}
