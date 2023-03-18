#[doc = r"Register block"]
#[repr(C)]
pub struct GICD_ICPENDR {
    #[doc = "0x00 - Interrupt Clear-Pending"]
    pub gicd_icpendr0: GICD_ICPENDR0,
    #[doc = "0x04 - Interrupt Clear-Pending"]
    pub gicd_icpendr1: GICD_ICPENDR1,
    #[doc = "0x08 - Interrupt Clear-Pending"]
    pub gicd_icpendr2: GICD_ICPENDR2,
    #[doc = "0x0c - Interrupt Clear-Pending"]
    pub gicd_icpendr3: GICD_ICPENDR3,
    #[doc = "0x10 - Interrupt Clear-Pending"]
    pub gicd_icpendr4: GICD_ICPENDR4,
    #[doc = "0x14 - Interrupt Clear-Pending"]
    pub gicd_icpendr5: GICD_ICPENDR5,
    #[doc = "0x18 - Interrupt Clear-Pending"]
    pub gicd_icpendr6: GICD_ICPENDR6,
}
#[doc = "GICD_ICPENDR0 (rw) register accessor: an alias for `Reg<GICD_ICPENDR0_SPEC>`"]
pub type GICD_ICPENDR0 = crate::Reg<gicd_icpendr0::GICD_ICPENDR0_SPEC>;
#[doc = "Interrupt Clear-Pending"]
pub mod gicd_icpendr0;
#[doc = "GICD_ICPENDR1 (rw) register accessor: an alias for `Reg<GICD_ICPENDR1_SPEC>`"]
pub type GICD_ICPENDR1 = crate::Reg<gicd_icpendr1::GICD_ICPENDR1_SPEC>;
#[doc = "Interrupt Clear-Pending"]
pub mod gicd_icpendr1;
#[doc = "GICD_ICPENDR2 (rw) register accessor: an alias for `Reg<GICD_ICPENDR2_SPEC>`"]
pub type GICD_ICPENDR2 = crate::Reg<gicd_icpendr2::GICD_ICPENDR2_SPEC>;
#[doc = "Interrupt Clear-Pending"]
pub mod gicd_icpendr2;
#[doc = "GICD_ICPENDR3 (rw) register accessor: an alias for `Reg<GICD_ICPENDR3_SPEC>`"]
pub type GICD_ICPENDR3 = crate::Reg<gicd_icpendr3::GICD_ICPENDR3_SPEC>;
#[doc = "Interrupt Clear-Pending"]
pub mod gicd_icpendr3;
#[doc = "GICD_ICPENDR4 (rw) register accessor: an alias for `Reg<GICD_ICPENDR4_SPEC>`"]
pub type GICD_ICPENDR4 = crate::Reg<gicd_icpendr4::GICD_ICPENDR4_SPEC>;
#[doc = "Interrupt Clear-Pending"]
pub mod gicd_icpendr4;
#[doc = "GICD_ICPENDR5 (rw) register accessor: an alias for `Reg<GICD_ICPENDR5_SPEC>`"]
pub type GICD_ICPENDR5 = crate::Reg<gicd_icpendr5::GICD_ICPENDR5_SPEC>;
#[doc = "Interrupt Clear-Pending"]
pub mod gicd_icpendr5;
#[doc = "GICD_ICPENDR6 (rw) register accessor: an alias for `Reg<GICD_ICPENDR6_SPEC>`"]
pub type GICD_ICPENDR6 = crate::Reg<gicd_icpendr6::GICD_ICPENDR6_SPEC>;
#[doc = "Interrupt Clear-Pending"]
pub mod gicd_icpendr6;
