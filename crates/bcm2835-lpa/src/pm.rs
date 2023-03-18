#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1c],
    #[doc = "0x1c - Reset Control"]
    pub rstc: RSTC,
    _reserved1: [u8; 0x04],
    #[doc = "0x24 - Watchdog control"]
    pub wdog: WDOG,
}
#[doc = "RSTC (rw) register accessor: an alias for `Reg<RSTC_SPEC>`"]
pub type RSTC = crate::Reg<rstc::RSTC_SPEC>;
#[doc = "Reset Control"]
pub mod rstc;
#[doc = "WDOG (rw) register accessor: an alias for `Reg<WDOG_SPEC>`"]
pub type WDOG = crate::Reg<wdog::WDOG_SPEC>;
#[doc = "Watchdog control"]
pub mod wdog;
