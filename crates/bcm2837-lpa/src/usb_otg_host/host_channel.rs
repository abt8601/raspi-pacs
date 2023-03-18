#[doc = r"Register block"]
#[repr(C)]
pub struct HOST_CHANNEL {
    #[doc = "0x00 - Characteristics register"]
    pub hcchar: HCCHAR,
    #[doc = "0x04 - Split control register"]
    pub hcsplt: HCSPLT,
    #[doc = "0x08 - Interrupt register"]
    pub hcint: HCINT,
    #[doc = "0x0c - Interrupt mask"]
    pub hcintmsk: HCINTMSK,
    #[doc = "0x10 - Transfer size"]
    pub hctsiz: HCTSIZ,
    #[doc = "0x14 - DMA address"]
    pub hcdma: HCDMA,
}
#[doc = "HCCHAR (rw) register accessor: an alias for `Reg<HCCHAR_SPEC>`"]
pub type HCCHAR = crate::Reg<hcchar::HCCHAR_SPEC>;
#[doc = "Characteristics register"]
pub mod hcchar;
#[doc = "HCSPLT (rw) register accessor: an alias for `Reg<HCSPLT_SPEC>`"]
pub type HCSPLT = crate::Reg<hcsplt::HCSPLT_SPEC>;
#[doc = "Split control register"]
pub mod hcsplt;
#[doc = "HCINT (rw) register accessor: an alias for `Reg<HCINT_SPEC>`"]
pub type HCINT = crate::Reg<hcint::HCINT_SPEC>;
#[doc = "Interrupt register"]
pub mod hcint;
#[doc = "HCINTMSK (rw) register accessor: an alias for `Reg<HCINTMSK_SPEC>`"]
pub type HCINTMSK = crate::Reg<hcintmsk::HCINTMSK_SPEC>;
#[doc = "Interrupt mask"]
pub mod hcintmsk;
#[doc = "HCTSIZ (rw) register accessor: an alias for `Reg<HCTSIZ_SPEC>`"]
pub type HCTSIZ = crate::Reg<hctsiz::HCTSIZ_SPEC>;
#[doc = "Transfer size"]
pub mod hctsiz;
#[doc = "HCDMA (rw) register accessor: an alias for `Reg<HCDMA_SPEC>`"]
pub type HCDMA = crate::Reg<hcdma::HCDMA_SPEC>;
#[doc = "DMA address"]
pub mod hcdma;
