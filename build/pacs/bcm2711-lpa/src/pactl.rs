#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct RegisterBlock {
    cs: CS,
}
impl RegisterBlock {
    #[doc = "0x00 - Interrupt status"]
    #[inline(always)]
    pub const fn cs(&self) -> &CS {
        &self.cs
    }
}
#[doc = "CS (rw) register accessor: Interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs`]
module"]
pub type CS = crate::Reg<cs::CS_SPEC>;
#[doc = "Interrupt status"]
pub mod cs;
