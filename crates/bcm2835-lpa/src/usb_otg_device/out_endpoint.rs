#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
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
#[doc = "DOEPCTL (rw) register accessor: Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl`]
module"]
pub type DOEPCTL = crate::Reg<doepctl::DOEPCTL_SPEC>;
#[doc = "Control"]
pub mod doepctl;
#[doc = "DOEPINT (rw) register accessor: Interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint`]
module"]
pub type DOEPINT = crate::Reg<doepint::DOEPINT_SPEC>;
#[doc = "Interrupt"]
pub mod doepint;
#[doc = "DOEPTSIZ (rw) register accessor: Transfer size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz`]
module"]
pub type DOEPTSIZ = crate::Reg<doeptsiz::DOEPTSIZ_SPEC>;
#[doc = "Transfer size"]
pub mod doeptsiz;
#[doc = "DOEPDMA (rw) register accessor: DMA address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdma`]
module"]
pub type DOEPDMA = crate::Reg<doepdma::DOEPDMA_SPEC>;
#[doc = "DMA address"]
pub mod doepdma;
