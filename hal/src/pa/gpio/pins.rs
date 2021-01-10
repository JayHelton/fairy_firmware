use super::{Disconnected, Level, Output, PhantomData, Pin, Port, Port0, PushPull};
use embedded_hal::digital::v2::OutputPin;
use void::Void;
// use void::Void;
/// GPIO parts
pub struct Port0Parts {
    /// Pin
    pub p0_13: P0_13<Disconnected>,
}
impl Port0Parts {
    pub fn new(_gpio: Port0) -> Self {
        Self {
            p0_13: P0_13 { _mode: PhantomData },
        }
    }
}
pub struct P0_13<MODE> {
    _mode: PhantomData<MODE>,
}

impl<MODE> P0_13<MODE> {
    /// Convert the pin to bepin a push-pull output with normal drive
    pub fn into_push_pull_output(self, initial_output: Level) -> P0_13<Output<PushPull>> {
        let mut pin = P0_13 { _mode: PhantomData };
        match initial_output {
            Level::Low => pin.set_low().unwrap(),
            Level::High => pin.set_high().unwrap(),
        }
        unsafe { &(*Port0::ptr()).pin_cnf[13] }.write(|w| {
            w.dir().output();
            w.input().disconnect();
            w.pull().disabled();
            w.drive().s0s1();
            w.sense().disabled();
            w
        });
        pin
    }

    pub fn into_disconnected(self) -> P0_13<Disconnected> {
        unsafe { &(*Port0::ptr()).pin_cnf[13] }.reset();
        P0_13 { _mode: PhantomData }
    }

    pub fn degrade(self) -> Pin<MODE> {
        Pin::new(Port::Port0, 13)
    }
}

impl<MODE> OutputPin for P0_13<Output<MODE>> {
    type Error = Void;
    /// Set the output as high
    fn set_high(&mut self) -> Result<(), Self::Error> {
        unsafe {
            (*Port0::ptr()).outset.write(|w| w.bits(1u32 << 13));
        }
        Ok(())
    }
    /// Set the output as low
    fn set_low(&mut self) -> Result<(), Self::Error> {
        unsafe {
            (*Port0::ptr()).outclr.write(|w| w.bits(1u32 << 13));
        }
        Ok(())
    }
}
