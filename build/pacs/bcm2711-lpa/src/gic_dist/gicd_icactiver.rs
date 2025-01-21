#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct GICD_ICACTIVER {
    gicd_icactiver0: GICD_ICACTIVER0,
    gicd_icactiver1: GICD_ICACTIVER1,
    gicd_icactiver2: GICD_ICACTIVER2,
    gicd_icactiver3: GICD_ICACTIVER3,
    gicd_icactiver4: GICD_ICACTIVER4,
    gicd_icactiver5: GICD_ICACTIVER5,
    gicd_icactiver6: GICD_ICACTIVER6,
}
impl GICD_ICACTIVER {
    #[doc = "0x00 - Interrupt Clear-Active"]
    #[inline(always)]
    pub const fn gicd_icactiver0(&self) -> &GICD_ICACTIVER0 {
        &self.gicd_icactiver0
    }
    #[doc = "0x04 - Interrupt Clear-Active"]
    #[inline(always)]
    pub const fn gicd_icactiver1(&self) -> &GICD_ICACTIVER1 {
        &self.gicd_icactiver1
    }
    #[doc = "0x08 - Interrupt Clear-Active"]
    #[inline(always)]
    pub const fn gicd_icactiver2(&self) -> &GICD_ICACTIVER2 {
        &self.gicd_icactiver2
    }
    #[doc = "0x0c - Interrupt Clear-Active"]
    #[inline(always)]
    pub const fn gicd_icactiver3(&self) -> &GICD_ICACTIVER3 {
        &self.gicd_icactiver3
    }
    #[doc = "0x10 - Interrupt Clear-Active"]
    #[inline(always)]
    pub const fn gicd_icactiver4(&self) -> &GICD_ICACTIVER4 {
        &self.gicd_icactiver4
    }
    #[doc = "0x14 - Interrupt Clear-Active"]
    #[inline(always)]
    pub const fn gicd_icactiver5(&self) -> &GICD_ICACTIVER5 {
        &self.gicd_icactiver5
    }
    #[doc = "0x18 - Interrupt Clear-Active"]
    #[inline(always)]
    pub const fn gicd_icactiver6(&self) -> &GICD_ICACTIVER6 {
        &self.gicd_icactiver6
    }
}
#[doc = "GICD_ICACTIVER0 (rw) register accessor: Interrupt Clear-Active\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icactiver0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icactiver0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icactiver0`]
module"]
pub type GICD_ICACTIVER0 = crate::Reg<gicd_icactiver0::GICD_ICACTIVER0_SPEC>;
#[doc = "Interrupt Clear-Active"]
pub mod gicd_icactiver0;
#[doc = "GICD_ICACTIVER1 (rw) register accessor: Interrupt Clear-Active\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icactiver1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icactiver1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icactiver1`]
module"]
pub type GICD_ICACTIVER1 = crate::Reg<gicd_icactiver1::GICD_ICACTIVER1_SPEC>;
#[doc = "Interrupt Clear-Active"]
pub mod gicd_icactiver1;
#[doc = "GICD_ICACTIVER2 (rw) register accessor: Interrupt Clear-Active\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icactiver2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icactiver2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icactiver2`]
module"]
pub type GICD_ICACTIVER2 = crate::Reg<gicd_icactiver2::GICD_ICACTIVER2_SPEC>;
#[doc = "Interrupt Clear-Active"]
pub mod gicd_icactiver2;
#[doc = "GICD_ICACTIVER3 (rw) register accessor: Interrupt Clear-Active\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icactiver3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icactiver3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icactiver3`]
module"]
pub type GICD_ICACTIVER3 = crate::Reg<gicd_icactiver3::GICD_ICACTIVER3_SPEC>;
#[doc = "Interrupt Clear-Active"]
pub mod gicd_icactiver3;
#[doc = "GICD_ICACTIVER4 (rw) register accessor: Interrupt Clear-Active\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icactiver4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icactiver4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icactiver4`]
module"]
pub type GICD_ICACTIVER4 = crate::Reg<gicd_icactiver4::GICD_ICACTIVER4_SPEC>;
#[doc = "Interrupt Clear-Active"]
pub mod gicd_icactiver4;
#[doc = "GICD_ICACTIVER5 (rw) register accessor: Interrupt Clear-Active\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icactiver5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icactiver5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icactiver5`]
module"]
pub type GICD_ICACTIVER5 = crate::Reg<gicd_icactiver5::GICD_ICACTIVER5_SPEC>;
#[doc = "Interrupt Clear-Active"]
pub mod gicd_icactiver5;
#[doc = "GICD_ICACTIVER6 (rw) register accessor: Interrupt Clear-Active\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icactiver6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icactiver6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icactiver6`]
module"]
pub type GICD_ICACTIVER6 = crate::Reg<gicd_icactiver6::GICD_ICACTIVER6_SPEC>;
#[doc = "Interrupt Clear-Active"]
pub mod gicd_icactiver6;
