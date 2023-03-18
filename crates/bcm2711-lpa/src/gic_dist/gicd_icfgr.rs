#[doc = r"Register block"]
#[repr(C)]
pub struct GICD_ICFGR {
    #[doc = "0x00 - Interrupt Configuration 0 - 15"]
    pub gicd_icfgr0: GICD_ICFGR0,
    #[doc = "0x04 - Interrupt Configuration 16 - 31"]
    pub gicd_icfgr4: GICD_ICFGR4,
    #[doc = "0x08 - Interrupt Configuration 32 - 47"]
    pub gicd_icfgr8: GICD_ICFGR8,
    #[doc = "0x0c - Interrupt Configuration 48 - 63"]
    pub gicd_icfgr12: GICD_ICFGR12,
    #[doc = "0x10 - Interrupt Configuration 64 - 79"]
    pub gicd_icfgr16: GICD_ICFGR16,
    #[doc = "0x14 - Interrupt Configuration 80 - 95"]
    pub gicd_icfgr20: GICD_ICFGR20,
    #[doc = "0x18 - Interrupt Configuration 96 - 111"]
    pub gicd_icfgr24: GICD_ICFGR24,
    #[doc = "0x1c - Interrupt Configuration 112 - 127"]
    pub gicd_icfgr28: GICD_ICFGR28,
    #[doc = "0x20 - Interrupt Configuration 128 - 143"]
    pub gicd_icfgr32: GICD_ICFGR32,
    #[doc = "0x24 - Interrupt Configuration 144 - 159"]
    pub gicd_icfgr36: GICD_ICFGR36,
    #[doc = "0x28 - Interrupt Configuration 160 - 175"]
    pub gicd_icfgr40: GICD_ICFGR40,
    #[doc = "0x2c - Interrupt Configuration 176 - 191"]
    pub gicd_icfgr44: GICD_ICFGR44,
    #[doc = "0x30 - Interrupt Configuration 192 - 207"]
    pub gicd_icfgr48: GICD_ICFGR48,
    #[doc = "0x34 - Interrupt Configuration 208 - 223"]
    pub gicd_icfgr52: GICD_ICFGR52,
}
#[doc = "GICD_ICFGR0 (rw) register accessor: an alias for `Reg<GICD_ICFGR0_SPEC>`"]
pub type GICD_ICFGR0 = crate::Reg<gicd_icfgr0::GICD_ICFGR0_SPEC>;
#[doc = "Interrupt Configuration 0 - 15"]
pub mod gicd_icfgr0;
#[doc = "GICD_ICFGR4 (rw) register accessor: an alias for `Reg<GICD_ICFGR4_SPEC>`"]
pub type GICD_ICFGR4 = crate::Reg<gicd_icfgr4::GICD_ICFGR4_SPEC>;
#[doc = "Interrupt Configuration 16 - 31"]
pub mod gicd_icfgr4;
#[doc = "GICD_ICFGR8 (rw) register accessor: an alias for `Reg<GICD_ICFGR8_SPEC>`"]
pub type GICD_ICFGR8 = crate::Reg<gicd_icfgr8::GICD_ICFGR8_SPEC>;
#[doc = "Interrupt Configuration 32 - 47"]
pub mod gicd_icfgr8;
#[doc = "GICD_ICFGR12 (rw) register accessor: an alias for `Reg<GICD_ICFGR12_SPEC>`"]
pub type GICD_ICFGR12 = crate::Reg<gicd_icfgr12::GICD_ICFGR12_SPEC>;
#[doc = "Interrupt Configuration 48 - 63"]
pub mod gicd_icfgr12;
#[doc = "GICD_ICFGR16 (rw) register accessor: an alias for `Reg<GICD_ICFGR16_SPEC>`"]
pub type GICD_ICFGR16 = crate::Reg<gicd_icfgr16::GICD_ICFGR16_SPEC>;
#[doc = "Interrupt Configuration 64 - 79"]
pub mod gicd_icfgr16;
#[doc = "GICD_ICFGR20 (rw) register accessor: an alias for `Reg<GICD_ICFGR20_SPEC>`"]
pub type GICD_ICFGR20 = crate::Reg<gicd_icfgr20::GICD_ICFGR20_SPEC>;
#[doc = "Interrupt Configuration 80 - 95"]
pub mod gicd_icfgr20;
#[doc = "GICD_ICFGR24 (rw) register accessor: an alias for `Reg<GICD_ICFGR24_SPEC>`"]
pub type GICD_ICFGR24 = crate::Reg<gicd_icfgr24::GICD_ICFGR24_SPEC>;
#[doc = "Interrupt Configuration 96 - 111"]
pub mod gicd_icfgr24;
#[doc = "GICD_ICFGR28 (rw) register accessor: an alias for `Reg<GICD_ICFGR28_SPEC>`"]
pub type GICD_ICFGR28 = crate::Reg<gicd_icfgr28::GICD_ICFGR28_SPEC>;
#[doc = "Interrupt Configuration 112 - 127"]
pub mod gicd_icfgr28;
#[doc = "GICD_ICFGR32 (rw) register accessor: an alias for `Reg<GICD_ICFGR32_SPEC>`"]
pub type GICD_ICFGR32 = crate::Reg<gicd_icfgr32::GICD_ICFGR32_SPEC>;
#[doc = "Interrupt Configuration 128 - 143"]
pub mod gicd_icfgr32;
#[doc = "GICD_ICFGR36 (rw) register accessor: an alias for `Reg<GICD_ICFGR36_SPEC>`"]
pub type GICD_ICFGR36 = crate::Reg<gicd_icfgr36::GICD_ICFGR36_SPEC>;
#[doc = "Interrupt Configuration 144 - 159"]
pub mod gicd_icfgr36;
#[doc = "GICD_ICFGR40 (rw) register accessor: an alias for `Reg<GICD_ICFGR40_SPEC>`"]
pub type GICD_ICFGR40 = crate::Reg<gicd_icfgr40::GICD_ICFGR40_SPEC>;
#[doc = "Interrupt Configuration 160 - 175"]
pub mod gicd_icfgr40;
#[doc = "GICD_ICFGR44 (rw) register accessor: an alias for `Reg<GICD_ICFGR44_SPEC>`"]
pub type GICD_ICFGR44 = crate::Reg<gicd_icfgr44::GICD_ICFGR44_SPEC>;
#[doc = "Interrupt Configuration 176 - 191"]
pub mod gicd_icfgr44;
#[doc = "GICD_ICFGR48 (rw) register accessor: an alias for `Reg<GICD_ICFGR48_SPEC>`"]
pub type GICD_ICFGR48 = crate::Reg<gicd_icfgr48::GICD_ICFGR48_SPEC>;
#[doc = "Interrupt Configuration 192 - 207"]
pub mod gicd_icfgr48;
#[doc = "GICD_ICFGR52 (rw) register accessor: an alias for `Reg<GICD_ICFGR52_SPEC>`"]
pub type GICD_ICFGR52 = crate::Reg<gicd_icfgr52::GICD_ICFGR52_SPEC>;
#[doc = "Interrupt Configuration 208 - 223"]
pub mod gicd_icfgr52;
