#[doc = r"Register block"]
#[repr(C)]
pub struct IN_ENDPOINT {
    #[doc = "0x00 - Control"]
    pub diepctl0: DIEPCTL0,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Interrupt"]
    pub diepint: DIEPINT,
    _reserved2: [u8; 0x04],
    #[doc = "0x10 - Transfer size"]
    pub dieptsiz: DIEPTSIZ,
    #[doc = "0x14 - DMA address"]
    pub diepdma: DIEPDMA,
    #[doc = "0x18 - Transmit FIFO status"]
    pub dtxfsts: DTXFSTS,
}
#[doc = "DIEPCTL0 (rw) register accessor: an alias for `Reg<DIEPCTL0_SPEC>`"]
pub type DIEPCTL0 = crate::Reg<diepctl0::DIEPCTL0_SPEC>;
#[doc = "Control"]
pub mod diepctl0;
#[doc = "DIEPINT (rw) register accessor: an alias for `Reg<DIEPINT_SPEC>`"]
pub type DIEPINT = crate::Reg<diepint::DIEPINT_SPEC>;
#[doc = "Interrupt"]
pub mod diepint;
#[doc = "DIEPTSIZ (rw) register accessor: an alias for `Reg<DIEPTSIZ_SPEC>`"]
pub type DIEPTSIZ = crate::Reg<dieptsiz::DIEPTSIZ_SPEC>;
#[doc = "Transfer size"]
pub mod dieptsiz;
#[doc = "DIEPDMA (rw) register accessor: an alias for `Reg<DIEPDMA_SPEC>`"]
pub type DIEPDMA = crate::Reg<diepdma::DIEPDMA_SPEC>;
#[doc = "DMA address"]
pub mod diepdma;
#[doc = "DTXFSTS (r) register accessor: an alias for `Reg<DTXFSTS_SPEC>`"]
pub type DTXFSTS = crate::Reg<dtxfsts::DTXFSTS_SPEC>;
#[doc = "Transmit FIFO status"]
pub mod dtxfsts;
