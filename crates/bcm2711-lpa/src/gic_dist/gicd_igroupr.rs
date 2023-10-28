#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct GICD_IGROUPR {
    #[doc = "0x00 - Interrupt Group"]
    pub gicd_igroupr0: GICD_IGROUPR0,
    #[doc = "0x04 - Interrupt Group"]
    pub gicd_igroupr1: GICD_IGROUPR1,
    #[doc = "0x08 - Interrupt Group"]
    pub gicd_igroupr2: GICD_IGROUPR2,
    #[doc = "0x0c - Interrupt Group"]
    pub gicd_igroupr3: GICD_IGROUPR3,
    #[doc = "0x10 - Interrupt Group"]
    pub gicd_igroupr4: GICD_IGROUPR4,
    #[doc = "0x14 - Interrupt Group"]
    pub gicd_igroupr5: GICD_IGROUPR5,
    #[doc = "0x18 - Interrupt Group"]
    pub gicd_igroupr6: GICD_IGROUPR6,
}
#[doc = "GICD_IGROUPR0 (rw) register accessor: Interrupt Group\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_igroupr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_igroupr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_igroupr0`]
module"]
pub type GICD_IGROUPR0 = crate::Reg<gicd_igroupr0::GICD_IGROUPR0_SPEC>;
#[doc = "Interrupt Group"]
pub mod gicd_igroupr0;
#[doc = "GICD_IGROUPR1 (rw) register accessor: Interrupt Group\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_igroupr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_igroupr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_igroupr1`]
module"]
pub type GICD_IGROUPR1 = crate::Reg<gicd_igroupr1::GICD_IGROUPR1_SPEC>;
#[doc = "Interrupt Group"]
pub mod gicd_igroupr1;
#[doc = "GICD_IGROUPR2 (rw) register accessor: Interrupt Group\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_igroupr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_igroupr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_igroupr2`]
module"]
pub type GICD_IGROUPR2 = crate::Reg<gicd_igroupr2::GICD_IGROUPR2_SPEC>;
#[doc = "Interrupt Group"]
pub mod gicd_igroupr2;
#[doc = "GICD_IGROUPR3 (rw) register accessor: Interrupt Group\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_igroupr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_igroupr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_igroupr3`]
module"]
pub type GICD_IGROUPR3 = crate::Reg<gicd_igroupr3::GICD_IGROUPR3_SPEC>;
#[doc = "Interrupt Group"]
pub mod gicd_igroupr3;
#[doc = "GICD_IGROUPR4 (rw) register accessor: Interrupt Group\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_igroupr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_igroupr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_igroupr4`]
module"]
pub type GICD_IGROUPR4 = crate::Reg<gicd_igroupr4::GICD_IGROUPR4_SPEC>;
#[doc = "Interrupt Group"]
pub mod gicd_igroupr4;
#[doc = "GICD_IGROUPR5 (rw) register accessor: Interrupt Group\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_igroupr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_igroupr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_igroupr5`]
module"]
pub type GICD_IGROUPR5 = crate::Reg<gicd_igroupr5::GICD_IGROUPR5_SPEC>;
#[doc = "Interrupt Group"]
pub mod gicd_igroupr5;
#[doc = "GICD_IGROUPR6 (rw) register accessor: Interrupt Group\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_igroupr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_igroupr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_igroupr6`]
module"]
pub type GICD_IGROUPR6 = crate::Reg<gicd_igroupr6::GICD_IGROUPR6_SPEC>;
#[doc = "Interrupt Group"]
pub mod gicd_igroupr6;
