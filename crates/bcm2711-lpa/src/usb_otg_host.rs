#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct RegisterBlock {
    hcfg: HCFG,
    hfir: HFIR,
    hfnum: HFNUM,
    _reserved3: [u8; 0x04],
    hptxsts: HPTXSTS,
    haint: HAINT,
    haintmsk: HAINTMSK,
    _reserved6: [u8; 0x24],
    hprt: HPRT,
    _reserved7: [u8; 0xbc],
    host_channel0: HOST_CHANNEL,
    _reserved8: [u8; 0x08],
    host_channel1: HOST_CHANNEL,
    _reserved9: [u8; 0x08],
    host_channel2: HOST_CHANNEL,
    _reserved10: [u8; 0x08],
    host_channel3: HOST_CHANNEL,
    _reserved11: [u8; 0x08],
    host_channel4: HOST_CHANNEL,
    _reserved12: [u8; 0x08],
    host_channel5: HOST_CHANNEL,
    _reserved13: [u8; 0x08],
    host_channel6: HOST_CHANNEL,
    _reserved14: [u8; 0x08],
    host_channel7: HOST_CHANNEL,
    _reserved15: [u8; 0x08],
    host_channel8: HOST_CHANNEL,
    _reserved16: [u8; 0x08],
    host_channel9: HOST_CHANNEL,
    _reserved17: [u8; 0x08],
    host_channel10: HOST_CHANNEL,
    _reserved18: [u8; 0x08],
    host_channel11: HOST_CHANNEL,
}
impl RegisterBlock {
    #[doc = "0x00 - OTG_HS host configuration register"]
    #[inline(always)]
    pub const fn hcfg(&self) -> &HCFG {
        &self.hcfg
    }
    #[doc = "0x04 - OTG_HS Host frame interval register"]
    #[inline(always)]
    pub const fn hfir(&self) -> &HFIR {
        &self.hfir
    }
    #[doc = "0x08 - OTG_HS host frame number/frame time remaining register"]
    #[inline(always)]
    pub const fn hfnum(&self) -> &HFNUM {
        &self.hfnum
    }
    #[doc = "0x10 - Host periodic transmit FIFO/queue status register"]
    #[inline(always)]
    pub const fn hptxsts(&self) -> &HPTXSTS {
        &self.hptxsts
    }
    #[doc = "0x14 - OTG_HS Host all channels interrupt register"]
    #[inline(always)]
    pub const fn haint(&self) -> &HAINT {
        &self.haint
    }
    #[doc = "0x18 - OTG_HS host all channels interrupt mask register"]
    #[inline(always)]
    pub const fn haintmsk(&self) -> &HAINTMSK {
        &self.haintmsk
    }
    #[doc = "0x40 - OTG_HS host port control and status register"]
    #[inline(always)]
    pub const fn hprt(&self) -> &HPRT {
        &self.hprt
    }
    #[doc = "0x100..0x118 - Host channel 0"]
    #[inline(always)]
    pub const fn host_channel0(&self) -> &HOST_CHANNEL {
        &self.host_channel0
    }
    #[doc = "0x120..0x138 - Host channel 1"]
    #[inline(always)]
    pub const fn host_channel1(&self) -> &HOST_CHANNEL {
        &self.host_channel1
    }
    #[doc = "0x140..0x158 - Host channel 2"]
    #[inline(always)]
    pub const fn host_channel2(&self) -> &HOST_CHANNEL {
        &self.host_channel2
    }
    #[doc = "0x160..0x178 - Host channel 3"]
    #[inline(always)]
    pub const fn host_channel3(&self) -> &HOST_CHANNEL {
        &self.host_channel3
    }
    #[doc = "0x180..0x198 - Host channel 4"]
    #[inline(always)]
    pub const fn host_channel4(&self) -> &HOST_CHANNEL {
        &self.host_channel4
    }
    #[doc = "0x1a0..0x1b8 - Host channel 5"]
    #[inline(always)]
    pub const fn host_channel5(&self) -> &HOST_CHANNEL {
        &self.host_channel5
    }
    #[doc = "0x1c0..0x1d8 - Host channel 6"]
    #[inline(always)]
    pub const fn host_channel6(&self) -> &HOST_CHANNEL {
        &self.host_channel6
    }
    #[doc = "0x1e0..0x1f8 - Host channel 7"]
    #[inline(always)]
    pub const fn host_channel7(&self) -> &HOST_CHANNEL {
        &self.host_channel7
    }
    #[doc = "0x200..0x218 - Host channel 8"]
    #[inline(always)]
    pub const fn host_channel8(&self) -> &HOST_CHANNEL {
        &self.host_channel8
    }
    #[doc = "0x220..0x238 - Host channel 9"]
    #[inline(always)]
    pub const fn host_channel9(&self) -> &HOST_CHANNEL {
        &self.host_channel9
    }
    #[doc = "0x240..0x258 - Host channel 10"]
    #[inline(always)]
    pub const fn host_channel10(&self) -> &HOST_CHANNEL {
        &self.host_channel10
    }
    #[doc = "0x260..0x278 - Host channel 11"]
    #[inline(always)]
    pub const fn host_channel11(&self) -> &HOST_CHANNEL {
        &self.host_channel11
    }
}
#[doc = "HCFG (rw) register accessor: OTG_HS host configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcfg`]
module"]
pub type HCFG = crate::Reg<hcfg::HCFG_SPEC>;
#[doc = "OTG_HS host configuration register"]
pub mod hcfg;
#[doc = "HFIR (rw) register accessor: OTG_HS Host frame interval register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfir`]
module"]
pub type HFIR = crate::Reg<hfir::HFIR_SPEC>;
#[doc = "OTG_HS Host frame interval register"]
pub mod hfir;
#[doc = "HFNUM (r) register accessor: OTG_HS host frame number/frame time remaining register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfnum::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfnum`]
module"]
pub type HFNUM = crate::Reg<hfnum::HFNUM_SPEC>;
#[doc = "OTG_HS host frame number/frame time remaining register"]
pub mod hfnum;
#[doc = "HPTXSTS (rw) register accessor: Host periodic transmit FIFO/queue status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hptxsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hptxsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hptxsts`]
module"]
pub type HPTXSTS = crate::Reg<hptxsts::HPTXSTS_SPEC>;
#[doc = "Host periodic transmit FIFO/queue status register"]
pub mod hptxsts;
#[doc = "HAINT (r) register accessor: OTG_HS Host all channels interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`haint::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@haint`]
module"]
pub type HAINT = crate::Reg<haint::HAINT_SPEC>;
#[doc = "OTG_HS Host all channels interrupt register"]
pub mod haint;
#[doc = "HAINTMSK (rw) register accessor: OTG_HS host all channels interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`haintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`haintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@haintmsk`]
module"]
pub type HAINTMSK = crate::Reg<haintmsk::HAINTMSK_SPEC>;
#[doc = "OTG_HS host all channels interrupt mask register"]
pub mod haintmsk;
#[doc = "HPRT (rw) register accessor: OTG_HS host port control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hprt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hprt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hprt`]
module"]
pub type HPRT = crate::Reg<hprt::HPRT_SPEC>;
#[doc = "OTG_HS host port control and status register"]
pub mod hprt;
#[doc = "Host channel %s"]
pub use self::host_channel::HOST_CHANNEL;
#[doc = r"Cluster"]
#[doc = "Host channel %s"]
pub mod host_channel;
