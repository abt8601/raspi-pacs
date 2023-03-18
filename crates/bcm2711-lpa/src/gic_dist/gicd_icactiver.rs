#[doc = r"Register block"]
#[repr(C)]
pub struct GICD_ICACTIVER {
    #[doc = "0x00 - Interrupt Clear-Active"]
    pub gicd_icactiver0: GICD_ICACTIVER0,
    #[doc = "0x04 - Interrupt Clear-Active"]
    pub gicd_icactiver1: GICD_ICACTIVER1,
    #[doc = "0x08 - Interrupt Clear-Active"]
    pub gicd_icactiver2: GICD_ICACTIVER2,
    #[doc = "0x0c - Interrupt Clear-Active"]
    pub gicd_icactiver3: GICD_ICACTIVER3,
    #[doc = "0x10 - Interrupt Clear-Active"]
    pub gicd_icactiver4: GICD_ICACTIVER4,
    #[doc = "0x14 - Interrupt Clear-Active"]
    pub gicd_icactiver5: GICD_ICACTIVER5,
    #[doc = "0x18 - Interrupt Clear-Active"]
    pub gicd_icactiver6: GICD_ICACTIVER6,
}
#[doc = "GICD_ICACTIVER0 (rw) register accessor: an alias for `Reg<GICD_ICACTIVER0_SPEC>`"]
pub type GICD_ICACTIVER0 = crate::Reg<gicd_icactiver0::GICD_ICACTIVER0_SPEC>;
#[doc = "Interrupt Clear-Active"]
pub mod gicd_icactiver0;
#[doc = "GICD_ICACTIVER1 (rw) register accessor: an alias for `Reg<GICD_ICACTIVER1_SPEC>`"]
pub type GICD_ICACTIVER1 = crate::Reg<gicd_icactiver1::GICD_ICACTIVER1_SPEC>;
#[doc = "Interrupt Clear-Active"]
pub mod gicd_icactiver1;
#[doc = "GICD_ICACTIVER2 (rw) register accessor: an alias for `Reg<GICD_ICACTIVER2_SPEC>`"]
pub type GICD_ICACTIVER2 = crate::Reg<gicd_icactiver2::GICD_ICACTIVER2_SPEC>;
#[doc = "Interrupt Clear-Active"]
pub mod gicd_icactiver2;
#[doc = "GICD_ICACTIVER3 (rw) register accessor: an alias for `Reg<GICD_ICACTIVER3_SPEC>`"]
pub type GICD_ICACTIVER3 = crate::Reg<gicd_icactiver3::GICD_ICACTIVER3_SPEC>;
#[doc = "Interrupt Clear-Active"]
pub mod gicd_icactiver3;
#[doc = "GICD_ICACTIVER4 (rw) register accessor: an alias for `Reg<GICD_ICACTIVER4_SPEC>`"]
pub type GICD_ICACTIVER4 = crate::Reg<gicd_icactiver4::GICD_ICACTIVER4_SPEC>;
#[doc = "Interrupt Clear-Active"]
pub mod gicd_icactiver4;
#[doc = "GICD_ICACTIVER5 (rw) register accessor: an alias for `Reg<GICD_ICACTIVER5_SPEC>`"]
pub type GICD_ICACTIVER5 = crate::Reg<gicd_icactiver5::GICD_ICACTIVER5_SPEC>;
#[doc = "Interrupt Clear-Active"]
pub mod gicd_icactiver5;
#[doc = "GICD_ICACTIVER6 (rw) register accessor: an alias for `Reg<GICD_ICACTIVER6_SPEC>`"]
pub type GICD_ICACTIVER6 = crate::Reg<gicd_icactiver6::GICD_ICACTIVER6_SPEC>;
#[doc = "Interrupt Clear-Active"]
pub mod gicd_icactiver6;
