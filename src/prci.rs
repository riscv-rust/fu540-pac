#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Crystal Input Control Register"]
    pub hfxosccfg: HFXOSCCFG,
    #[doc = "0x04 - Core PLL Configuration Register"]
    pub corepllcfg0: COREPLLCFG0,
    _reserved0: [u8; 4usize],
    #[doc = "0x0c - DDR PLL Configuration Register"]
    pub ddrpllcfg0: DDRPLLCFG0,
    #[doc = "0x10 - DDR PLL Configuration Register"]
    pub ddrpllcfg1: DDRPLLCFG1,
    _reserved1: [u8; 8usize],
    #[doc = "0x1c - Gigabit Ethernet PLL Configuration Register"]
    pub gemgxlpllcfg0: GEMGXLPLLCFG0,
    #[doc = "0x20 - Gigabit Ethernet PLL Configuration Register"]
    pub gemgxlpllcfg1: GEMGXLPLLCFG1,
    #[doc = "0x24 - CORECLK Source Selection Register"]
    pub coreclksel: CORECLKSEL,
    #[doc = "0x28 - Peripheral Devices Reset Control Register"]
    pub devicesresetreg: DEVICESRESETREG,
    #[doc = "0x2c - CLKMUX Status Register"]
    pub clkmuxstatus: CLKMUXSTATUS,
    _reserved2: [u8; 192usize],
    #[doc = "0xf0 - PROCMON Configuration Register"]
    pub procmoncfg: PROCMONCFG,
}
#[doc = "Crystal Input Control Register"]
pub struct HFXOSCCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Crystal Input Control Register"]
pub mod hfxosccfg;
#[doc = "Core PLL Configuration Register"]
pub struct COREPLLCFG0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Core PLL Configuration Register"]
pub mod corepllcfg0;
#[doc = "DDR PLL Configuration Register"]
pub struct DDRPLLCFG0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DDR PLL Configuration Register"]
pub mod ddrpllcfg0;
#[doc = "DDR PLL Configuration Register"]
pub struct DDRPLLCFG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DDR PLL Configuration Register"]
pub mod ddrpllcfg1;
#[doc = "Gigabit Ethernet PLL Configuration Register"]
pub struct GEMGXLPLLCFG0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Gigabit Ethernet PLL Configuration Register"]
pub mod gemgxlpllcfg0;
#[doc = "Gigabit Ethernet PLL Configuration Register"]
pub struct GEMGXLPLLCFG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Gigabit Ethernet PLL Configuration Register"]
pub mod gemgxlpllcfg1;
#[doc = "CORECLK Source Selection Register"]
pub struct CORECLKSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CORECLK Source Selection Register"]
pub mod coreclksel;
#[doc = "Peripheral Devices Reset Control Register"]
pub struct DEVICESRESETREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Devices Reset Control Register"]
pub mod devicesresetreg;
#[doc = "CLKMUX Status Register"]
pub struct CLKMUXSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLKMUX Status Register"]
pub mod clkmuxstatus;
#[doc = "PROCMON Configuration Register"]
pub struct PROCMONCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PROCMON Configuration Register"]
pub mod procmoncfg;
