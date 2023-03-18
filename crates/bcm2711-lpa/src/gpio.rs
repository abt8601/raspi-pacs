#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO Function Select 0"]
    pub gpfsel0: GPFSEL0,
    #[doc = "0x04 - GPIO Function Select 1"]
    pub gpfsel1: GPFSEL1,
    #[doc = "0x08 - GPIO Function Select 2"]
    pub gpfsel2: GPFSEL2,
    #[doc = "0x0c - GPIO Function Select 3"]
    pub gpfsel3: GPFSEL3,
    #[doc = "0x10 - GPIO Function Select 4"]
    pub gpfsel4: GPFSEL4,
    #[doc = "0x14 - GPIO Function Select 5"]
    pub gpfsel5: GPFSEL5,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - GPIO Pin Output Set 0"]
    pub gpset0: GPSET0,
    #[doc = "0x20 - GPIO Pin Output Set 1"]
    pub gpset1: GPSET1,
    _reserved8: [u8; 0x04],
    #[doc = "0x28 - GPIO Pin Output Clear 0"]
    pub gpclr0: GPCLR0,
    #[doc = "0x2c - GPIO Pin Output Clear 1"]
    pub gpclr1: GPCLR1,
    _reserved10: [u8; 0x04],
    #[doc = "0x34 - GPIO Pin Level 0"]
    pub gplev0: GPLEV0,
    #[doc = "0x38 - GPIO Pin Level 1"]
    pub gplev1: GPLEV1,
    _reserved12: [u8; 0x04],
    #[doc = "0x40 - GPIO Pin Event Detect Status 0"]
    pub gpeds0: GPEDS0,
    #[doc = "0x44 - GPIO Pin Event Detect Status 1"]
    pub gpeds1: GPEDS1,
    _reserved14: [u8; 0x04],
    #[doc = "0x4c - GPIO Pin Rising Edge Detect Enable 0"]
    pub gpren0: GPREN0,
    #[doc = "0x50 - GPIO Pin Rising Edge Detect Enable 1"]
    pub gpren1: GPREN1,
    _reserved16: [u8; 0x04],
    #[doc = "0x58 - GPIO Pin Falling Edge Detect Enable 0"]
    pub gpfen0: GPFEN0,
    #[doc = "0x5c - GPIO Pin Falling Edge Detect Enable 1"]
    pub gpfen1: GPFEN1,
    _reserved18: [u8; 0x04],
    #[doc = "0x64 - GPIO Pin High Detect Enable 0"]
    pub gphen0: GPHEN0,
    #[doc = "0x68 - GPIO Pin High Detect Enable 1"]
    pub gphen1: GPHEN1,
    _reserved20: [u8; 0x04],
    #[doc = "0x70 - GPIO Pin Low Detect Enable 0"]
    pub gplen0: GPLEN0,
    #[doc = "0x74 - GPIO Pin Low Detect Enable 1"]
    pub gplen1: GPLEN1,
    _reserved22: [u8; 0x04],
    #[doc = "0x7c - GPIO Pin Async. Rising Edge Detect 0"]
    pub gparen0: GPAREN0,
    #[doc = "0x80 - GPIO Pin Async. Rising Edge Detect 1"]
    pub gparen1: GPAREN1,
    _reserved24: [u8; 0x04],
    #[doc = "0x88 - GPIO Pin Async. Falling Edge Detect 0"]
    pub gpafen0: GPAFEN0,
    #[doc = "0x8c - GPIO Pin Async. Falling Edge Detect 1"]
    pub gpafen1: GPAFEN1,
    _reserved26: [u8; 0x40],
    #[doc = "0xd0 - Undocumented multiplexing bits"]
    pub extra_mux: EXTRA_MUX,
    _reserved27: [u8; 0x10],
    #[doc = "0xe4 - GPIO Pull-up / Pull-down Register 0"]
    pub gpio_pup_pdn_cntrl_reg0: GPIO_PUP_PDN_CNTRL_REG0,
    #[doc = "0xe8 - GPIO Pull-up / Pull-down Register 1"]
    pub gpio_pup_pdn_cntrl_reg1: GPIO_PUP_PDN_CNTRL_REG1,
    #[doc = "0xec - GPIO Pull-up / Pull-down Register 2"]
    pub gpio_pup_pdn_cntrl_reg2: GPIO_PUP_PDN_CNTRL_REG2,
    #[doc = "0xf0 - GPIO Pull-up / Pull-down Register 3"]
    pub gpio_pup_pdn_cntrl_reg3: GPIO_PUP_PDN_CNTRL_REG3,
}
#[doc = "GPFSEL0 (rw) register accessor: an alias for `Reg<GPFSEL0_SPEC>`"]
pub type GPFSEL0 = crate::Reg<gpfsel0::GPFSEL0_SPEC>;
#[doc = "GPIO Function Select 0"]
pub mod gpfsel0;
#[doc = "GPFSEL1 (rw) register accessor: an alias for `Reg<GPFSEL1_SPEC>`"]
pub type GPFSEL1 = crate::Reg<gpfsel1::GPFSEL1_SPEC>;
#[doc = "GPIO Function Select 1"]
pub mod gpfsel1;
#[doc = "GPFSEL2 (rw) register accessor: an alias for `Reg<GPFSEL2_SPEC>`"]
pub type GPFSEL2 = crate::Reg<gpfsel2::GPFSEL2_SPEC>;
#[doc = "GPIO Function Select 2"]
pub mod gpfsel2;
#[doc = "GPFSEL3 (rw) register accessor: an alias for `Reg<GPFSEL3_SPEC>`"]
pub type GPFSEL3 = crate::Reg<gpfsel3::GPFSEL3_SPEC>;
#[doc = "GPIO Function Select 3"]
pub mod gpfsel3;
#[doc = "GPFSEL4 (rw) register accessor: an alias for `Reg<GPFSEL4_SPEC>`"]
pub type GPFSEL4 = crate::Reg<gpfsel4::GPFSEL4_SPEC>;
#[doc = "GPIO Function Select 4"]
pub mod gpfsel4;
#[doc = "GPFSEL5 (rw) register accessor: an alias for `Reg<GPFSEL5_SPEC>`"]
pub type GPFSEL5 = crate::Reg<gpfsel5::GPFSEL5_SPEC>;
#[doc = "GPIO Function Select 5"]
pub mod gpfsel5;
#[doc = "GPSET0 (w) register accessor: an alias for `Reg<GPSET0_SPEC>`"]
pub type GPSET0 = crate::Reg<gpset0::GPSET0_SPEC>;
#[doc = "GPIO Pin Output Set 0"]
pub mod gpset0;
#[doc = "GPSET1 (w) register accessor: an alias for `Reg<GPSET1_SPEC>`"]
pub type GPSET1 = crate::Reg<gpset1::GPSET1_SPEC>;
#[doc = "GPIO Pin Output Set 1"]
pub mod gpset1;
#[doc = "GPCLR0 (w) register accessor: an alias for `Reg<GPCLR0_SPEC>`"]
pub type GPCLR0 = crate::Reg<gpclr0::GPCLR0_SPEC>;
#[doc = "GPIO Pin Output Clear 0"]
pub mod gpclr0;
#[doc = "GPCLR1 (w) register accessor: an alias for `Reg<GPCLR1_SPEC>`"]
pub type GPCLR1 = crate::Reg<gpclr1::GPCLR1_SPEC>;
#[doc = "GPIO Pin Output Clear 1"]
pub mod gpclr1;
#[doc = "GPLEV0 (r) register accessor: an alias for `Reg<GPLEV0_SPEC>`"]
pub type GPLEV0 = crate::Reg<gplev0::GPLEV0_SPEC>;
#[doc = "GPIO Pin Level 0"]
pub mod gplev0;
#[doc = "GPLEV1 (r) register accessor: an alias for `Reg<GPLEV1_SPEC>`"]
pub type GPLEV1 = crate::Reg<gplev1::GPLEV1_SPEC>;
#[doc = "GPIO Pin Level 1"]
pub mod gplev1;
#[doc = "GPEDS0 (rw) register accessor: an alias for `Reg<GPEDS0_SPEC>`"]
pub type GPEDS0 = crate::Reg<gpeds0::GPEDS0_SPEC>;
#[doc = "GPIO Pin Event Detect Status 0"]
pub mod gpeds0;
#[doc = "GPEDS1 (rw) register accessor: an alias for `Reg<GPEDS1_SPEC>`"]
pub type GPEDS1 = crate::Reg<gpeds1::GPEDS1_SPEC>;
#[doc = "GPIO Pin Event Detect Status 1"]
pub mod gpeds1;
#[doc = "GPREN0 (rw) register accessor: an alias for `Reg<GPREN0_SPEC>`"]
pub type GPREN0 = crate::Reg<gpren0::GPREN0_SPEC>;
#[doc = "GPIO Pin Rising Edge Detect Enable 0"]
pub mod gpren0;
#[doc = "GPREN1 (rw) register accessor: an alias for `Reg<GPREN1_SPEC>`"]
pub type GPREN1 = crate::Reg<gpren1::GPREN1_SPEC>;
#[doc = "GPIO Pin Rising Edge Detect Enable 1"]
pub mod gpren1;
#[doc = "GPFEN0 (rw) register accessor: an alias for `Reg<GPFEN0_SPEC>`"]
pub type GPFEN0 = crate::Reg<gpfen0::GPFEN0_SPEC>;
#[doc = "GPIO Pin Falling Edge Detect Enable 0"]
pub mod gpfen0;
#[doc = "GPFEN1 (rw) register accessor: an alias for `Reg<GPFEN1_SPEC>`"]
pub type GPFEN1 = crate::Reg<gpfen1::GPFEN1_SPEC>;
#[doc = "GPIO Pin Falling Edge Detect Enable 1"]
pub mod gpfen1;
#[doc = "GPHEN0 (rw) register accessor: an alias for `Reg<GPHEN0_SPEC>`"]
pub type GPHEN0 = crate::Reg<gphen0::GPHEN0_SPEC>;
#[doc = "GPIO Pin High Detect Enable 0"]
pub mod gphen0;
#[doc = "GPHEN1 (rw) register accessor: an alias for `Reg<GPHEN1_SPEC>`"]
pub type GPHEN1 = crate::Reg<gphen1::GPHEN1_SPEC>;
#[doc = "GPIO Pin High Detect Enable 1"]
pub mod gphen1;
#[doc = "GPLEN0 (rw) register accessor: an alias for `Reg<GPLEN0_SPEC>`"]
pub type GPLEN0 = crate::Reg<gplen0::GPLEN0_SPEC>;
#[doc = "GPIO Pin Low Detect Enable 0"]
pub mod gplen0;
#[doc = "GPLEN1 (rw) register accessor: an alias for `Reg<GPLEN1_SPEC>`"]
pub type GPLEN1 = crate::Reg<gplen1::GPLEN1_SPEC>;
#[doc = "GPIO Pin Low Detect Enable 1"]
pub mod gplen1;
#[doc = "GPAREN0 (rw) register accessor: an alias for `Reg<GPAREN0_SPEC>`"]
pub type GPAREN0 = crate::Reg<gparen0::GPAREN0_SPEC>;
#[doc = "GPIO Pin Async. Rising Edge Detect 0"]
pub mod gparen0;
#[doc = "GPAREN1 (rw) register accessor: an alias for `Reg<GPAREN1_SPEC>`"]
pub type GPAREN1 = crate::Reg<gparen1::GPAREN1_SPEC>;
#[doc = "GPIO Pin Async. Rising Edge Detect 1"]
pub mod gparen1;
#[doc = "GPAFEN0 (rw) register accessor: an alias for `Reg<GPAFEN0_SPEC>`"]
pub type GPAFEN0 = crate::Reg<gpafen0::GPAFEN0_SPEC>;
#[doc = "GPIO Pin Async. Falling Edge Detect 0"]
pub mod gpafen0;
#[doc = "GPAFEN1 (rw) register accessor: an alias for `Reg<GPAFEN1_SPEC>`"]
pub type GPAFEN1 = crate::Reg<gpafen1::GPAFEN1_SPEC>;
#[doc = "GPIO Pin Async. Falling Edge Detect 1"]
pub mod gpafen1;
#[doc = "EXTRA_MUX (rw) register accessor: an alias for `Reg<EXTRA_MUX_SPEC>`"]
pub type EXTRA_MUX = crate::Reg<extra_mux::EXTRA_MUX_SPEC>;
#[doc = "Undocumented multiplexing bits"]
pub mod extra_mux;
#[doc = "GPIO_PUP_PDN_CNTRL_REG0 (rw) register accessor: an alias for `Reg<GPIO_PUP_PDN_CNTRL_REG0_SPEC>`"]
pub type GPIO_PUP_PDN_CNTRL_REG0 =
    crate::Reg<gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL_REG0_SPEC>;
