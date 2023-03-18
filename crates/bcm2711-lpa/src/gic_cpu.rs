#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CPU Interface Control"]
    pub gicc_ctlr: GICC_CTLR,
    #[doc = "0x04 - Interrupt Priority Mask"]
    pub gicc_pmr: GICC_PMR,
    #[doc = "0x08 - Binary Point"]
    pub gicc_bpr: GICC_BPR,
    #[doc = "0x0c - Interrupt Acknowledge"]
    pub gicc_iar: GICC_IAR,
    #[doc = "0x10 - End of Interrupt"]
    pub gicc_eoir: GICC_EOIR,
    #[doc = "0x14 - Running Priority"]
    pub gicc_rpr: GICC_RPR,
    #[doc = "0x18 - Highest Priority Pending Interrupt"]
    pub gicc_hppir: GICC_HPPIR,
    #[doc = "0x1c - Aliased Binary Point"]
    pub gicc_abpr: GICC_ABPR,
    #[doc = "0x20 - Aliased Interrupt Acknowledge"]
    pub gicc_aiar: GICC_AIAR,
    #[doc = "0x24 - Aliased End of Interrupt"]
    pub gicc_aeoir: GICC_AEOIR,
    #[doc = "0x28 - Aliased Highest Priority Pending Interrupt"]
    pub gicc_ahppir: GICC_AHPPIR,
    _reserved11: [u8; 0xa4],
    #[doc = "0xd0 - Active Priority"]
    pub gicc_apr0: GICC_APR0,
    _reserved12: [u8; 0x0c],
    #[doc = "0xe0 - Non-Secure Active Priority"]
    pub gicc_nsapr0: GICC_NSAPR0,
    _reserved13: [u8; 0x18],
    #[doc = "0xfc - CPU Interface Identification Register"]
    pub gicc_iidr: GICC_IIDR,
    _reserved14: [u8; 0x0f00],
    #[doc = "0x1000 - Deactivate Interrupt"]
    pub gicc_dir: GICC_DIR,
}
#[doc = "GICC_CTLR (rw) register accessor: an alias for `Reg<GICC_CTLR_SPEC>`"]
pub type GICC_CTLR = crate::Reg<gicc_ctlr::GICC_CTLR_SPEC>;
#[doc = "CPU Interface Control"]
pub mod gicc_ctlr;
#[doc = "GICC_PMR (rw) register accessor: an alias for `Reg<GICC_PMR_SPEC>`"]
pub type GICC_PMR = crate::Reg<gicc_pmr::GICC_PMR_SPEC>;
#[doc = "Interrupt Priority Mask"]
pub mod gicc_pmr;
#[doc = "GICC_BPR (rw) register accessor: an alias for `Reg<GICC_BPR_SPEC>`"]
pub type GICC_BPR = crate::Reg<gicc_bpr::GICC_BPR_SPEC>;
#[doc = "Binary Point"]
pub mod gicc_bpr;
#[doc = "GICC_IAR (r) register accessor: an alias for `Reg<GICC_IAR_SPEC>`"]
pub type GICC_IAR = crate::Reg<gicc_iar::GICC_IAR_SPEC>;
#[doc = "Interrupt Acknowledge"]
pub mod gicc_iar;
#[doc = "GICC_EOIR (w) register accessor: an alias for `Reg<GICC_EOIR_SPEC>`"]
pub type GICC_EOIR = crate::Reg<gicc_eoir::GICC_EOIR_SPEC>;
#[doc = "End of Interrupt"]
pub mod gicc_eoir;
#[doc = "GICC_RPR (r) register accessor: an alias for `Reg<GICC_RPR_SPEC>`"]
pub type GICC_RPR = crate::Reg<gicc_rpr::GICC_RPR_SPEC>;
#[doc = "Running Priority"]
pub mod gicc_rpr;
#[doc = "GICC_HPPIR (rw) register accessor: an alias for `Reg<GICC_HPPIR_SPEC>`"]
pub type GICC_HPPIR = crate::Reg<gicc_hppir::GICC_HPPIR_SPEC>;
#[doc = "Highest Priority Pending Interrupt"]
pub mod gicc_hppir;
#[doc = "GICC_ABPR (rw) register accessor: an alias for `Reg<GICC_ABPR_SPEC>`"]
pub type GICC_ABPR = crate::Reg<gicc_abpr::GICC_ABPR_SPEC>;
#[doc = "Aliased Binary Point"]
pub mod gicc_abpr;
#[doc = "GICC_AIAR (r) register accessor: an alias for `Reg<GICC_AIAR_SPEC>`"]
pub type GICC_AIAR = crate::Reg<gicc_aiar::GICC_AIAR_SPEC>;
#[doc = "Aliased Interrupt Acknowledge"]
pub mod gicc_aiar;
#[doc = "GICC_AEOIR (w) register accessor: an alias for `Reg<GICC_AEOIR_SPEC>`"]
pub type GICC_AEOIR = crate::Reg<gicc_aeoir::GICC_AEOIR_SPEC>;
#[doc = "Aliased End of Interrupt"]
pub mod gicc_aeoir;
#[doc = "GICC_AHPPIR (r) register accessor: an alias for `Reg<GICC_AHPPIR_SPEC>`"]
pub type GICC_AHPPIR = crate::Reg<gicc_ahppir::GICC_AHPPIR_SPEC>;
#[doc = "Aliased Highest Priority Pending Interrupt"]
pub mod gicc_ahppir;
#[doc = "GICC_APR0 (rw) register accessor: an alias for `Reg<GICC_APR0_SPEC>`"]
pub type GICC_APR0 = crate::Reg<gicc_apr0::GICC_APR0_SPEC>;
#[doc = "Active Priority"]
pub mod gicc_apr0;
#[doc = "GICC_NSAPR0 (rw) register accessor: an alias for `Reg<GICC_NSAPR0_SPEC>`"]
pub type GICC_NSAPR0 = crate::Reg<gicc_nsapr0::GICC_NSAPR0_SPEC>;
#[doc = "Non-Secure Active Priority"]
pub mod gicc_nsapr0;
#[doc = "GICC_IIDR (rw) register accessor: an alias for `Reg<GICC_IIDR_SPEC>`"]
pub type GICC_IIDR = crate::Reg<gicc_iidr::GICC_IIDR_SPEC>;
#[doc = "CPU Interface Identification Register"]
pub mod gicc_iidr;
#[doc = "GICC_DIR (w) register accessor: an alias for `Reg<GICC_DIR_SPEC>`"]
pub type GICC_DIR = crate::Reg<gicc_dir::GICC_DIR_SPEC>;
#[doc = "Deactivate Interrupt"]
pub mod gicc_dir;
