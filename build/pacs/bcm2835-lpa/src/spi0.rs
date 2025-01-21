#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct RegisterBlock {
    cs: CS,
    fifo: FIFO,
    clk: CLK,
    dlen: DLEN,
    ltoh: LTOH,
    dc: DC,
}
impl RegisterBlock {
    #[doc = "0x00 - Control and Status"]
    #[inline(always)]
    pub const fn cs(&self) -> &CS {
        &self.cs
    }
    #[doc = "0x04 - FIFO access"]
    #[inline(always)]
    pub const fn fifo(&self) -> &FIFO {
        &self.fifo
    }
    #[doc = "0x08 - Clock divider"]
    #[inline(always)]
    pub const fn clk(&self) -> &CLK {
        &self.clk
    }
    #[doc = "0x0c - Data length"]
    #[inline(always)]
    pub const fn dlen(&self) -> &DLEN {
        &self.dlen
    }
    #[doc = "0x10 - LoSSI output hold delay"]
    #[inline(always)]
    pub const fn ltoh(&self) -> &LTOH {
        &self.ltoh
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn dc(&self) -> &DC {
        &self.dc
    }
}
#[doc = "CS (rw) register accessor: Control and Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs`]
module"]
pub type CS = crate::Reg<cs::CS_SPEC>;
#[doc = "Control and Status"]
pub mod cs;
#[doc = "FIFO (rw) register accessor: FIFO access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo`]
module"]
pub type FIFO = crate::Reg<fifo::FIFO_SPEC>;
#[doc = "FIFO access"]
pub mod fifo;
#[doc = "CLK (rw) register accessor: Clock divider\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk`]
module"]
pub type CLK = crate::Reg<clk::CLK_SPEC>;
#[doc = "Clock divider"]
pub mod clk;
#[doc = "DLEN (rw) register accessor: Data length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dlen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dlen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dlen`]
module"]
pub type DLEN = crate::Reg<dlen::DLEN_SPEC>;
#[doc = "Data length"]
pub mod dlen;
#[doc = "LTOH (rw) register accessor: LoSSI output hold delay\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ltoh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ltoh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ltoh`]
module"]
pub type LTOH = crate::Reg<ltoh::LTOH_SPEC>;
#[doc = "LoSSI output hold delay"]
pub mod ltoh;
#[doc = "DC (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dc`]
module"]
pub type DC = crate::Reg<dc::DC_SPEC>;
#[doc = ""]
pub mod dc;
