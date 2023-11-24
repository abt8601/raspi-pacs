#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct RegisterBlock {
    dcfg: DCFG,
    dctl: DCTL,
    dsts: DSTS,
    _reserved3: [u8; 0x04],
    diepmsk: DIEPMSK,
    doepmsk: DOEPMSK,
    daint: DAINT,
    daintmsk: DAINTMSK,
    _reserved7: [u8; 0x08],
    dvbusdis: DVBUSDIS,
    dvbuspulse: DVBUSPULSE,
    dthrctl: DTHRCTL,
    diepempmsk: DIEPEMPMSK,
    deachint: DEACHINT,
    deachintmsk: DEACHINTMSK,
    diepeachmsk1: DIEPEACHMSK1,
    _reserved14: [u8; 0x3c],
    doepeachmsk1: DOEPEACHMSK1,
    _reserved15: [u8; 0x7c],
    in_endpoint0: IN_ENDPOINT,
    _reserved16: [u8; 0x04],
    in_endpoint1: IN_ENDPOINT,
    _reserved17: [u8; 0x04],
    in_endpoint2: IN_ENDPOINT,
    _reserved18: [u8; 0x04],
    in_endpoint3: IN_ENDPOINT,
    _reserved19: [u8; 0x04],
    in_endpoint4: IN_ENDPOINT,
    _reserved20: [u8; 0x04],
    in_endpoint5: IN_ENDPOINT,
    _reserved21: [u8; 0x04],
    in_endpoint6: IN_ENDPOINT,
    _reserved22: [u8; 0x04],
    in_endpoint7: IN_ENDPOINT,
    _reserved23: [u8; 0x04],
    in_endpoint8: IN_ENDPOINT,
    _reserved24: [u8; 0x04],
    in_endpoint9: IN_ENDPOINT,
    _reserved25: [u8; 0x04],
    in_endpoint10: IN_ENDPOINT,
    _reserved26: [u8; 0x04],
    in_endpoint11: IN_ENDPOINT,
    _reserved27: [u8; 0x84],
    out_endpoint0: OUT_ENDPOINT,
    _reserved28: [u8; 0x08],
    out_endpoint1: OUT_ENDPOINT,
    _reserved29: [u8; 0x08],
    out_endpoint2: OUT_ENDPOINT,
    _reserved30: [u8; 0x08],
    out_endpoint3: OUT_ENDPOINT,
    _reserved31: [u8; 0x08],
    out_endpoint4: OUT_ENDPOINT,
    _reserved32: [u8; 0x08],
    out_endpoint5: OUT_ENDPOINT,
    _reserved33: [u8; 0x08],
    out_endpoint6: OUT_ENDPOINT,
    _reserved34: [u8; 0x08],
    out_endpoint7: OUT_ENDPOINT,
    _reserved35: [u8; 0x08],
    out_endpoint8: OUT_ENDPOINT,
    _reserved36: [u8; 0x08],
    out_endpoint9: OUT_ENDPOINT,
    _reserved37: [u8; 0x08],
    out_endpoint10: OUT_ENDPOINT,
    _reserved38: [u8; 0x08],
    out_endpoint11: OUT_ENDPOINT,
}
impl RegisterBlock {
    #[doc = "0x00 - OTG_HS device configuration register"]
    #[inline(always)]
    pub const fn dcfg(&self) -> &DCFG {
        &self.dcfg
    }
    #[doc = "0x04 - OTG_HS device control register"]
    #[inline(always)]
    pub const fn dctl(&self) -> &DCTL {
        &self.dctl
    }
    #[doc = "0x08 - OTG_HS device status register"]
    #[inline(always)]
    pub const fn dsts(&self) -> &DSTS {
        &self.dsts
    }
    #[doc = "0x10 - OTG_HS device IN endpoint common interrupt mask register"]
    #[inline(always)]
    pub const fn diepmsk(&self) -> &DIEPMSK {
        &self.diepmsk
    }
    #[doc = "0x14 - OTG_HS device OUT endpoint common interrupt mask register"]
    #[inline(always)]
    pub const fn doepmsk(&self) -> &DOEPMSK {
        &self.doepmsk
    }
    #[doc = "0x18 - OTG_HS device all endpoints interrupt register"]
    #[inline(always)]
    pub const fn daint(&self) -> &DAINT {
        &self.daint
    }
    #[doc = "0x1c - OTG_HS all endpoints interrupt mask register"]
    #[inline(always)]
    pub const fn daintmsk(&self) -> &DAINTMSK {
        &self.daintmsk
    }
    #[doc = "0x28 - OTG_HS device VBUS discharge time register"]
    #[inline(always)]
    pub const fn dvbusdis(&self) -> &DVBUSDIS {
        &self.dvbusdis
    }
    #[doc = "0x2c - OTG_HS device VBUS pulsing time register"]
    #[inline(always)]
    pub const fn dvbuspulse(&self) -> &DVBUSPULSE {
        &self.dvbuspulse
    }
    #[doc = "0x30 - OTG_HS Device threshold control register"]
    #[inline(always)]
    pub const fn dthrctl(&self) -> &DTHRCTL {
        &self.dthrctl
    }
    #[doc = "0x34 - OTG_HS device IN endpoint FIFO empty interrupt mask register"]
    #[inline(always)]
    pub const fn diepempmsk(&self) -> &DIEPEMPMSK {
        &self.diepempmsk
    }
    #[doc = "0x38 - OTG_HS device each endpoint interrupt register"]
    #[inline(always)]
    pub const fn deachint(&self) -> &DEACHINT {
        &self.deachint
    }
    #[doc = "0x3c - OTG_HS device each endpoint interrupt register mask"]
    #[inline(always)]
    pub const fn deachintmsk(&self) -> &DEACHINTMSK {
        &self.deachintmsk
    }
    #[doc = "0x40 - OTG_HS device each in endpoint-1 interrupt register"]
    #[inline(always)]
    pub const fn diepeachmsk1(&self) -> &DIEPEACHMSK1 {
        &self.diepeachmsk1
    }
    #[doc = "0x80 - OTG_HS device each OUT endpoint-1 interrupt register"]
    #[inline(always)]
    pub const fn doepeachmsk1(&self) -> &DOEPEACHMSK1 {
        &self.doepeachmsk1
    }
    #[doc = "0x100..0x11c - IN Endpoint 0"]
    #[inline(always)]
    pub const fn in_endpoint0(&self) -> &IN_ENDPOINT {
        &self.in_endpoint0
    }
    #[doc = "0x120..0x13c - IN Endpoint 1"]
    #[inline(always)]
    pub const fn in_endpoint1(&self) -> &IN_ENDPOINT {
        &self.in_endpoint1
    }
    #[doc = "0x140..0x15c - IN Endpoint 2"]
    #[inline(always)]
    pub const fn in_endpoint2(&self) -> &IN_ENDPOINT {
        &self.in_endpoint2
    }
    #[doc = "0x160..0x17c - IN Endpoint 3"]
    #[inline(always)]
    pub const fn in_endpoint3(&self) -> &IN_ENDPOINT {
        &self.in_endpoint3
    }
    #[doc = "0x180..0x19c - IN Endpoint 4"]
    #[inline(always)]
    pub const fn in_endpoint4(&self) -> &IN_ENDPOINT {
        &self.in_endpoint4
    }
    #[doc = "0x1a0..0x1bc - IN Endpoint 5"]
    #[inline(always)]
    pub const fn in_endpoint5(&self) -> &IN_ENDPOINT {
        &self.in_endpoint5
    }
    #[doc = "0x1c0..0x1dc - IN Endpoint 6"]
    #[inline(always)]
    pub const fn in_endpoint6(&self) -> &IN_ENDPOINT {
        &self.in_endpoint6
    }
    #[doc = "0x1e0..0x1fc - IN Endpoint 7"]
    #[inline(always)]
    pub const fn in_endpoint7(&self) -> &IN_ENDPOINT {
        &self.in_endpoint7
    }
    #[doc = "0x200..0x21c - IN Endpoint 8"]
    #[inline(always)]
    pub const fn in_endpoint8(&self) -> &IN_ENDPOINT {
        &self.in_endpoint8
    }
    #[doc = "0x220..0x23c - IN Endpoint 9"]
    #[inline(always)]
    pub const fn in_endpoint9(&self) -> &IN_ENDPOINT {
        &self.in_endpoint9
    }
    #[doc = "0x240..0x25c - IN Endpoint 10"]
    #[inline(always)]
    pub const fn in_endpoint10(&self) -> &IN_ENDPOINT {
        &self.in_endpoint10
    }
    #[doc = "0x260..0x27c - IN Endpoint 11"]
    #[inline(always)]
    pub const fn in_endpoint11(&self) -> &IN_ENDPOINT {
        &self.in_endpoint11
    }
    #[doc = "0x300..0x318 - OUT Endpoint 0"]
    #[inline(always)]
    pub const fn out_endpoint0(&self) -> &OUT_ENDPOINT {
        &self.out_endpoint0
    }
    #[doc = "0x320..0x338 - OUT Endpoint 1"]
    #[inline(always)]
    pub const fn out_endpoint1(&self) -> &OUT_ENDPOINT {
        &self.out_endpoint1
    }
    #[doc = "0x340..0x358 - OUT Endpoint 2"]
    #[inline(always)]
    pub const fn out_endpoint2(&self) -> &OUT_ENDPOINT {
        &self.out_endpoint2
    }
    #[doc = "0x360..0x378 - OUT Endpoint 3"]
    #[inline(always)]
    pub const fn out_endpoint3(&self) -> &OUT_ENDPOINT {
        &self.out_endpoint3
    }
    #[doc = "0x380..0x398 - OUT Endpoint 4"]
    #[inline(always)]
    pub const fn out_endpoint4(&self) -> &OUT_ENDPOINT {
        &self.out_endpoint4
    }
    #[doc = "0x3a0..0x3b8 - OUT Endpoint 5"]
    #[inline(always)]
    pub const fn out_endpoint5(&self) -> &OUT_ENDPOINT {
        &self.out_endpoint5
    }
    #[doc = "0x3c0..0x3d8 - OUT Endpoint 6"]
    #[inline(always)]
    pub const fn out_endpoint6(&self) -> &OUT_ENDPOINT {
        &self.out_endpoint6
    }
    #[doc = "0x3e0..0x3f8 - OUT Endpoint 7"]
    #[inline(always)]
    pub const fn out_endpoint7(&self) -> &OUT_ENDPOINT {
        &self.out_endpoint7
    }
    #[doc = "0x400..0x418 - OUT Endpoint 8"]
    #[inline(always)]
    pub const fn out_endpoint8(&self) -> &OUT_ENDPOINT {
        &self.out_endpoint8
    }
    #[doc = "0x420..0x438 - OUT Endpoint 9"]
    #[inline(always)]
    pub const fn out_endpoint9(&self) -> &OUT_ENDPOINT {
        &self.out_endpoint9
    }
    #[doc = "0x440..0x458 - OUT Endpoint 10"]
    #[inline(always)]
    pub const fn out_endpoint10(&self) -> &OUT_ENDPOINT {
        &self.out_endpoint10
    }
    #[doc = "0x460..0x478 - OUT Endpoint 11"]
    #[inline(always)]
    pub const fn out_endpoint11(&self) -> &OUT_ENDPOINT {
        &self.out_endpoint11
    }
}
#[doc = "DCFG (rw) register accessor: OTG_HS device configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcfg`]
module"]
pub type DCFG = crate::Reg<dcfg::DCFG_SPEC>;
#[doc = "OTG_HS device configuration register"]
pub mod dcfg;
#[doc = "DCTL (rw) register accessor: OTG_HS device control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dctl`]
module"]
pub type DCTL = crate::Reg<dctl::DCTL_SPEC>;
#[doc = "OTG_HS device control register"]
pub mod dctl;
#[doc = "DSTS (r) register accessor: OTG_HS device status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsts`]
module"]
pub type DSTS = crate::Reg<dsts::DSTS_SPEC>;
#[doc = "OTG_HS device status register"]
pub mod dsts;
#[doc = "DIEPMSK (rw) register accessor: OTG_HS device IN endpoint common interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepmsk`]
module"]
pub type DIEPMSK = crate::Reg<diepmsk::DIEPMSK_SPEC>;
#[doc = "OTG_HS device IN endpoint common interrupt mask register"]
pub mod diepmsk;
#[doc = "DOEPMSK (rw) register accessor: OTG_HS device OUT endpoint common interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepmsk`]
module"]
pub type DOEPMSK = crate::Reg<doepmsk::DOEPMSK_SPEC>;
#[doc = "OTG_HS device OUT endpoint common interrupt mask register"]
pub mod doepmsk;
#[doc = "DAINT (r) register accessor: OTG_HS device all endpoints interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daint::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daint`]
module"]
pub type DAINT = crate::Reg<daint::DAINT_SPEC>;
#[doc = "OTG_HS device all endpoints interrupt register"]
pub mod daint;
#[doc = "DAINTMSK (rw) register accessor: OTG_HS all endpoints interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`daintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daintmsk`]
module"]
pub type DAINTMSK = crate::Reg<daintmsk::DAINTMSK_SPEC>;
#[doc = "OTG_HS all endpoints interrupt mask register"]
pub mod daintmsk;
#[doc = "DVBUSDIS (rw) register accessor: OTG_HS device VBUS discharge time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dvbusdis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dvbusdis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvbusdis`]
module"]
pub type DVBUSDIS = crate::Reg<dvbusdis::DVBUSDIS_SPEC>;
#[doc = "OTG_HS device VBUS discharge time register"]
pub mod dvbusdis;
#[doc = "DVBUSPULSE (rw) register accessor: OTG_HS device VBUS pulsing time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dvbuspulse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dvbuspulse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvbuspulse`]
module"]
pub type DVBUSPULSE = crate::Reg<dvbuspulse::DVBUSPULSE_SPEC>;
#[doc = "OTG_HS device VBUS pulsing time register"]
pub mod dvbuspulse;
#[doc = "DTHRCTL (rw) register accessor: OTG_HS Device threshold control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dthrctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dthrctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dthrctl`]
module"]
pub type DTHRCTL = crate::Reg<dthrctl::DTHRCTL_SPEC>;
#[doc = "OTG_HS Device threshold control register"]
pub mod dthrctl;
#[doc = "DIEPEMPMSK (rw) register accessor: OTG_HS device IN endpoint FIFO empty interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepempmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepempmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepempmsk`]
module"]
pub type DIEPEMPMSK = crate::Reg<diepempmsk::DIEPEMPMSK_SPEC>;
#[doc = "OTG_HS device IN endpoint FIFO empty interrupt mask register"]
pub mod diepempmsk;
#[doc = "DEACHINT (rw) register accessor: OTG_HS device each endpoint interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deachint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deachint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deachint`]
module"]
pub type DEACHINT = crate::Reg<deachint::DEACHINT_SPEC>;
#[doc = "OTG_HS device each endpoint interrupt register"]
pub mod deachint;
#[doc = "DEACHINTMSK (rw) register accessor: OTG_HS device each endpoint interrupt register mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deachintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deachintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deachintmsk`]
module"]
pub type DEACHINTMSK = crate::Reg<deachintmsk::DEACHINTMSK_SPEC>;
#[doc = "OTG_HS device each endpoint interrupt register mask"]
pub mod deachintmsk;
#[doc = "DIEPEACHMSK1 (rw) register accessor: OTG_HS device each in endpoint-1 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepeachmsk1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepeachmsk1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepeachmsk1`]
module"]
pub type DIEPEACHMSK1 = crate::Reg<diepeachmsk1::DIEPEACHMSK1_SPEC>;
#[doc = "OTG_HS device each in endpoint-1 interrupt register"]
pub mod diepeachmsk1;
#[doc = "DOEPEACHMSK1 (rw) register accessor: OTG_HS device each OUT endpoint-1 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepeachmsk1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepeachmsk1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepeachmsk1`]
module"]
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
