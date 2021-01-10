use core::marker::PhantomData;
use core::ops::Deref;
#[doc = "GPIO Port 1"]
pub struct Port0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for Port0 {}
impl Port0 {
    pub const fn ptr() -> *const port_0::RegisterBlock {
        0x5000_0000 as *const _
    }
}

impl Deref for Port0 {
    type Target = port_0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*Port0::ptr() }
    }
}

pub mod gpio;
pub mod port_0;
pub struct Peripherals {
    pub port_0: Port0,
}

impl Peripherals {
    pub fn take() -> Option<Self> {
        Some(unsafe { Peripherals::steal() })
    }

    pub unsafe fn steal() -> Self {
        Peripherals {
            port_0: Port0 {
                _marker: PhantomData,
            },
        }
    }
}
