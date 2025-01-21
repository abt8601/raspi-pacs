#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct RegisterBlock {
    irq: IRQ,
    enables: ENABLES,
}
impl RegisterBlock {
    #[doc = "0x00 - Interrupt status"]
    #[inline(always)]
    pub const fn irq(&self) -> &IRQ {
        &self.irq
    }
    #[doc = "0x04 - Enable sub-peripherals"]
    #[inline(always)]
    pub const fn enables(&self) -> &ENABLES {
        &self.enables
    }
}
#[doc = "IRQ (rw) register accessor: Interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq`]
module"]
pub type IRQ = crate::Reg<irq::IRQ_SPEC>;
#[doc = "Interrupt status"]
#[path = "aux_/irq.rs"]
pub mod irq;
#[doc = "ENABLES (rw) register accessor: Enable sub-peripherals\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enables::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enables::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enables`]
module"]
pub type ENABLES = crate::Reg<enables::ENABLES_SPEC>;
#[doc = "Enable sub-peripherals"]
#[path = "aux_/enables.rs"]
pub mod enables;
