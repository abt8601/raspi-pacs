#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1c],
    rstc: RSTC,
    _reserved1: [u8; 0x04],
    wdog: WDOG,
}
impl RegisterBlock {
    #[doc = "0x1c - Reset Control"]
    #[inline(always)]
    pub const fn rstc(&self) -> &RSTC {
        &self.rstc
    }
    #[doc = "0x24 - Watchdog control"]
    #[inline(always)]
    pub const fn wdog(&self) -> &WDOG {
        &self.wdog
    }
}
#[doc = "RSTC (rw) register accessor: Reset Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstc`]
module"]
pub type RSTC = crate::Reg<rstc::RSTC_SPEC>;
#[doc = "Reset Control"]
pub mod rstc;
#[doc = "WDOG (rw) register accessor: Watchdog control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdog::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdog::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdog`]
module"]
pub type WDOG = crate::Reg<wdog::WDOG_SPEC>;
#[doc = "Watchdog control"]
pub mod wdog;
