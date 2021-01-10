use core::marker::PhantomData;

use crate::pa::{port_0 as gpio, Port0};
use embedded_hal::digital::v2::OutputPin;
use void::Void;

pub struct Disconnected;
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}
/// Push pull output (type state).
pub struct PushPull;
/// Represents a digital input or output level.
pub enum Level {
    Low,
    High,
}
/// A GPIO port with up to 32 pins.
pub enum Port {
    /// Port 0, available on all nRF52 and nRF51 MCUs.
    Port0,
}

pub struct Pin<MODE> {
    pin_port: u8,
    _mode: PhantomData<MODE>,
}
impl<MODE> Pin<MODE> {
    pub(crate) fn new(port: Port, pin: u8) -> Self {
        let port_bits = match port {
            Port::Port0 => 0x00,
        };
        Self {
            pin_port: pin | port_bits,
            _mode: PhantomData,
        }
    }
    #[inline]
    pub fn pin(&self) -> u8 {
        #[cfg(not(any(feature = "52833", feature = "52840")))]
        {
            self.pin_port
        }
    }
    #[inline]
    pub fn port(&self) -> Port {
        #[cfg(not(any(feature = "52833", feature = "52840")))]
        {
            Port::Port0
        }
    }
    fn block(&self) -> &gpio::RegisterBlock {
        let ptr = match self.port() {
            Port::Port0 => Port0::ptr(),
        };
        unsafe { &*ptr }
    }
    pub(crate) fn conf(&self) -> &gpio::PIN_CNF {
        &self.block().pin_cnf[self.pin() as usize]
    }

    /// Convert the pin to be a push-pull output with normal drive.
    pub fn into_push_pull_output(self, initial_output: Level) -> Pin<Output<PushPull>> {
        let mut pin = Pin {
            _mode: PhantomData,
            pin_port: self.pin_port,
        };
        match initial_output {
            Level::Low => pin.set_low().unwrap(),
            Level::High => pin.set_high().unwrap(),
        }
        self.conf().write(|w| {
            w.dir().output();
            w.input().connect();
            w.pull().disabled();
            w.drive().s0s1();
            w.sense().disabled();
            w
        });
        pin
    }
    /// Disconnects the pin.
    ///
    /// In disconnected mode the pin cannot be used as input or output.
    /// It is primarily useful to reduce power usage.
    pub fn into_disconnected(self) -> Pin<Disconnected> {
        self.conf().reset();
        Pin {
            _mode: PhantomData,
            pin_port: self.pin_port,
        }
    }
}

impl<MODE> OutputPin for Pin<Output<MODE>> {
    type Error = Void;
    /// Set the output as high.
    fn set_high(&mut self) -> Result<(), Self::Error> {
        unsafe {
            self.block().outset.write(|w| w.bits(1u32 << self.pin()));
        }
        Ok(())
    }
    /// Set the output as low.
    fn set_low(&mut self) -> Result<(), Self::Error> {
        unsafe {
            self.block().outclr.write(|w| w.bits(1u32 << self.pin()));
        }
        Ok(())
    }
}

pub mod pins;
