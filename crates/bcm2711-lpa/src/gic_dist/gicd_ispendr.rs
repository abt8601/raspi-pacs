#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct GICD_ISPENDR {
    gicd_ispendr0: GICD_ISPENDR0,
    gicd_ispendr1: GICD_ISPENDR1,
    gicd_ispendr2: GICD_ISPENDR2,
    gicd_ispendr3: GICD_ISPENDR3,
    gicd_ispendr4: GICD_ISPENDR4,
    gicd_ispendr5: GICD_ISPENDR5,
    gicd_ispendr6: GICD_ISPENDR6,
}
impl GICD_ISPENDR {
    #[doc = "0x00 - Interrupt Set-Pending"]
    #[inline(always)]
    pub const fn gicd_ispendr0(&self) -> &GICD_ISPENDR0 {
        &self.gicd_ispendr0
    }
    #[doc = "0x04 - Interrupt Set-Pending"]
    #[inline(always)]
    pub const fn gicd_ispendr1(&self) -> &GICD_ISPENDR1 {
        &self.gicd_ispendr1
    }
    #[doc = "0x08 - Interrupt Set-Pending"]
    #[inline(always)]
    pub const fn gicd_ispendr2(&self) -> &GICD_ISPENDR2 {
        &self.gicd_ispendr2
    }
    #[doc = "0x0c - Interrupt Set-Pending"]
    #[inline(always)]
    pub const fn gicd_ispendr3(&self) -> &GICD_ISPENDR3 {
        &self.gicd_ispendr3
    }
    #[doc = "0x10 - Interrupt Set-Pending"]
    #[inline(always)]
    pub const fn gicd_ispendr4(&self) -> &GICD_ISPENDR4 {
        &self.gicd_ispendr4
    }
    #[doc = "0x14 - Interrupt Set-Pending"]
    #[inline(always)]
    pub const fn gicd_ispendr5(&self) -> &GICD_ISPENDR5 {
        &self.gicd_ispendr5
    }
    #[doc = "0x18 - Interrupt Set-Pending"]
    #[inline(always)]
    pub const fn gicd_ispendr6(&self) -> &GICD_ISPENDR6 {
        &self.gicd_ispendr6
    }
}
#[doc = "GICD_ISPENDR0 (rw) register accessor: Interrupt Set-Pending\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ispendr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ispendr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ispendr0`]
module"]
pub type GICD_ISPENDR0 = crate::Reg<gicd_ispendr0::GICD_ISPENDR0_SPEC>;
#[doc = "Interrupt Set-Pending"]
pub mod gicd_ispendr0;
#[doc = "GICD_ISPENDR1 (rw) register accessor: Interrupt Set-Pending\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ispendr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ispendr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ispendr1`]
module"]
pub type GICD_ISPENDR1 = crate::Reg<gicd_ispendr1::GICD_ISPENDR1_SPEC>;
#[doc = "Interrupt Set-Pending"]
pub mod gicd_ispendr1;
#[doc = "GICD_ISPENDR2 (rw) register accessor: Interrupt Set-Pending\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ispendr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ispendr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ispendr2`]
module"]
pub type GICD_ISPENDR2 = crate::Reg<gicd_ispendr2::GICD_ISPENDR2_SPEC>;
#[doc = "Interrupt Set-Pending"]
pub mod gicd_ispendr2;
#[doc = "GICD_ISPENDR3 (rw) register accessor: Interrupt Set-Pending\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ispendr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ispendr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ispendr3`]
module"]
pub type GICD_ISPENDR3 = crate::Reg<gicd_ispendr3::GICD_ISPENDR3_SPEC>;
#[doc = "Interrupt Set-Pending"]
pub mod gicd_ispendr3;
#[doc = "GICD_ISPENDR4 (rw) register accessor: Interrupt Set-Pending\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ispendr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ispendr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ispendr4`]
module"]
pub type GICD_ISPENDR4 = crate::Reg<gicd_ispendr4::GICD_ISPENDR4_SPEC>;
#[doc = "Interrupt Set-Pending"]
pub mod gicd_ispendr4;
#[doc = "GICD_ISPENDR5 (rw) register accessor: Interrupt Set-Pending\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ispendr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ispendr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ispendr5`]
module"]
pub type GICD_ISPENDR5 = crate::Reg<gicd_ispendr5::GICD_ISPENDR5_SPEC>;
#[doc = "Interrupt Set-Pending"]
pub mod gicd_ispendr5;
#[doc = "GICD_ISPENDR6 (rw) register accessor: Interrupt Set-Pending\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ispendr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ispendr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ispendr6`]
module"]
pub type GICD_ISPENDR6 = crate::Reg<gicd_ispendr6::GICD_ISPENDR6_SPEC>;
#[doc = "Interrupt Set-Pending"]
pub mod gicd_ispendr6;
