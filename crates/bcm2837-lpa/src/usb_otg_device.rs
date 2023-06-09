#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_HS device configuration register"]
    pub dcfg: DCFG,
    #[doc = "0x04 - OTG_HS device control register"]
    pub dctl: DCTL,
    #[doc = "0x08 - OTG_HS device status register"]
    pub dsts: DSTS,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - OTG_HS device IN endpoint common interrupt mask register"]
    pub diepmsk: DIEPMSK,
    #[doc = "0x14 - OTG_HS device OUT endpoint common interrupt mask register"]
    pub doepmsk: DOEPMSK,
    #[doc = "0x18 - OTG_HS device all endpoints interrupt register"]
    pub daint: DAINT,
    #[doc = "0x1c - OTG_HS all endpoints interrupt mask register"]
    pub daintmsk: DAINTMSK,
    _reserved7: [u8; 0x08],
    #[doc = "0x28 - OTG_HS device VBUS discharge time register"]
    pub dvbusdis: DVBUSDIS,
    #[doc = "0x2c - OTG_HS device VBUS pulsing time register"]
    pub dvbuspulse: DVBUSPULSE,
    #[doc = "0x30 - OTG_HS Device threshold control register"]
    pub dthrctl: DTHRCTL,
    #[doc = "0x34 - OTG_HS device IN endpoint FIFO empty interrupt mask register"]
    pub diepempmsk: DIEPEMPMSK,
    #[doc = "0x38 - OTG_HS device each endpoint interrupt register"]
    pub deachint: DEACHINT,
    #[doc = "0x3c - OTG_HS device each endpoint interrupt register mask"]
    pub deachintmsk: DEACHINTMSK,
    #[doc = "0x40 - OTG_HS device each in endpoint-1 interrupt register"]
    pub diepeachmsk1: DIEPEACHMSK1,
    _reserved14: [u8; 0x3c],
    #[doc = "0x80 - OTG_HS device each OUT endpoint-1 interrupt register"]
    pub doepeachmsk1: DOEPEACHMSK1,
    _reserved15: [u8; 0x7c],
    #[doc = "0x100..0x11c - IN Endpoint %s"]
    pub in_endpoint0: IN_ENDPOINT,
    _reserved16: [u8; 0x04],
    #[doc = "0x120..0x13c - IN Endpoint %s"]
    pub in_endpoint1: IN_ENDPOINT,
    _reserved17: [u8; 0x04],
    #[doc = "0x140..0x15c - IN Endpoint %s"]
    pub in_endpoint2: IN_ENDPOINT,
    _reserved18: [u8; 0x04],
    #[doc = "0x160..0x17c - IN Endpoint %s"]
    pub in_endpoint3: IN_ENDPOINT,
    _reserved19: [u8; 0x04],
    #[doc = "0x180..0x19c - IN Endpoint %s"]
    pub in_endpoint4: IN_ENDPOINT,
    _reserved20: [u8; 0x04],
    #[doc = "0x1a0..0x1bc - IN Endpoint %s"]
    pub in_endpoint5: IN_ENDPOINT,
    _reserved21: [u8; 0x04],
    #[doc = "0x1c0..0x1dc - IN Endpoint %s"]
    pub in_endpoint6: IN_ENDPOINT,
    _reserved22: [u8; 0x04],
    #[doc = "0x1e0..0x1fc - IN Endpoint %s"]
    pub in_endpoint7: IN_ENDPOINT,
    _reserved23: [u8; 0x04],
    #[doc = "0x200..0x21c - IN Endpoint %s"]
    pub in_endpoint8: IN_ENDPOINT,
    _reserved24: [u8; 0x04],
    #[doc = "0x220..0x23c - IN Endpoint %s"]
    pub in_endpoint9: IN_ENDPOINT,
    _reserved25: [u8; 0x04],
    #[doc = "0x240..0x25c - IN Endpoint %s"]
    pub in_endpoint10: IN_ENDPOINT,
    _reserved26: [u8; 0x04],
    #[doc = "0x260..0x27c - IN Endpoint %s"]
    pub in_endpoint11: IN_ENDPOINT,
    _reserved27: [u8; 0x84],
    #[doc = "0x300..0x318 - OUT Endpoint %s"]
    pub out_endpoint0: OUT_ENDPOINT,
    _reserved28: [u8; 0x08],
    #[doc = "0x320..0x338 - OUT Endpoint %s"]
    pub out_endpoint1: OUT_ENDPOINT,
    _reserved29: [u8; 0x08],
    #[doc = "0x340..0x358 - OUT Endpoint %s"]
    pub out_endpoint2: OUT_ENDPOINT,
    _reserved30: [u8; 0x08],
    #[doc = "0x360..0x378 - OUT Endpoint %s"]
    pub out_endpoint3: OUT_ENDPOINT,
    _reserved31: [u8; 0x08],
    #[doc = "0x380..0x398 - OUT Endpoint %s"]
    pub out_endpoint4: OUT_ENDPOINT,
    _reserved32: [u8; 0x08],
    #[doc = "0x3a0..0x3b8 - OUT Endpoint %s"]
    pub out_endpoint5: OUT_ENDPOINT,
    _reserved33: [u8; 0x08],
    #[doc = "0x3c0..0x3d8 - OUT Endpoint %s"]
    pub out_endpoint6: OUT_ENDPOINT,
    _reserved34: [u8; 0x08],
    #[doc = "0x3e0..0x3f8 - OUT Endpoint %s"]
    pub out_endpoint7: OUT_ENDPOINT,
    _reserved35: [u8; 0x08],
    #[doc = "0x400..0x418 - OUT Endpoint %s"]
    pub out_endpoint8: OUT_ENDPOINT,
    _reserved36: [u8; 0x08],
    #[doc = "0x420..0x438 - OUT Endpoint %s"]
    pub out_endpoint9: OUT_ENDPOINT,
    _reserved37: [u8; 0x08],
    #[doc = "0x440..0x458 - OUT Endpoint %s"]
    pub out_endpoint10: OUT_ENDPOINT,
    _reserved38: [u8; 0x08],
    #[doc = "0x460..0x478 - OUT Endpoint %s"]
    pub out_endpoint11: OUT_ENDPOINT,
}
#[doc = "DCFG (rw) register accessor: an alias for `Reg<DCFG_SPEC>`"]
pub type DCFG = crate::Reg<dcfg::DCFG_SPEC>;
#[doc = "OTG_HS device configuration register"]
pub mod dcfg;
#[doc = "DCTL (rw) register accessor: an alias for `Reg<DCTL_SPEC>`"]
pub type DCTL = crate::Reg<dctl::DCTL_SPEC>;
#[doc = "OTG_HS device control register"]
pub mod dctl;
#[doc = "DSTS (r) register accessor: an alias for `Reg<DSTS_SPEC>`"]
pub type DSTS = crate::Reg<dsts::DSTS_SPEC>;
#[doc = "OTG_HS device status register"]
pub mod dsts;
#[doc = "DIEPMSK (rw) register accessor: an alias for `Reg<DIEPMSK_SPEC>`"]
pub type DIEPMSK = crate::Reg<diepmsk::DIEPMSK_SPEC>;
#[doc = "OTG_HS device IN endpoint common interrupt mask register"]
pub mod diepmsk;
#[doc = "DOEPMSK (rw) register accessor: an alias for `Reg<DOEPMSK_SPEC>`"]
pub type DOEPMSK = crate::Reg<doepmsk::DOEPMSK_SPEC>;
#[doc = "OTG_HS device OUT endpoint common interrupt mask register"]
pub mod doepmsk;
#[doc = "DAINT (r) register accessor: an alias for `Reg<DAINT_SPEC>`"]
pub type DAINT = crate::Reg<daint::DAINT_SPEC>;
#[doc = "OTG_HS device all endpoints interrupt register"]
pub mod daint;
#[doc = "DAINTMSK (rw) register accessor: an alias for `Reg<DAINTMSK_SPEC>`"]
pub type DAINTMSK = crate::Reg<daintmsk::DAINTMSK_SPEC>;
#[doc = "OTG_HS all endpoints interrupt mask register"]
pub mod daintmsk;
#[doc = "DVBUSDIS (rw) register accessor: an alias for `Reg<DVBUSDIS_SPEC>`"]
pub type DVBUSDIS = crate::Reg<dvbusdis::DVBUSDIS_SPEC>;
#[doc = "OTG_HS device VBUS discharge time register"]
pub mod dvbusdis;
#[doc = "DVBUSPULSE (rw) register accessor: an alias for `Reg<DVBUSPULSE_SPEC>`"]
pub type DVBUSPULSE = crate::Reg<dvbuspulse::DVBUSPULSE_SPEC>;
#[doc = "OTG_HS device VBUS pulsing time register"]
pub mod dvbuspulse;
#[doc = "DTHRCTL (rw) register accessor: an alias for `Reg<DTHRCTL_SPEC>`"]
pub type DTHRCTL = crate::Reg<dthrctl::DTHRCTL_SPEC>;
#[doc = "OTG_HS Device threshold control register"]
pub mod dthrctl;
#[doc = "DIEPEMPMSK (rw) register accessor: an alias for `Reg<DIEPEMPMSK_SPEC>`"]
pub type DIEPEMPMSK = crate::Reg<diepempmsk::DIEPEMPMSK_SPEC>;
#[doc = "OTG_HS device IN endpoint FIFO empty interrupt mask register"]
pub mod diepempmsk;
#[doc = "DEACHINT (rw) register accessor: an alias for `Reg<DEACHINT_SPEC>`"]
pub type DEACHINT = crate::Reg<deachint::DEACHINT_SPEC>;
#[doc = "OTG_HS device each endpoint interrupt register"]
pub mod deachint;
#[doc = "DEACHINTMSK (rw) register accessor: an alias for `Reg<DEACHINTMSK_SPEC>`"]
pub type DEACHINTMSK = crate::Reg<deachintmsk::DEACHINTMSK_SPEC>;
#[doc = "OTG_HS device each endpoint interrupt register mask"]
pub mod deachintmsk;
#[doc = "DIEPEACHMSK1 (rw) register accessor: an alias for `Reg<DIEPEACHMSK1_SPEC>`"]
pub type DIEPEACHMSK1 = crate::Reg<diepeachmsk1::DIEPEACHMSK1_SPEC>;
#[doc = "OTG_HS device each in endpoint-1 interrupt register"]
pub mod diepeachmsk1;
#[doc = "DOEPEACHMSK1 (rw) register accessor: an alias for `Reg<DOEPEACHMSK1_SPEC>`"]
pub type DOEPEACHMSK1 = crate::Reg<doepeachmsk1::DOEPEACHMSK1_SPEC>;
#[doc = "OTG_HS device each OUT endpoint-1 interrupt register"]
pub mod doepeachmsk1;
#[doc = "IN Endpoint %s"]
pub use self::in_endpoint::IN_ENDPOINT;
#[doc = r"Cluster"]
#[doc = "IN Endpoint %s"]
pub mod in_endpoint;
#[doc = "OUT Endpoint %s"]
pub use self::out_endpoint::OUT_ENDPOINT;
#[doc = r"Cluster"]
#[doc = "OUT Endpoint %s"]
pub mod out_endpoint;
