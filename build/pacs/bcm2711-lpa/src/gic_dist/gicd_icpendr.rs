#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct GICD_ICPENDR {
    gicd_icpendr0: GICD_ICPENDR0,
    gicd_icpendr1: GICD_ICPENDR1,
    gicd_icpendr2: GICD_ICPENDR2,
    gicd_icpendr3: GICD_ICPENDR3,
    gicd_icpendr4: GICD_ICPENDR4,
    gicd_icpendr5: GICD_ICPENDR5,
    gicd_icpendr6: GICD_ICPENDR6,
}
impl GICD_ICPENDR {
    #[doc = "0x00 - Interrupt Clear-Pending"]
    #[inline(always)]
    pub const fn gicd_icpendr0(&self) -> &GICD_ICPENDR0 {
        &self.gicd_icpendr0
    }
    #[doc = "0x04 - Interrupt Clear-Pending"]
    #[inline(always)]
    pub const fn gicd_icpendr1(&self) -> &GICD_ICPENDR1 {
        &self.gicd_icpendr1
    }
    #[doc = "0x08 - Interrupt Clear-Pending"]
    #[inline(always)]
    pub const fn gicd_icpendr2(&self) -> &GICD_ICPENDR2 {
        &self.gicd_icpendr2
    }
    #[doc = "0x0c - Interrupt Clear-Pending"]
    #[inline(always)]
    pub const fn gicd_icpendr3(&self) -> &GICD_ICPENDR3 {
        &self.gicd_icpendr3
    }
    #[doc = "0x10 - Interrupt Clear-Pending"]
    #[inline(always)]
    pub const fn gicd_icpendr4(&self) -> &GICD_ICPENDR4 {
        &self.gicd_icpendr4
    }
    #[doc = "0x14 - Interrupt Clear-Pending"]
    #[inline(always)]
    pub const fn gicd_icpendr5(&self) -> &GICD_ICPENDR5 {
        &self.gicd_icpendr5
    }
    #[doc = "0x18 - Interrupt Clear-Pending"]
    #[inline(always)]
    pub const fn gicd_icpendr6(&self) -> &GICD_ICPENDR6 {
        &self.gicd_icpendr6
    }
}
#[doc = "GICD_ICPENDR0 (rw) register accessor: Interrupt Clear-Pending\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icpendr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icpendr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icpendr0`]
module"]
pub type GICD_ICPENDR0 = crate::Reg<gicd_icpendr0::GICD_ICPENDR0_SPEC>;
#[doc = "Interrupt Clear-Pending"]
pub mod gicd_icpendr0;
#[doc = "GICD_ICPENDR1 (rw) register accessor: Interrupt Clear-Pending\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icpendr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icpendr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icpendr1`]
module"]
pub type GICD_ICPENDR1 = crate::Reg<gicd_icpendr1::GICD_ICPENDR1_SPEC>;
#[doc = "Interrupt Clear-Pending"]
pub mod gicd_icpendr1;
#[doc = "GICD_ICPENDR2 (rw) register accessor: Interrupt Clear-Pending\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icpendr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icpendr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icpendr2`]
module"]
pub type GICD_ICPENDR2 = crate::Reg<gicd_icpendr2::GICD_ICPENDR2_SPEC>;
#[doc = "Interrupt Clear-Pending"]
pub mod gicd_icpendr2;
#[doc = "GICD_ICPENDR3 (rw) register accessor: Interrupt Clear-Pending\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icpendr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icpendr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icpendr3`]
module"]
pub type GICD_ICPENDR3 = crate::Reg<gicd_icpendr3::GICD_ICPENDR3_SPEC>;
#[doc = "Interrupt Clear-Pending"]
pub mod gicd_icpendr3;
#[doc = "GICD_ICPENDR4 (rw) register accessor: Interrupt Clear-Pending\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icpendr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icpendr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icpendr4`]
module"]
pub type GICD_ICPENDR4 = crate::Reg<gicd_icpendr4::GICD_ICPENDR4_SPEC>;
#[doc = "Interrupt Clear-Pending"]
pub mod gicd_icpendr4;
#[doc = "GICD_ICPENDR5 (rw) register accessor: Interrupt Clear-Pending\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icpendr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icpendr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icpendr5`]
module"]
pub type GICD_ICPENDR5 = crate::Reg<gicd_icpendr5::GICD_ICPENDR5_SPEC>;
#[doc = "Interrupt Clear-Pending"]
pub mod gicd_icpendr5;
#[doc = "GICD_ICPENDR6 (rw) register accessor: Interrupt Clear-Pending\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_icpendr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_icpendr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_icpendr6`]
module"]
pub type GICD_ICPENDR6 = crate::Reg<gicd_icpendr6::GICD_ICPENDR6_SPEC>;
#[doc = "Interrupt Clear-Pending"]
pub mod gicd_icpendr6;
