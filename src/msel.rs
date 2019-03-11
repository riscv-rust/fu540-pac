#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - The MSEL pin state"]
    pub msel: MSEL,
}
#[doc = "The MSEL pin state"]
pub struct MSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "The MSEL pin state"]
pub mod msel;