#[doc = "GPIO Pull-up / Pull-down Register 0"]
pub mod gpio_pup_pdn_cntrl_reg0;
#[doc = "GPIO_PUP_PDN_CNTRL_REG1 (rw) register accessor: an alias for `Reg<GPIO_PUP_PDN_CNTRL_REG1_SPEC>`"]
pub type GPIO_PUP_PDN_CNTRL_REG1 =
    crate::Reg<gpio_pup_pdn_cntrl_reg1::GPIO_PUP_PDN_CNTRL_REG1_SPEC>;
#[doc = "GPIO Pull-up / Pull-down Register 1"]
pub mod gpio_pup_pdn_cntrl_reg1;
#[doc = "GPIO_PUP_PDN_CNTRL_REG2 (rw) register accessor: an alias for `Reg<GPIO_PUP_PDN_CNTRL_REG2_SPEC>`"]
pub type GPIO_PUP_PDN_CNTRL_REG2 =
    crate::Reg<gpio_pup_pdn_cntrl_reg2::GPIO_PUP_PDN_CNTRL_REG2_SPEC>;
#[doc = "GPIO Pull-up / Pull-down Register 2"]
pub mod gpio_pup_pdn_cntrl_reg2;
#[doc = "GPIO_PUP_PDN_CNTRL_REG3 (rw) register accessor: an alias for `Reg<GPIO_PUP_PDN_CNTRL_REG3_SPEC>`"]
pub type GPIO_PUP_PDN_CNTRL_REG3 =
    crate::Reg<gpio_pup_pdn_cntrl_reg3::GPIO_PUP_PDN_CNTRL_REG3_SPEC>;
#[doc = "GPIO Pull-up / Pull-down Register 3"]
pub mod gpio_pup_pdn_cntrl_reg3;
