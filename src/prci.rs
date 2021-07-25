#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Crystal Input Control Register"]
    pub hfxosccfg: crate::Reg<hfxosccfg::HFXOSCCFG_SPEC>,
    #[doc = "0x04 - Core PLL Configuration Register"]
    pub corepllcfg0: crate::Reg<corepllcfg0::COREPLLCFG0_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - DDR PLL Configuration Register"]
    pub ddrpllcfg0: crate::Reg<ddrpllcfg0::DDRPLLCFG0_SPEC>,
    #[doc = "0x10 - DDR PLL Configuration Register"]
    pub ddrpllcfg1: crate::Reg<ddrpllcfg1::DDRPLLCFG1_SPEC>,
    _reserved4: [u8; 0x08],
    #[doc = "0x1c - Gigabit Ethernet PLL Configuration Register"]
    pub gemgxlpllcfg0: crate::Reg<gemgxlpllcfg0::GEMGXLPLLCFG0_SPEC>,
    #[doc = "0x20 - Gigabit Ethernet PLL Configuration Register"]
    pub gemgxlpllcfg1: crate::Reg<gemgxlpllcfg1::GEMGXLPLLCFG1_SPEC>,
    #[doc = "0x24 - CORECLK Source Selection Register"]
    pub coreclksel: crate::Reg<coreclksel::CORECLKSEL_SPEC>,
    #[doc = "0x28 - Peripheral Devices Reset Control Register"]
    pub devicesresetreg: crate::Reg<devicesresetreg::DEVICESRESETREG_SPEC>,
    #[doc = "0x2c - CLKMUX Status Register"]
    pub clkmuxstatus: crate::Reg<clkmuxstatus::CLKMUXSTATUS_SPEC>,
    _reserved9: [u8; 0xc0],
    #[doc = "0xf0 - PROCMON Configuration Register"]
    pub procmoncfg: crate::Reg<procmoncfg::PROCMONCFG_SPEC>,
}
#[doc = "hfxosccfg register accessor: an alias for `Reg<HFXOSCCFG_SPEC>`"]
pub type HFXOSCCFG = crate::Reg<hfxosccfg::HFXOSCCFG_SPEC>;
#[doc = "Crystal Input Control Register"]
pub mod hfxosccfg;
#[doc = "corepllcfg0 register accessor: an alias for `Reg<COREPLLCFG0_SPEC>`"]
pub type COREPLLCFG0 = crate::Reg<corepllcfg0::COREPLLCFG0_SPEC>;
#[doc = "Core PLL Configuration Register"]
pub mod corepllcfg0;
#[doc = "ddrpllcfg0 register accessor: an alias for `Reg<DDRPLLCFG0_SPEC>`"]
pub type DDRPLLCFG0 = crate::Reg<ddrpllcfg0::DDRPLLCFG0_SPEC>;
#[doc = "DDR PLL Configuration Register"]
pub mod ddrpllcfg0;
#[doc = "ddrpllcfg1 register accessor: an alias for `Reg<DDRPLLCFG1_SPEC>`"]
pub type DDRPLLCFG1 = crate::Reg<ddrpllcfg1::DDRPLLCFG1_SPEC>;
#[doc = "DDR PLL Configuration Register"]
pub mod ddrpllcfg1;
#[doc = "gemgxlpllcfg0 register accessor: an alias for `Reg<GEMGXLPLLCFG0_SPEC>`"]
pub type GEMGXLPLLCFG0 = crate::Reg<gemgxlpllcfg0::GEMGXLPLLCFG0_SPEC>;
#[doc = "Gigabit Ethernet PLL Configuration Register"]
pub mod gemgxlpllcfg0;
#[doc = "gemgxlpllcfg1 register accessor: an alias for `Reg<GEMGXLPLLCFG1_SPEC>`"]
pub type GEMGXLPLLCFG1 = crate::Reg<gemgxlpllcfg1::GEMGXLPLLCFG1_SPEC>;
#[doc = "Gigabit Ethernet PLL Configuration Register"]
pub mod gemgxlpllcfg1;
#[doc = "coreclksel register accessor: an alias for `Reg<CORECLKSEL_SPEC>`"]
pub type CORECLKSEL = crate::Reg<coreclksel::CORECLKSEL_SPEC>;
#[doc = "CORECLK Source Selection Register"]
pub mod coreclksel;
#[doc = "devicesresetreg register accessor: an alias for `Reg<DEVICESRESETREG_SPEC>`"]
pub type DEVICESRESETREG = crate::Reg<devicesresetreg::DEVICESRESETREG_SPEC>;
#[doc = "Peripheral Devices Reset Control Register"]
pub mod devicesresetreg;
#[doc = "clkmuxstatus register accessor: an alias for `Reg<CLKMUXSTATUS_SPEC>`"]
pub type CLKMUXSTATUS = crate::Reg<clkmuxstatus::CLKMUXSTATUS_SPEC>;
#[doc = "CLKMUX Status Register"]
pub mod clkmuxstatus;
#[doc = "procmoncfg register accessor: an alias for `Reg<PROCMONCFG_SPEC>`"]
pub type PROCMONCFG = crate::Reg<procmoncfg::PROCMONCFG_SPEC>;
#[doc = "PROCMON Configuration Register"]
pub mod procmoncfg;
