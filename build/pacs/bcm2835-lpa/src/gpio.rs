#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct RegisterBlock {
    gpfsel0: GPFSEL0,
    gpfsel1: GPFSEL1,
    gpfsel2: GPFSEL2,
    gpfsel3: GPFSEL3,
    gpfsel4: GPFSEL4,
    gpfsel5: GPFSEL5,
    _reserved6: [u8; 0x04],
    gpset0: GPSET0,
    gpset1: GPSET1,
    _reserved8: [u8; 0x04],
    gpclr0: GPCLR0,
    gpclr1: GPCLR1,
    _reserved10: [u8; 0x04],
    gplev0: GPLEV0,
    gplev1: GPLEV1,
    _reserved12: [u8; 0x04],
    gpeds0: GPEDS0,
    gpeds1: GPEDS1,
    _reserved14: [u8; 0x04],
    gpren0: GPREN0,
    gpren1: GPREN1,
    _reserved16: [u8; 0x04],
    gpfen0: GPFEN0,
    gpfen1: GPFEN1,
    _reserved18: [u8; 0x04],
    gphen0: GPHEN0,
    gphen1: GPHEN1,
    _reserved20: [u8; 0x04],
    gplen0: GPLEN0,
    gplen1: GPLEN1,
    _reserved22: [u8; 0x04],
    gparen0: GPAREN0,
    gparen1: GPAREN1,
    _reserved24: [u8; 0x04],
    gpafen0: GPAFEN0,
    gpafen1: GPAFEN1,
    _reserved26: [u8; 0x40],
    extra_mux: EXTRA_MUX,
    _reserved27: [u8; 0x10],
    gpio_pup_pdn_cntrl_reg0: GPIO_PUP_PDN_CNTRL_REG0,
    gpio_pup_pdn_cntrl_reg1: GPIO_PUP_PDN_CNTRL_REG1,
    gpio_pup_pdn_cntrl_reg2: GPIO_PUP_PDN_CNTRL_REG2,
    gpio_pup_pdn_cntrl_reg3: GPIO_PUP_PDN_CNTRL_REG3,
}
impl RegisterBlock {
    #[doc = "0x00 - GPIO Function Select 0"]
    #[inline(always)]
    pub const fn gpfsel0(&self) -> &GPFSEL0 {
        &self.gpfsel0
    }
    #[doc = "0x04 - GPIO Function Select 1"]
    #[inline(always)]
    pub const fn gpfsel1(&self) -> &GPFSEL1 {
        &self.gpfsel1
    }
    #[doc = "0x08 - GPIO Function Select 2"]
    #[inline(always)]
    pub const fn gpfsel2(&self) -> &GPFSEL2 {
        &self.gpfsel2
    }
    #[doc = "0x0c - GPIO Function Select 3"]
    #[inline(always)]
    pub const fn gpfsel3(&self) -> &GPFSEL3 {
        &self.gpfsel3
    }
    #[doc = "0x10 - GPIO Function Select 4"]
    #[inline(always)]
    pub const fn gpfsel4(&self) -> &GPFSEL4 {
        &self.gpfsel4
    }
    #[doc = "0x14 - GPIO Function Select 5"]
    #[inline(always)]
    pub const fn gpfsel5(&self) -> &GPFSEL5 {
        &self.gpfsel5
    }
    #[doc = "0x1c - GPIO Pin Output Set 0"]
    #[inline(always)]
    pub const fn gpset0(&self) -> &GPSET0 {
        &self.gpset0
    }
    #[doc = "0x20 - GPIO Pin Output Set 1"]
    #[inline(always)]
    pub const fn gpset1(&self) -> &GPSET1 {
        &self.gpset1
    }
    #[doc = "0x28 - GPIO Pin Output Clear 0"]
    #[inline(always)]
    pub const fn gpclr0(&self) -> &GPCLR0 {
        &self.gpclr0
    }
    #[doc = "0x2c - GPIO Pin Output Clear 1"]
    #[inline(always)]
    pub const fn gpclr1(&self) -> &GPCLR1 {
        &self.gpclr1
    }
    #[doc = "0x34 - GPIO Pin Level 0"]
    #[inline(always)]
    pub const fn gplev0(&self) -> &GPLEV0 {
        &self.gplev0
    }
    #[doc = "0x38 - GPIO Pin Level 1"]
    #[inline(always)]
    pub const fn gplev1(&self) -> &GPLEV1 {
        &self.gplev1
    }
    #[doc = "0x40 - GPIO Pin Event Detect Status 0"]
    #[inline(always)]
    pub const fn gpeds0(&self) -> &GPEDS0 {
        &self.gpeds0
    }
    #[doc = "0x44 - GPIO Pin Event Detect Status 1"]
    #[inline(always)]
    pub const fn gpeds1(&self) -> &GPEDS1 {
        &self.gpeds1
    }
    #[doc = "0x4c - GPIO Pin Rising Edge Detect Enable 0"]
    #[inline(always)]
    pub const fn gpren0(&self) -> &GPREN0 {
        &self.gpren0
    }
    #[doc = "0x50 - GPIO Pin Rising Edge Detect Enable 1"]
    #[inline(always)]
    pub const fn gpren1(&self) -> &GPREN1 {
        &self.gpren1
    }
    #[doc = "0x58 - GPIO Pin Falling Edge Detect Enable 0"]
    #[inline(always)]
    pub const fn gpfen0(&self) -> &GPFEN0 {
        &self.gpfen0
    }
    #[doc = "0x5c - GPIO Pin Falling Edge Detect Enable 1"]
    #[inline(always)]
    pub const fn gpfen1(&self) -> &GPFEN1 {
        &self.gpfen1
    }
    #[doc = "0x64 - GPIO Pin High Detect Enable 0"]
    #[inline(always)]
    pub const fn gphen0(&self) -> &GPHEN0 {
        &self.gphen0
    }
    #[doc = "0x68 - GPIO Pin High Detect Enable 1"]
    #[inline(always)]
    pub const fn gphen1(&self) -> &GPHEN1 {
        &self.gphen1
    }
    #[doc = "0x70 - GPIO Pin Low Detect Enable 0"]
    #[inline(always)]
    pub const fn gplen0(&self) -> &GPLEN0 {
        &self.gplen0
    }
    #[doc = "0x74 - GPIO Pin Low Detect Enable 1"]
    #[inline(always)]
    pub const fn gplen1(&self) -> &GPLEN1 {
        &self.gplen1
    }
    #[doc = "0x7c - GPIO Pin Async. Rising Edge Detect 0"]
    #[inline(always)]
    pub const fn gparen0(&self) -> &GPAREN0 {
        &self.gparen0
    }
    #[doc = "0x80 - GPIO Pin Async. Rising Edge Detect 1"]
    #[inline(always)]
    pub const fn gparen1(&self) -> &GPAREN1 {
        &self.gparen1
    }
    #[doc = "0x88 - GPIO Pin Async. Falling Edge Detect 0"]
    #[inline(always)]
    pub const fn gpafen0(&self) -> &GPAFEN0 {
        &self.gpafen0
    }
    #[doc = "0x8c - GPIO Pin Async. Falling Edge Detect 1"]
    #[inline(always)]
    pub const fn gpafen1(&self) -> &GPAFEN1 {
        &self.gpafen1
    }
    #[doc = "0xd0 - Undocumented multiplexing bits"]
    #[inline(always)]
    pub const fn extra_mux(&self) -> &EXTRA_MUX {
        &self.extra_mux
    }
    #[doc = "0xe4 - GPIO Pull-up / Pull-down Register 0"]
    #[inline(always)]
    pub const fn gpio_pup_pdn_cntrl_reg0(&self) -> &GPIO_PUP_PDN_CNTRL_REG0 {
        &self.gpio_pup_pdn_cntrl_reg0
    }
    #[doc = "0xe8 - GPIO Pull-up / Pull-down Register 1"]
    #[inline(always)]
    pub const fn gpio_pup_pdn_cntrl_reg1(&self) -> &GPIO_PUP_PDN_CNTRL_REG1 {
        &self.gpio_pup_pdn_cntrl_reg1
    }
    #[doc = "0xec - GPIO Pull-up / Pull-down Register 2"]
    #[inline(always)]
    pub const fn gpio_pup_pdn_cntrl_reg2(&self) -> &GPIO_PUP_PDN_CNTRL_REG2 {
        &self.gpio_pup_pdn_cntrl_reg2
    }
    #[doc = "0xf0 - GPIO Pull-up / Pull-down Register 3"]
    #[inline(always)]
    pub const fn gpio_pup_pdn_cntrl_reg3(&self) -> &GPIO_PUP_PDN_CNTRL_REG3 {
        &self.gpio_pup_pdn_cntrl_reg3
    }
}
#[doc = "GPFSEL0 (rw) register accessor: GPIO Function Select 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpfsel0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpfsel0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpfsel0`]
module"]
pub type GPFSEL0 = crate::Reg<gpfsel0::GPFSEL0_SPEC>;
#[doc = "GPIO Function Select 0"]
pub mod gpfsel0;
#[doc = "GPFSEL1 (rw) register accessor: GPIO Function Select 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpfsel1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpfsel1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpfsel1`]
module"]
pub type GPFSEL1 = crate::Reg<gpfsel1::GPFSEL1_SPEC>;
#[doc = "GPIO Function Select 1"]
pub mod gpfsel1;
#[doc = "GPFSEL2 (rw) register accessor: GPIO Function Select 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpfsel2::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpfsel2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpfsel2`]
module"]
pub type GPFSEL2 = crate::Reg<gpfsel2::GPFSEL2_SPEC>;
#[doc = "GPIO Function Select 2"]
pub mod gpfsel2;
#[doc = "GPFSEL3 (rw) register accessor: GPIO Function Select 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpfsel3::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpfsel3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpfsel3`]
module"]
pub type GPFSEL3 = crate::Reg<gpfsel3::GPFSEL3_SPEC>;
#[doc = "GPIO Function Select 3"]
pub mod gpfsel3;
#[doc = "GPFSEL4 (rw) register accessor: GPIO Function Select 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpfsel4::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpfsel4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpfsel4`]
module"]
pub type GPFSEL4 = crate::Reg<gpfsel4::GPFSEL4_SPEC>;
#[doc = "GPIO Function Select 4"]
pub mod gpfsel4;
#[doc = "GPFSEL5 (rw) register accessor: GPIO Function Select 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpfsel5::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpfsel5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpfsel5`]
module"]
pub type GPFSEL5 = crate::Reg<gpfsel5::GPFSEL5_SPEC>;
#[doc = "GPIO Function Select 5"]
pub mod gpfsel5;
#[doc = "GPSET0 (w) register accessor: GPIO Pin Output Set 0\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpset0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpset0`]
module"]
pub type GPSET0 = crate::Reg<gpset0::GPSET0_SPEC>;
#[doc = "GPIO Pin Output Set 0"]
pub mod gpset0;
#[doc = "GPSET1 (w) register accessor: GPIO Pin Output Set 1\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpset1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpset1`]
module"]
pub type GPSET1 = crate::Reg<gpset1::GPSET1_SPEC>;
#[doc = "GPIO Pin Output Set 1"]
pub mod gpset1;
#[doc = "GPCLR0 (w) register accessor: GPIO Pin Output Clear 0\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpclr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpclr0`]
module"]
pub type GPCLR0 = crate::Reg<gpclr0::GPCLR0_SPEC>;
#[doc = "GPIO Pin Output Clear 0"]
pub mod gpclr0;
#[doc = "GPCLR1 (w) register accessor: GPIO Pin Output Clear 1\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpclr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpclr1`]
module"]
pub type GPCLR1 = crate::Reg<gpclr1::GPCLR1_SPEC>;
#[doc = "GPIO Pin Output Clear 1"]
pub mod gpclr1;
#[doc = "GPLEV0 (r) register accessor: GPIO Pin Level 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gplev0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gplev0`]
module"]
pub type GPLEV0 = crate::Reg<gplev0::GPLEV0_SPEC>;
#[doc = "GPIO Pin Level 0"]
pub mod gplev0;
#[doc = "GPLEV1 (r) register accessor: GPIO Pin Level 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gplev1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gplev1`]
module"]
pub type GPLEV1 = crate::Reg<gplev1::GPLEV1_SPEC>;
#[doc = "GPIO Pin Level 1"]
pub mod gplev1;
#[doc = "GPEDS0 (rw) register accessor: GPIO Pin Event Detect Status 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpeds0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpeds0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpeds0`]
module"]
pub type GPEDS0 = crate::Reg<gpeds0::GPEDS0_SPEC>;
#[doc = "GPIO Pin Event Detect Status 0"]
pub mod gpeds0;
#[doc = "GPEDS1 (rw) register accessor: GPIO Pin Event Detect Status 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpeds1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpeds1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpeds1`]
module"]
pub type GPEDS1 = crate::Reg<gpeds1::GPEDS1_SPEC>;
#[doc = "GPIO Pin Event Detect Status 1"]
pub mod gpeds1;
#[doc = "GPREN0 (rw) register accessor: GPIO Pin Rising Edge Detect Enable 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpren0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpren0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpren0`]
module"]
pub type GPREN0 = crate::Reg<gpren0::GPREN0_SPEC>;
#[doc = "GPIO Pin Rising Edge Detect Enable 0"]
pub mod gpren0;
#[doc = "GPREN1 (rw) register accessor: GPIO Pin Rising Edge Detect Enable 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpren1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpren1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpren1`]
module"]
pub type GPREN1 = crate::Reg<gpren1::GPREN1_SPEC>;
#[doc = "GPIO Pin Rising Edge Detect Enable 1"]
pub mod gpren1;
#[doc = "GPFEN0 (rw) register accessor: GPIO Pin Falling Edge Detect Enable 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpfen0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpfen0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpfen0`]
module"]
pub type GPFEN0 = crate::Reg<gpfen0::GPFEN0_SPEC>;
#[doc = "GPIO Pin Falling Edge Detect Enable 0"]
pub mod gpfen0;
#[doc = "GPFEN1 (rw) register accessor: GPIO Pin Falling Edge Detect Enable 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpfen1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpfen1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpfen1`]
module"]
pub type GPFEN1 = crate::Reg<gpfen1::GPFEN1_SPEC>;
#[doc = "GPIO Pin Falling Edge Detect Enable 1"]
pub mod gpfen1;
#[doc = "GPHEN0 (rw) register accessor: GPIO Pin High Detect Enable 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gphen0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gphen0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gphen0`]
module"]
pub type GPHEN0 = crate::Reg<gphen0::GPHEN0_SPEC>;
#[doc = "GPIO Pin High Detect Enable 0"]
pub mod gphen0;
#[doc = "GPHEN1 (rw) register accessor: GPIO Pin High Detect Enable 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gphen1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gphen1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gphen1`]
module"]
pub type GPHEN1 = crate::Reg<gphen1::GPHEN1_SPEC>;
#[doc = "GPIO Pin High Detect Enable 1"]
pub mod gphen1;
#[doc = "GPLEN0 (rw) register accessor: GPIO Pin Low Detect Enable 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gplen0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gplen0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gplen0`]
module"]
pub type GPLEN0 = crate::Reg<gplen0::GPLEN0_SPEC>;
#[doc = "GPIO Pin Low Detect Enable 0"]
pub mod gplen0;
#[doc = "GPLEN1 (rw) register accessor: GPIO Pin Low Detect Enable 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gplen1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gplen1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gplen1`]
module"]
pub type GPLEN1 = crate::Reg<gplen1::GPLEN1_SPEC>;
#[doc = "GPIO Pin Low Detect Enable 1"]
pub mod gplen1;
#[doc = "GPAREN0 (rw) register accessor: GPIO Pin Async. Rising Edge Detect 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gparen0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gparen0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gparen0`]
module"]
pub type GPAREN0 = crate::Reg<gparen0::GPAREN0_SPEC>;
#[doc = "GPIO Pin Async. Rising Edge Detect 0"]
pub mod gparen0;
#[doc = "GPAREN1 (rw) register accessor: GPIO Pin Async. Rising Edge Detect 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gparen1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gparen1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gparen1`]
module"]
pub type GPAREN1 = crate::Reg<gparen1::GPAREN1_SPEC>;
#[doc = "GPIO Pin Async. Rising Edge Detect 1"]
pub mod gparen1;
#[doc = "GPAFEN0 (rw) register accessor: GPIO Pin Async. Falling Edge Detect 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpafen0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpafen0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpafen0`]
module"]
pub type GPAFEN0 = crate::Reg<gpafen0::GPAFEN0_SPEC>;
#[doc = "GPIO Pin Async. Falling Edge Detect 0"]
pub mod gpafen0;
#[doc = "GPAFEN1 (rw) register accessor: GPIO Pin Async. Falling Edge Detect 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpafen1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpafen1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpafen1`]
module"]
pub type GPAFEN1 = crate::Reg<gpafen1::GPAFEN1_SPEC>;
#[doc = "GPIO Pin Async. Falling Edge Detect 1"]
pub mod gpafen1;
#[doc = "EXTRA_MUX (rw) register accessor: Undocumented multiplexing bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extra_mux::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extra_mux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extra_mux`]
module"]
pub type EXTRA_MUX = crate::Reg<extra_mux::EXTRA_MUX_SPEC>;
#[doc = "Undocumented multiplexing bits"]
pub mod extra_mux;
#[doc = "GPIO_PUP_PDN_CNTRL_REG0 (rw) register accessor: GPIO Pull-up / Pull-down Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_pup_pdn_cntrl_reg0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_pup_pdn_cntrl_reg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_pup_pdn_cntrl_reg0`]
module"]
pub type GPIO_PUP_PDN_CNTRL_REG0 =
    crate::Reg<gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL_REG0_SPEC>;
