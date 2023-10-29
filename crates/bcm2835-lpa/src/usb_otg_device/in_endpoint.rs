#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
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
#[doc = "DIEPCTL0 (rw) register accessor: Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl0`]
module"]
pub type DIEPCTL0 = crate::Reg<diepctl0::DIEPCTL0_SPEC>;
#[doc = "Control"]
pub mod diepctl0;
#[doc = "DIEPINT (rw) register accessor: Interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint`]
module"]
pub type DIEPINT = crate::Reg<diepint::DIEPINT_SPEC>;
#[doc = "Interrupt"]
pub mod diepint;
#[doc = "DIEPTSIZ (rw) register accessor: Transfer size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz`]
module"]
pub type DIEPTSIZ = crate::Reg<dieptsiz::DIEPTSIZ_SPEC>;
#[doc = "Transfer size"]
pub mod dieptsiz;
#[doc = "DIEPDMA (rw) register accessor: DMA address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepdma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdma`]
module"]
pub type DIEPDMA = crate::Reg<diepdma::DIEPDMA_SPEC>;
#[doc = "DMA address"]
pub mod diepdma;
#[doc = "DTXFSTS (r) register accessor: Transmit FIFO status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts`]
module"]
pub type DTXFSTS = crate::Reg<dtxfsts::DTXFSTS_SPEC>;
#[doc = "Transmit FIFO status"]
pub mod dtxfsts;
