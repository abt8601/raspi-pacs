#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct HOST_CHANNEL {
    hcchar: HCCHAR,
    hcsplt: HCSPLT,
    hcint: HCINT,
    hcintmsk: HCINTMSK,
    hctsiz: HCTSIZ,
    hcdma: HCDMA,
}
impl HOST_CHANNEL {
    #[doc = "0x00 - Characteristics register"]
    #[inline(always)]
    pub const fn hcchar(&self) -> &HCCHAR {
        &self.hcchar
    }
    #[doc = "0x04 - Split control register"]
    #[inline(always)]
    pub const fn hcsplt(&self) -> &HCSPLT {
        &self.hcsplt
    }
    #[doc = "0x08 - Interrupt register"]
    #[inline(always)]
    pub const fn hcint(&self) -> &HCINT {
        &self.hcint
    }
    #[doc = "0x0c - Interrupt mask"]
    #[inline(always)]
    pub const fn hcintmsk(&self) -> &HCINTMSK {
        &self.hcintmsk
    }
    #[doc = "0x10 - Transfer size"]
    #[inline(always)]
    pub const fn hctsiz(&self) -> &HCTSIZ {
        &self.hctsiz
    }
    #[doc = "0x14 - DMA address"]
    #[inline(always)]
    pub const fn hcdma(&self) -> &HCDMA {
        &self.hcdma
    }
}
#[doc = "HCCHAR (rw) register accessor: Characteristics register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar`]
module"]
pub type HCCHAR = crate::Reg<hcchar::HCCHAR_SPEC>;
#[doc = "Characteristics register"]
pub mod hcchar;
#[doc = "HCSPLT (rw) register accessor: Split control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcsplt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcsplt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt`]
module"]
pub type HCSPLT = crate::Reg<hcsplt::HCSPLT_SPEC>;
#[doc = "Split control register"]
pub mod hcsplt;
#[doc = "HCINT (rw) register accessor: Interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint`]
module"]
pub type HCINT = crate::Reg<hcint::HCINT_SPEC>;
#[doc = "Interrupt register"]
pub mod hcint;
#[doc = "HCINTMSK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk`]
module"]
pub type HCINTMSK = crate::Reg<hcintmsk::HCINTMSK_SPEC>;
#[doc = "Interrupt mask"]
pub mod hcintmsk;
#[doc = "HCTSIZ (rw) register accessor: Transfer size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz`]
module"]
pub type HCTSIZ = crate::Reg<hctsiz::HCTSIZ_SPEC>;
#[doc = "Transfer size"]
pub mod hctsiz;
#[doc = "HCDMA (rw) register accessor: DMA address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcdma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma`]
module"]
pub type HCDMA = crate::Reg<hcdma::HCDMA_SPEC>;
#[doc = "DMA address"]
pub mod hcdma;