#[doc = "GPIO Pull-up / Pull-down Register 0"]
pub mod gpio_pup_pdn_cntrl_reg0;
#[doc = "GPIO_PUP_PDN_CNTRL_REG1 (rw) register accessor: GPIO Pull-up / Pull-down Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_pup_pdn_cntrl_reg1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_pup_pdn_cntrl_reg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_pup_pdn_cntrl_reg1`]
module"]
pub type GPIO_PUP_PDN_CNTRL_REG1 =
    crate::Reg<gpio_pup_pdn_cntrl_reg1::GPIO_PUP_PDN_CNTRL_REG1_SPEC>;
#[doc = "GPIO Pull-up / Pull-down Register 1"]
pub mod gpio_pup_pdn_cntrl_reg1;
#[doc = "GPIO_PUP_PDN_CNTRL_REG2 (rw) register accessor: GPIO Pull-up / Pull-down Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_pup_pdn_cntrl_reg2::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_pup_pdn_cntrl_reg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_pup_pdn_cntrl_reg2`]
module"]
pub type GPIO_PUP_PDN_CNTRL_REG2 =
    crate::Reg<gpio_pup_pdn_cntrl_reg2::GPIO_PUP_PDN_CNTRL_REG2_SPEC>;
#[doc = "GPIO Pull-up / Pull-down Register 2"]
pub mod gpio_pup_pdn_cntrl_reg2;
#[doc = "GPIO_PUP_PDN_CNTRL_REG3 (rw) register accessor: GPIO Pull-up / Pull-down Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_pup_pdn_cntrl_reg3::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_pup_pdn_cntrl_reg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_pup_pdn_cntrl_reg3`]
module"]
pub type GPIO_PUP_PDN_CNTRL_REG3 =
    crate::Reg<gpio_pup_pdn_cntrl_reg3::GPIO_PUP_PDN_CNTRL_REG3_SPEC>;
#[doc = "GPIO Pull-up / Pull-down Register 3"]
pub mod gpio_pup_pdn_cntrl_reg3;
