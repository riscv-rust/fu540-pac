#![doc = "Peripheral access API for FU540 microcontrollers (generated using svd2rust v0.14.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.14.0/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate riscv;
#[cfg(feature = "rt")]
extern crate riscv_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = "Mode Select"]
pub struct MSEL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MSEL {}
impl MSEL {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const msel::RegisterBlock {
        4096 as *const _
    }
}
impl Deref for MSEL {
    type Target = msel::RegisterBlock;
    fn deref(&self) -> &msel::RegisterBlock {
        unsafe { &*MSEL::ptr() }
    }
}
#[doc = "Mode Select"]
pub mod msel;
#[doc = "PRCI (Power Reset Clocking Interrupt) Block"]
pub struct PRCI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PRCI {}
impl PRCI {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const prci::RegisterBlock {
        268435456 as *const _
    }
}
impl Deref for PRCI {
    type Target = prci::RegisterBlock;
    fn deref(&self) -> &prci::RegisterBlock {
        unsafe { &*PRCI::ptr() }
    }
}
#[doc = "PRCI (Power Reset Clocking Interrupt) Block"]
pub mod prci;
#[doc = "Universal Asynchronous Receiver Transmitter 0"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart0::RegisterBlock {
        268500992 as *const _
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    fn deref(&self) -> &uart0::RegisterBlock {
        unsafe { &*UART0::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver Transmitter 0"]
pub mod uart0;
#[doc = "Universal Asynchronous Receiver Transmitter 1"]
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART1 {}
impl UART1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart0::RegisterBlock {
        268505088 as *const _
    }
}
impl Deref for UART1 {
    type Target = uart0::RegisterBlock;
    fn deref(&self) -> &uart0::RegisterBlock {
        unsafe { &*UART1::ptr() }
    }
}
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "MSEL"]
    pub MSEL: MSEL,
    #[doc = "PRCI"]
    pub PRCI: PRCI,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "UART1"]
    pub UART1: UART1,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        riscv::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            MSEL: MSEL {
                _marker: PhantomData,
            },
            PRCI: PRCI {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            UART1: UART1 {
                _marker: PhantomData,
            },
        }
    }
}
