#[doc = r"Register block"]
#[repr(C)]
pub struct OUT_ENDPOINT {
    #[doc = "0x00 - Control"]
    pub doepctl: DOEPCTL,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Interrupt"]
    pub doepint: DOEPINT,
    _reserved2: [u8; 0x04],
    #[doc = "0x10 - Transfer size"]
    pub doeptsiz: DOEPTSIZ,
    #[doc = "0x14 - DMA address"]
    pub doepdma: DOEPDMA,
}
#[doc = "DOEPCTL (rw) register accessor: an alias for `Reg<DOEPCTL_SPEC>`"]
pub type DOEPCTL = crate::Reg<doepctl::DOEPCTL_SPEC>;
#[doc = "Control"]
pub mod doepctl;
#[doc = "DOEPINT (rw) register accessor: an alias for `Reg<DOEPINT_SPEC>`"]
pub type DOEPINT = crate::Reg<doepint::DOEPINT_SPEC>;
#[doc = "Interrupt"]
pub mod doepint;
#[doc = "DOEPTSIZ (rw) register accessor: an alias for `Reg<DOEPTSIZ_SPEC>`"]
pub type DOEPTSIZ = crate::Reg<doeptsiz::DOEPTSIZ_SPEC>;
#[doc = "Transfer size"]
pub mod doeptsiz;
#[doc = "DOEPDMA (rw) register accessor: an alias for `Reg<DOEPDMA_SPEC>`"]
pub type DOEPDMA = crate::Reg<doepdma::DOEPDMA_SPEC>;
#[doc = "DMA address"]
pub mod doepdma;
