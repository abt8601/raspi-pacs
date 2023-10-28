#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct GICD_ICENABLER {
    #[doc = "0x00 - Interrupt Clear-Enable"]
    pub gicd_icenabler0: GICD_ICENABLER0,
    #[doc = "0x04 - Interrupt Clear-Enable"]
    pub gicd_icenabler1: GICD_ICENABLER1,
    #[doc = "0x08 - Interrupt Clear-Enable"]
    pub gicd_icenabler2: GICD_ICENABLER2,
    #[doc = "0x0c - Interrupt Clear-Enable"]
    pub gicd_icenabler3: GICD_ICENABLER3,
    #[doc = "0x10 - Interrupt Clear-Enable"]
    pub gicd_icenabler4: GICD_ICENABLER4,
    #[doc = "0x14 - Interrupt Clear-Enable"]
    pub gicd_icenabler5: GICD_ICENABLER5,
    #[doc = "0x18 - Interrupt Clear-Enable"]
    pub gicd_icenabler6: GICD_ICENABLER6,
}
#[doc = "GICD_ICENABLER0 (rw) register accessor: Interrupt Clear-Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icenabler0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icenabler0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icenabler0`]
module"]
pub type GICD_ICENABLER0 = crate::Reg<gicd_icenabler0::GICD_ICENABLER0_SPEC>;
#[doc = "Interrupt Clear-Enable"]
pub mod gicd_icenabler0;
#[doc = "GICD_ICENABLER1 (rw) register accessor: Interrupt Clear-Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icenabler1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icenabler1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icenabler1`]
module"]
pub type GICD_ICENABLER1 = crate::Reg<gicd_icenabler1::GICD_ICENABLER1_SPEC>;
#[doc = "Interrupt Clear-Enable"]
pub mod gicd_icenabler1;
#[doc = "GICD_ICENABLER2 (rw) register accessor: Interrupt Clear-Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icenabler2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icenabler2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icenabler2`]
module"]
pub type GICD_ICENABLER2 = crate::Reg<gicd_icenabler2::GICD_ICENABLER2_SPEC>;
#[doc = "Interrupt Clear-Enable"]
pub mod gicd_icenabler2;
#[doc = "GICD_ICENABLER3 (rw) register accessor: Interrupt Clear-Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icenabler3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icenabler3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icenabler3`]
module"]
pub type GICD_ICENABLER3 = crate::Reg<gicd_icenabler3::GICD_ICENABLER3_SPEC>;
#[doc = "Interrupt Clear-Enable"]
pub mod gicd_icenabler3;
#[doc = "GICD_ICENABLER4 (rw) register accessor: Interrupt Clear-Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icenabler4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icenabler4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icenabler4`]
module"]
pub type GICD_ICENABLER4 = crate::Reg<gicd_icenabler4::GICD_ICENABLER4_SPEC>;
#[doc = "Interrupt Clear-Enable"]
pub mod gicd_icenabler4;
#[doc = "GICD_ICENABLER5 (rw) register accessor: Interrupt Clear-Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icenabler5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icenabler5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icenabler5`]
module"]
pub type GICD_ICENABLER5 = crate::Reg<gicd_icenabler5::GICD_ICENABLER5_SPEC>;
#[doc = "Interrupt Clear-Enable"]
pub mod gicd_icenabler5;
#[doc = "GICD_ICENABLER6 (rw) register accessor: Interrupt Clear-Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icenabler6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icenabler6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icenabler6`]
module"]
pub type GICD_ICENABLER6 = crate::Reg<gicd_icenabler6::GICD_ICENABLER6_SPEC>;
#[doc = "Interrupt Clear-Enable"]
pub mod gicd_icenabler6;
