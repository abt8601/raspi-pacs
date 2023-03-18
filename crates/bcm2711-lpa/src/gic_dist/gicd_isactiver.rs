#[doc = r"Register block"]
#[repr(C)]
pub struct GICD_ISACTIVER {
    #[doc = "0x00 - Interrupt Set-Active"]
    pub gicd_isactiver0: GICD_ISACTIVER0,
    #[doc = "0x04 - Interrupt Set-Active"]
    pub gicd_isactiver1: GICD_ISACTIVER1,
    #[doc = "0x08 - Interrupt Set-Active"]
    pub gicd_isactiver2: GICD_ISACTIVER2,
    #[doc = "0x0c - Interrupt Set-Active"]
    pub gicd_isactiver3: GICD_ISACTIVER3,
    #[doc = "0x10 - Interrupt Set-Active"]
    pub gicd_isactiver4: GICD_ISACTIVER4,
    #[doc = "0x14 - Interrupt Set-Active"]
    pub gicd_isactiver5: GICD_ISACTIVER5,
    #[doc = "0x18 - Interrupt Set-Active"]
    pub gicd_isactiver6: GICD_ISACTIVER6,
}
#[doc = "GICD_ISACTIVER0 (rw) register accessor: an alias for `Reg<GICD_ISACTIVER0_SPEC>`"]
pub type GICD_ISACTIVER0 = crate::Reg<gicd_isactiver0::GICD_ISACTIVER0_SPEC>;
#[doc = "Interrupt Set-Active"]
pub mod gicd_isactiver0;
#[doc = "GICD_ISACTIVER1 (rw) register accessor: an alias for `Reg<GICD_ISACTIVER1_SPEC>`"]
pub type GICD_ISACTIVER1 = crate::Reg<gicd_isactiver1::GICD_ISACTIVER1_SPEC>;
#[doc = "Interrupt Set-Active"]
pub mod gicd_isactiver1;
#[doc = "GICD_ISACTIVER2 (rw) register accessor: an alias for `Reg<GICD_ISACTIVER2_SPEC>`"]
pub type GICD_ISACTIVER2 = crate::Reg<gicd_isactiver2::GICD_ISACTIVER2_SPEC>;
#[doc = "Interrupt Set-Active"]
pub mod gicd_isactiver2;
#[doc = "GICD_ISACTIVER3 (rw) register accessor: an alias for `Reg<GICD_ISACTIVER3_SPEC>`"]
pub type GICD_ISACTIVER3 = crate::Reg<gicd_isactiver3::GICD_ISACTIVER3_SPEC>;
#[doc = "Interrupt Set-Active"]
pub mod gicd_isactiver3;
#[doc = "GICD_ISACTIVER4 (rw) register accessor: an alias for `Reg<GICD_ISACTIVER4_SPEC>`"]
pub type GICD_ISACTIVER4 = crate::Reg<gicd_isactiver4::GICD_ISACTIVER4_SPEC>;
#[doc = "Interrupt Set-Active"]
pub mod gicd_isactiver4;
#[doc = "GICD_ISACTIVER5 (rw) register accessor: an alias for `Reg<GICD_ISACTIVER5_SPEC>`"]
pub type GICD_ISACTIVER5 = crate::Reg<gicd_isactiver5::GICD_ISACTIVER5_SPEC>;
#[doc = "Interrupt Set-Active"]
pub mod gicd_isactiver5;
#[doc = "GICD_ISACTIVER6 (rw) register accessor: an alias for `Reg<GICD_ISACTIVER6_SPEC>`"]
pub type GICD_ISACTIVER6 = crate::Reg<gicd_isactiver6::GICD_ISACTIVER6_SPEC>;
#[doc = "Interrupt Set-Active"]
pub mod gicd_isactiver6;
