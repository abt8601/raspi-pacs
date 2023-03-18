#[doc = r"Register block"]
#[repr(C)]
pub struct GICD_ISPENDR {
    #[doc = "0x00 - Interrupt Set-Pending"]
    pub gicd_ispendr0: GICD_ISPENDR0,
    #[doc = "0x04 - Interrupt Set-Pending"]
    pub gicd_ispendr1: GICD_ISPENDR1,
    #[doc = "0x08 - Interrupt Set-Pending"]
    pub gicd_ispendr2: GICD_ISPENDR2,
    #[doc = "0x0c - Interrupt Set-Pending"]
    pub gicd_ispendr3: GICD_ISPENDR3,
    #[doc = "0x10 - Interrupt Set-Pending"]
    pub gicd_ispendr4: GICD_ISPENDR4,
    #[doc = "0x14 - Interrupt Set-Pending"]
    pub gicd_ispendr5: GICD_ISPENDR5,
    #[doc = "0x18 - Interrupt Set-Pending"]
    pub gicd_ispendr6: GICD_ISPENDR6,
}
#[doc = "GICD_ISPENDR0 (rw) register accessor: an alias for `Reg<GICD_ISPENDR0_SPEC>`"]
pub type GICD_ISPENDR0 = crate::Reg<gicd_ispendr0::GICD_ISPENDR0_SPEC>;
#[doc = "Interrupt Set-Pending"]
pub mod gicd_ispendr0;
#[doc = "GICD_ISPENDR1 (rw) register accessor: an alias for `Reg<GICD_ISPENDR1_SPEC>`"]
pub type GICD_ISPENDR1 = crate::Reg<gicd_ispendr1::GICD_ISPENDR1_SPEC>;
#[doc = "Interrupt Set-Pending"]
pub mod gicd_ispendr1;
#[doc = "GICD_ISPENDR2 (rw) register accessor: an alias for `Reg<GICD_ISPENDR2_SPEC>`"]
pub type GICD_ISPENDR2 = crate::Reg<gicd_ispendr2::GICD_ISPENDR2_SPEC>;
#[doc = "Interrupt Set-Pending"]
pub mod gicd_ispendr2;
#[doc = "GICD_ISPENDR3 (rw) register accessor: an alias for `Reg<GICD_ISPENDR3_SPEC>`"]
pub type GICD_ISPENDR3 = crate::Reg<gicd_ispendr3::GICD_ISPENDR3_SPEC>;
#[doc = "Interrupt Set-Pending"]
pub mod gicd_ispendr3;
#[doc = "GICD_ISPENDR4 (rw) register accessor: an alias for `Reg<GICD_ISPENDR4_SPEC>`"]
pub type GICD_ISPENDR4 = crate::Reg<gicd_ispendr4::GICD_ISPENDR4_SPEC>;
#[doc = "Interrupt Set-Pending"]
pub mod gicd_ispendr4;
#[doc = "GICD_ISPENDR5 (rw) register accessor: an alias for `Reg<GICD_ISPENDR5_SPEC>`"]
pub type GICD_ISPENDR5 = crate::Reg<gicd_ispendr5::GICD_ISPENDR5_SPEC>;
#[doc = "Interrupt Set-Pending"]
pub mod gicd_ispendr5;
#[doc = "GICD_ISPENDR6 (rw) register accessor: an alias for `Reg<GICD_ISPENDR6_SPEC>`"]
pub type GICD_ISPENDR6 = crate::Reg<gicd_ispendr6::GICD_ISPENDR6_SPEC>;
#[doc = "Interrupt Set-Pending"]
pub mod gicd_ispendr6;
