#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_HS host configuration register"]
    pub hcfg: HCFG,
    #[doc = "0x04 - OTG_HS Host frame interval register"]
    pub hfir: HFIR,
    #[doc = "0x08 - OTG_HS host frame number/frame time remaining register"]
    pub hfnum: HFNUM,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Host periodic transmit FIFO/queue status register"]
    pub hptxsts: HPTXSTS,
    #[doc = "0x14 - OTG_HS Host all channels interrupt register"]
    pub haint: HAINT,
    #[doc = "0x18 - OTG_HS host all channels interrupt mask register"]
    pub haintmsk: HAINTMSK,
    _reserved6: [u8; 0x24],
    #[doc = "0x40 - OTG_HS host port control and status register"]
    pub hprt: HPRT,
    _reserved7: [u8; 0xbc],
    #[doc = "0x100..0x118 - Host channel %s"]
    pub host_channel0: HOST_CHANNEL,
    _reserved8: [u8; 0x08],
    #[doc = "0x120..0x138 - Host channel %s"]
    pub host_channel1: HOST_CHANNEL,
    _reserved9: [u8; 0x08],
    #[doc = "0x140..0x158 - Host channel %s"]
    pub host_channel2: HOST_CHANNEL,
    _reserved10: [u8; 0x08],
    #[doc = "0x160..0x178 - Host channel %s"]
    pub host_channel3: HOST_CHANNEL,
    _reserved11: [u8; 0x08],
    #[doc = "0x180..0x198 - Host channel %s"]
    pub host_channel4: HOST_CHANNEL,
    _reserved12: [u8; 0x08],
    #[doc = "0x1a0..0x1b8 - Host channel %s"]
    pub host_channel5: HOST_CHANNEL,
    _reserved13: [u8; 0x08],
    #[doc = "0x1c0..0x1d8 - Host channel %s"]
    pub host_channel6: HOST_CHANNEL,
    _reserved14: [u8; 0x08],
    #[doc = "0x1e0..0x1f8 - Host channel %s"]
    pub host_channel7: HOST_CHANNEL,
    _reserved15: [u8; 0x08],
    #[doc = "0x200..0x218 - Host channel %s"]
    pub host_channel8: HOST_CHANNEL,
    _reserved16: [u8; 0x08],
    #[doc = "0x220..0x238 - Host channel %s"]
    pub host_channel9: HOST_CHANNEL,
    _reserved17: [u8; 0x08],
    #[doc = "0x240..0x258 - Host channel %s"]
    pub host_channel10: HOST_CHANNEL,
    _reserved18: [u8; 0x08],
    #[doc = "0x260..0x278 - Host channel %s"]
    pub host_channel11: HOST_CHANNEL,
}
#[doc = "HCFG (rw) register accessor: an alias for `Reg<HCFG_SPEC>`"]
pub type HCFG = crate::Reg<hcfg::HCFG_SPEC>;
#[doc = "OTG_HS host configuration register"]
pub mod hcfg;
#[doc = "HFIR (rw) register accessor: an alias for `Reg<HFIR_SPEC>`"]
pub type HFIR = crate::Reg<hfir::HFIR_SPEC>;
#[doc = "OTG_HS Host frame interval register"]
pub mod hfir;
#[doc = "HFNUM (r) register accessor: an alias for `Reg<HFNUM_SPEC>`"]
pub type HFNUM = crate::Reg<hfnum::HFNUM_SPEC>;
#[doc = "OTG_HS host frame number/frame time remaining register"]
pub mod hfnum;
#[doc = "HPTXSTS (rw) register accessor: an alias for `Reg<HPTXSTS_SPEC>`"]
pub type HPTXSTS = crate::Reg<hptxsts::HPTXSTS_SPEC>;
#[doc = "Host periodic transmit FIFO/queue status register"]
pub mod hptxsts;
#[doc = "HAINT (r) register accessor: an alias for `Reg<HAINT_SPEC>`"]
pub type HAINT = crate::Reg<haint::HAINT_SPEC>;
#[doc = "OTG_HS Host all channels interrupt register"]
pub mod haint;
#[doc = "HAINTMSK (rw) register accessor: an alias for `Reg<HAINTMSK_SPEC>`"]
pub type HAINTMSK = crate::Reg<haintmsk::HAINTMSK_SPEC>;
#[doc = "OTG_HS host all channels interrupt mask register"]
pub mod haintmsk;
#[doc = "HPRT (rw) register accessor: an alias for `Reg<HPRT_SPEC>`"]
pub type HPRT = crate::Reg<hprt::HPRT_SPEC>;
#[doc = "OTG_HS host port control and status register"]
pub mod hprt;
#[doc = "Host channel %s"]
pub use self::host_channel::HOST_CHANNEL;
#[doc = r"Cluster"]
#[doc = "Host channel %s"]
pub mod host_channel;
