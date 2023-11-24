#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct GICD_ISACTIVER {
    gicd_isactiver0: GICD_ISACTIVER0,
    gicd_isactiver1: GICD_ISACTIVER1,
    gicd_isactiver2: GICD_ISACTIVER2,
    gicd_isactiver3: GICD_ISACTIVER3,
    gicd_isactiver4: GICD_ISACTIVER4,
    gicd_isactiver5: GICD_ISACTIVER5,
    gicd_isactiver6: GICD_ISACTIVER6,
}
impl GICD_ISACTIVER {
    #[doc = "0x00 - Interrupt Set-Active"]
    #[inline(always)]
    pub const fn gicd_isactiver0(&self) -> &GICD_ISACTIVER0 {
        &self.gicd_isactiver0
    }
    #[doc = "0x04 - Interrupt Set-Active"]
    #[inline(always)]
    pub const fn gicd_isactiver1(&self) -> &GICD_ISACTIVER1 {
        &self.gicd_isactiver1
    }
    #[doc = "0x08 - Interrupt Set-Active"]
    #[inline(always)]
    pub const fn gicd_isactiver2(&self) -> &GICD_ISACTIVER2 {
        &self.gicd_isactiver2
    }
    #[doc = "0x0c - Interrupt Set-Active"]
    #[inline(always)]
    pub const fn gicd_isactiver3(&self) -> &GICD_ISACTIVER3 {
        &self.gicd_isactiver3
    }
    #[doc = "0x10 - Interrupt Set-Active"]
    #[inline(always)]
    pub const fn gicd_isactiver4(&self) -> &GICD_ISACTIVER4 {
        &self.gicd_isactiver4
    }
    #[doc = "0x14 - Interrupt Set-Active"]
    #[inline(always)]
    pub const fn gicd_isactiver5(&self) -> &GICD_ISACTIVER5 {
        &self.gicd_isactiver5
    }
    #[doc = "0x18 - Interrupt Set-Active"]
    #[inline(always)]
    pub const fn gicd_isactiver6(&self) -> &GICD_ISACTIVER6 {
        &self.gicd_isactiver6
    }
}
#[doc = "GICD_ISACTIVER0 (rw) register accessor: Interrupt Set-Active\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_isactiver0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_isactiver0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_isactiver0`]
module"]
pub type GICD_ISACTIVER0 = crate::Reg<gicd_isactiver0::GICD_ISACTIVER0_SPEC>;
#[doc = "Interrupt Set-Active"]
pub mod gicd_isactiver0;
#[doc = "GICD_ISACTIVER1 (rw) register accessor: Interrupt Set-Active\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_isactiver1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_isactiver1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_isactiver1`]
module"]
pub type GICD_ISACTIVER1 = crate::Reg<gicd_isactiver1::GICD_ISACTIVER1_SPEC>;
#[doc = "Interrupt Set-Active"]
pub mod gicd_isactiver1;
#[doc = "GICD_ISACTIVER2 (rw) register accessor: Interrupt Set-Active\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_isactiver2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_isactiver2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_isactiver2`]
module"]
pub type GICD_ISACTIVER2 = crate::Reg<gicd_isactiver2::GICD_ISACTIVER2_SPEC>;
#[doc = "Interrupt Set-Active"]
pub mod gicd_isactiver2;
#[doc = "GICD_ISACTIVER3 (rw) register accessor: Interrupt Set-Active\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_isactiver3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_isactiver3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_isactiver3`]
module"]
pub type GICD_ISACTIVER3 = crate::Reg<gicd_isactiver3::GICD_ISACTIVER3_SPEC>;
#[doc = "Interrupt Set-Active"]
pub mod gicd_isactiver3;
#[doc = "GICD_ISACTIVER4 (rw) register accessor: Interrupt Set-Active\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_isactiver4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_isactiver4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_isactiver4`]
module"]
pub type GICD_ISACTIVER4 = crate::Reg<gicd_isactiver4::GICD_ISACTIVER4_SPEC>;
#[doc = "Interrupt Set-Active"]
pub mod gicd_isactiver4;
#[doc = "GICD_ISACTIVER5 (rw) register accessor: Interrupt Set-Active\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_isactiver5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_isactiver5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_isactiver5`]
module"]
pub type GICD_ISACTIVER5 = crate::Reg<gicd_isactiver5::GICD_ISACTIVER5_SPEC>;
#[doc = "Interrupt Set-Active"]
pub mod gicd_isactiver5;
#[doc = "GICD_ISACTIVER6 (rw) register accessor: Interrupt Set-Active\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_isactiver6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_isactiver6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_isactiver6`]
module"]
pub type GICD_ISACTIVER6 = crate::Reg<gicd_isactiver6::GICD_ISACTIVER6_SPEC>;
#[doc = "Interrupt Set-Active"]
pub mod gicd_isactiver6;
