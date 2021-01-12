use crate::generics::{Readable, Reg, Writable};
use core::{marker::PhantomData, ops::Deref};

#[doc = "GPIO Port 1"]
pub struct Port0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for Port0 {}
impl Port0 {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
    pub const fn ptr() -> *const RegisterBlock {
        0x5000_0000 as *const _
    }
}

impl Deref for Port0 {
    type Target = RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*Port0::ptr() }
    }
}

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1284usize],
    #[doc = "0x504 - Write GPIO port"]
    pub out: OUT,
    #[doc = "0x508 - Set individual bits in GPIO port"]
    pub outset: OUTSET,
    #[doc = "0x50c - Clear individual bits in GPIO port"]
    pub outclr: OUTCLR,
    #[doc = "0x510 - Read GPIO port"]
    pub in_: IN,
    #[doc = "0x514 - Direction of GPIO pins"]
    pub dir: DIR,
    #[doc = "0x518 - DIR set register"]
    pub dirset: DIRSET,
    #[doc = "0x51c - DIR clear register"]
    pub dirclr: DIRCLR,
    #[doc = "0x520 - Latch register indicating what GPIO pins that have met the criteria set in the PIN_CNF\\[n\\].SENSE registers"]
    pub latch: LATCH,
    #[doc = "0x524 - Select between default DETECT signal behaviour and LDETECT mode"]
    pub detectmode: DETECTMODE,
    _reserved9: [u8; 472usize],
    #[doc = "0x700 - Description collection\\[n\\]: Configuration of GPIO pins"]
    pub pin_cnf: [PIN_CNF; 32],
}
pub type OUT = Reg<u32, _OUT>;
pub struct _OUT;
impl Readable for OUT {}
impl Writable for OUT {}
pub mod out;

pub type OUTSET = Reg<u32, _OUTSET>;
pub struct _OUTSET;
impl Readable for OUTSET {}
impl Writable for OUTSET {}
pub mod outset;

pub type OUTCLR = Reg<u32, _OUTCLR>;
pub struct _OUTCLR;
impl Readable for OUTCLR {}
impl Writable for OUTCLR {}
pub mod outclr;

pub type IN = Reg<u32, _IN>;
pub struct _IN;
impl Readable for IN {}
pub mod in_;

pub type DIR = Reg<u32, _DIR>;
pub struct _DIR;
impl Readable for DIR {}
impl Writable for DIR {}
pub mod dir;

pub type DIRSET = Reg<u32, _DIRSET>;
pub struct _DIRSET;
impl Readable for DIRSET {}
impl Writable for DIRSET {}
pub mod dirset;

pub type DIRCLR = Reg<u32, _DIRCLR>;
pub struct _DIRCLR;
impl Readable for DIRCLR {}
impl Writable for DIRCLR {}
pub mod dirclr;

pub type LATCH = Reg<u32, _LATCH>;
pub struct _LATCH;
impl Readable for LATCH {}
impl Writable for LATCH {}
pub mod latch;

pub type DETECTMODE = Reg<u32, _DETECTMODE>;
pub struct _DETECTMODE;
impl Readable for DETECTMODE {}
impl Writable for DETECTMODE {}
pub mod detectmode;

pub type PIN_CNF = Reg<u32, _PIN_CNF>;
pub struct _PIN_CNF;
impl Readable for PIN_CNF {}
impl Writable for PIN_CNF {}
pub mod pin_cnf;
