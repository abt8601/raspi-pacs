#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control / Status"]
    pub cs: CS,
    #[doc = "0x04 - Lower 32 bits for the free running counter"]
    pub clo: CLO,
    #[doc = "0x08 - Higher 32 bits for the free running counter"]
    pub chi: CHI,
    #[doc = "0x0c - Compare channel 0"]
    pub c0: C0,
    #[doc = "0x10 - Compare channel 1"]
    pub c1: C1,
    #[doc = "0x14 - Compare channel 2"]
    pub c2: C2,
    #[doc = "0x18 - Compare channel 3"]
    pub c3: C3,
}
#[doc = "CS (rw) register accessor: Control / Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs`]
module"]
pub type CS = crate::Reg<cs::CS_SPEC>;
#[doc = "Control / Status"]
pub mod cs;
#[doc = "CLO (r) register accessor: Lower 32 bits for the free running counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clo`]
module"]
pub type CLO = crate::Reg<clo::CLO_SPEC>;
#[doc = "Lower 32 bits for the free running counter"]
pub mod clo;
#[doc = "CHI (r) register accessor: Higher 32 bits for the free running counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chi`]
module"]
pub type CHI = crate::Reg<chi::CHI_SPEC>;
#[doc = "Higher 32 bits for the free running counter"]
pub mod chi;
#[doc = "C0 (rw) register accessor: Compare channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c0`]
module"]
pub type C0 = crate::Reg<c0::C0_SPEC>;
#[doc = "Compare channel 0"]
pub mod c0;
#[doc = "C1 (rw) register accessor: Compare channel 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1`]
module"]
pub type C1 = crate::Reg<c1::C1_SPEC>;
#[doc = "Compare channel 1"]
pub mod c1;
#[doc = "C2 (rw) register accessor: Compare channel 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2`]
module"]
pub type C2 = crate::Reg<c2::C2_SPEC>;
#[doc = "Compare channel 2"]
pub mod c2;
#[doc = "C3 (rw) register accessor: Compare channel 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c3`]
module"]
pub type C3 = crate::Reg<c3::C3_SPEC>;
#[doc = "Compare channel 3"]
pub mod c3;
