#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctl: CTL,
    #[doc = "0x04 - Status"]
    pub sta: STA,
    #[doc = "0x08 - DMA control"]
    pub dmac: DMAC,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Range for channel 1"]
    pub rng1: RNG1,
    #[doc = "0x14 - Channel 1 data"]
    pub dat1: DAT1,
    #[doc = "0x18 - FIFO input"]
    pub fif1: FIF1,
    _reserved6: [u8; 0x04],
    #[doc = "0x20 - Range for channel 2"]
    pub rng2: RNG2,
    #[doc = "0x24 - Channel 2 data"]
    pub dat2: DAT2,
}
#[doc = "CTL (rw) register accessor: Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control"]
pub mod ctl;
#[doc = "STA (rw) register accessor: Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sta`]
module"]
pub type STA = crate::Reg<sta::STA_SPEC>;
#[doc = "Status"]
pub mod sta;
#[doc = "DMAC (rw) register accessor: DMA control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmac::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac`]
module"]
pub type DMAC = crate::Reg<dmac::DMAC_SPEC>;
#[doc = "DMA control"]
pub mod dmac;
#[doc = "RNG1 (rw) register accessor: Range for channel 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rng1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rng1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng1`]
module"]
pub type RNG1 = crate::Reg<rng1::RNG1_SPEC>;
#[doc = "Range for channel 1"]
pub mod rng1;
#[doc = "DAT1 (rw) register accessor: Channel 1 data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dat1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dat1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dat1`]
module"]
pub type DAT1 = crate::Reg<dat1::DAT1_SPEC>;
#[doc = "Channel 1 data"]
pub mod dat1;
#[doc = "FIF1 (w) register accessor: FIFO input\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fif1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fif1`]
module"]
pub type FIF1 = crate::Reg<fif1::FIF1_SPEC>;
#[doc = "FIFO input"]
pub mod fif1;
#[doc = "RNG2 (rw) register accessor: Range for channel 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rng2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rng2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng2`]
module"]
pub type RNG2 = crate::Reg<rng2::RNG2_SPEC>;
#[doc = "Range for channel 2"]
pub mod rng2;
#[doc = "DAT2 (rw) register accessor: Channel 2 data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dat2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dat2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dat2`]
module"]
pub type DAT2 = crate::Reg<dat2::DAT2_SPEC>;
#[doc = "Channel 2 data"]
pub mod dat2;
