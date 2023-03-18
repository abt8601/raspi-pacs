#[doc = r"Register block"]
#[repr(C)]
pub struct GICD_ISENABLER {
    #[doc = "0x00 - Interrupt Set-Enable"]
    pub gicd_isenabler0: GICD_ISENABLER0,
    #[doc = "0x04 - Interrupt Set-Enable"]
    pub gicd_isenabler1: GICD_ISENABLER1,
    #[doc = "0x08 - Interrupt Set-Enable"]
    pub gicd_isenabler2: GICD_ISENABLER2,
    #[doc = "0x0c - Interrupt Set-Enable"]
    pub gicd_isenabler3: GICD_ISENABLER3,
    #[doc = "0x10 - Interrupt Set-Enable"]
    pub gicd_isenabler4: GICD_ISENABLER4,
    #[doc = "0x14 - Interrupt Set-Enable"]
    pub gicd_isenabler5: GICD_ISENABLER5,
    #[doc = "0x18 - Interrupt Set-Enable"]
    pub gicd_isenabler6: GICD_ISENABLER6,
}
#[doc = "GICD_ISENABLER0 (rw) register accessor: an alias for `Reg<GICD_ISENABLER0_SPEC>`"]
pub type GICD_ISENABLER0 = crate::Reg<gicd_isenabler0::GICD_ISENABLER0_SPEC>;
#[doc = "Interrupt Set-Enable"]
pub mod gicd_isenabler0;
#[doc = "GICD_ISENABLER1 (rw) register accessor: an alias for `Reg<GICD_ISENABLER1_SPEC>`"]
pub type GICD_ISENABLER1 = crate::Reg<gicd_isenabler1::GICD_ISENABLER1_SPEC>;
#[doc = "Interrupt Set-Enable"]
pub mod gicd_isenabler1;
#[doc = "GICD_ISENABLER2 (rw) register accessor: an alias for `Reg<GICD_ISENABLER2_SPEC>`"]
pub type GICD_ISENABLER2 = crate::Reg<gicd_isenabler2::GICD_ISENABLER2_SPEC>;
#[doc = "Interrupt Set-Enable"]
pub mod gicd_isenabler2;
#[doc = "GICD_ISENABLER3 (rw) register accessor: an alias for `Reg<GICD_ISENABLER3_SPEC>`"]
pub type GICD_ISENABLER3 = crate::Reg<gicd_isenabler3::GICD_ISENABLER3_SPEC>;
#[doc = "Interrupt Set-Enable"]
pub mod gicd_isenabler3;
#[doc = "GICD_ISENABLER4 (rw) register accessor: an alias for `Reg<GICD_ISENABLER4_SPEC>`"]
pub type GICD_ISENABLER4 = crate::Reg<gicd_isenabler4::GICD_ISENABLER4_SPEC>;
#[doc = "Interrupt Set-Enable"]
pub mod gicd_isenabler4;
#[doc = "GICD_ISENABLER5 (rw) register accessor: an alias for `Reg<GICD_ISENABLER5_SPEC>`"]
pub type GICD_ISENABLER5 = crate::Reg<gicd_isenabler5::GICD_ISENABLER5_SPEC>;
#[doc = "Interrupt Set-Enable"]
pub mod gicd_isenabler5;
#[doc = "GICD_ISENABLER6 (rw) register accessor: an alias for `Reg<GICD_ISENABLER6_SPEC>`"]
pub type GICD_ISENABLER6 = crate::Reg<gicd_isenabler6::GICD_ISENABLER6_SPEC>;
#[doc = "Interrupt Set-Enable"]
pub mod gicd_isenabler6;
