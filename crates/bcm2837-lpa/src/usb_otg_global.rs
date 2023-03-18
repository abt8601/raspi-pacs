#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_HS control and status register"]
    pub gotgctl: GOTGCTL,
    #[doc = "0x04 - OTG_HS interrupt register"]
    pub gotgint: GOTGINT,
    #[doc = "0x08 - OTG_HS AHB configuration register"]
    pub gahbcfg: GAHBCFG,
    #[doc = "0x0c - OTG_HS USB configuration register"]
    pub gusbcfg: GUSBCFG,
    #[doc = "0x10 - OTG_HS reset register"]
    pub grstctl: GRSTCTL,
    #[doc = "0x14 - OTG_HS core interrupt register"]
    pub gintsts: GINTSTS,
    #[doc = "0x18 - OTG_HS interrupt mask register"]
    pub gintmsk: GINTMSK,
    _reserved_7_grxstsr: [u8; 0x04],
    _reserved_8_grxstsp: [u8; 0x04],
    #[doc = "0x24 - OTG_HS Receive FIFO size register"]
    pub grxfsiz: GRXFSIZ,
    _reserved_10_gnptxfsiz_host: [u8; 0x04],
    #[doc = "0x2c - OTG_HS nonperiodic transmit FIFO/queue status register"]
    pub gnptxsts: GNPTXSTS,
    _reserved12: [u8; 0x08],
    #[doc = "0x38 - OTG_HS general core configuration register"]
    pub gccfg: GCCFG,
    #[doc = "0x3c - OTG_HS core ID register"]
    pub cid: CID,
    #[doc = "0x40 - OTG_HS vendor ID register"]
    pub vid: VID,
    #[doc = "0x44 - Direction"]
    pub hw_direction: HW_DIRECTION,
    #[doc = "0x48 - Hardware Config 0"]
    pub hw_config0: HW_CONFIG0,
    _reserved17: [u8; 0xb4],
    #[doc = "0x100 - OTG_HS Host periodic transmit FIFO size register"]
    pub hptxfsiz: HPTXFSIZ,
    #[doc = "0x104 - OTG_HS device IN endpoint transmit FIFO size register"]
    pub dieptxf1: DIEPTXF1,
    #[doc = "0x108 - OTG_HS device IN endpoint transmit FIFO size register"]
    pub dieptxf2: DIEPTXF2,
    _reserved20: [u8; 0x10],
    #[doc = "0x11c - OTG_HS device IN endpoint transmit FIFO size register"]
    pub dieptxf3: DIEPTXF3,
    #[doc = "0x120 - OTG_HS device IN endpoint transmit FIFO size register"]
    pub dieptxf4: DIEPTXF4,
    #[doc = "0x124 - OTG_HS device IN endpoint transmit FIFO size register"]
    pub dieptxf5: DIEPTXF5,
    #[doc = "0x128 - OTG_HS device IN endpoint transmit FIFO size register"]
    pub dieptxf6: DIEPTXF6,
    #[doc = "0x12c - OTG_HS device IN endpoint transmit FIFO size register"]
    pub dieptxf7: DIEPTXF7,
}
impl RegisterBlock {
    #[doc = "0x1c - OTG_HS Receive status debug read register (peripheral mode mode)"]
    #[inline(always)]
    pub const fn grxstsr_peripheral(&self) -> &GRXSTSR_PERIPHERAL {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    #[doc = "0x1c - OTG_HS Receive status debug read register (host mode)"]
    #[inline(always)]
    pub const fn grxstsr_host(&self) -> &GRXSTSR_HOST {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    #[doc = "0x20 - OTG_HS status read and pop register (peripheral mode)"]
    #[inline(always)]
    pub const fn grxstsp_peripheral(&self) -> &GRXSTSP_PERIPHERAL {
        unsafe { &*(self as *const Self).cast::<u8>().add(32usize).cast() }
    }
    #[doc = "0x20 - OTG_HS status read and pop register (host mode)"]
    #[inline(always)]
    pub const fn grxstsp_host(&self) -> &GRXSTSP_HOST {
        unsafe { &*(self as *const Self).cast::<u8>().add(32usize).cast() }
    }
    #[doc = "0x28 - Endpoint 0 transmit FIFO size (peripheral mode)"]
    #[inline(always)]
    pub const fn tx0fsiz_peripheral(&self) -> &TX0FSIZ_PERIPHERAL {
        unsafe { &*(self as *const Self).cast::<u8>().add(40usize).cast() }
    }
    #[doc = "0x28 - OTG_HS nonperiodic transmit FIFO size register (host mode)"]
    #[inline(always)]
    pub const fn gnptxfsiz_host(&self) -> &GNPTXFSIZ_HOST {
        unsafe { &*(self as *const Self).cast::<u8>().add(40usize).cast() }
    }
}
#[doc = "GOTGCTL (rw) register accessor: an alias for `Reg<GOTGCTL_SPEC>`"]
pub type GOTGCTL = crate::Reg<gotgctl::GOTGCTL_SPEC>;
#[doc = "OTG_HS control and status register"]
pub mod gotgctl;
#[doc = "GOTGINT (rw) register accessor: an alias for `Reg<GOTGINT_SPEC>`"]
pub type GOTGINT = crate::Reg<gotgint::GOTGINT_SPEC>;
#[doc = "OTG_HS interrupt register"]
pub mod gotgint;
#[doc = "GAHBCFG (rw) register accessor: an alias for `Reg<GAHBCFG_SPEC>`"]
pub type GAHBCFG = crate::Reg<gahbcfg::GAHBCFG_SPEC>;
#[doc = "OTG_HS AHB configuration register"]
pub mod gahbcfg;
#[doc = "GUSBCFG (rw) register accessor: an alias for `Reg<GUSBCFG_SPEC>`"]
pub type GUSBCFG = crate::Reg<gusbcfg::GUSBCFG_SPEC>;
#[doc = "OTG_HS USB configuration register"]
pub mod gusbcfg;
#[doc = "GRSTCTL (rw) register accessor: an alias for `Reg<GRSTCTL_SPEC>`"]
pub type GRSTCTL = crate::Reg<grstctl::GRSTCTL_SPEC>;
#[doc = "OTG_HS reset register"]
pub mod grstctl;
#[doc = "GINTSTS (rw) register accessor: an alias for `Reg<GINTSTS_SPEC>`"]
pub type GINTSTS = crate::Reg<gintsts::GINTSTS_SPEC>;
#[doc = "OTG_HS core interrupt register"]
pub mod gintsts;
#[doc = "GINTMSK (rw) register accessor: an alias for `Reg<GINTMSK_SPEC>`"]
pub type GINTMSK = crate::Reg<gintmsk::GINTMSK_SPEC>;
#[doc = "OTG_HS interrupt mask register"]
pub mod gintmsk;
#[doc = "GRXSTSR_Host (r) register accessor: an alias for `Reg<GRXSTSR_HOST_SPEC>`"]
pub type GRXSTSR_HOST = crate::Reg<grxstsr_host::GRXSTSR_HOST_SPEC>;
#[doc = "OTG_HS Receive status debug read register (host mode)"]
pub mod grxstsr_host;
#[doc = "GRXSTSP_Host (r) register accessor: an alias for `Reg<GRXSTSP_HOST_SPEC>`"]
pub type GRXSTSP_HOST = crate::Reg<grxstsp_host::GRXSTSP_HOST_SPEC>;
#[doc = "OTG_HS status read and pop register (host mode)"]
pub mod grxstsp_host;
#[doc = "GRXFSIZ (rw) register accessor: an alias for `Reg<GRXFSIZ_SPEC>`"]
pub type GRXFSIZ = crate::Reg<grxfsiz::GRXFSIZ_SPEC>;
#[doc = "OTG_HS Receive FIFO size register"]
pub mod grxfsiz;
#[doc = "GNPTXFSIZ_Host (rw) register accessor: an alias for `Reg<GNPTXFSIZ_HOST_SPEC>`"]
pub type GNPTXFSIZ_HOST = crate::Reg<gnptxfsiz_host::GNPTXFSIZ_HOST_SPEC>;
#[doc = "OTG_HS nonperiodic transmit FIFO size register (host mode)"]
pub mod gnptxfsiz_host;
#[doc = "TX0FSIZ_Peripheral (rw) register accessor: an alias for `Reg<TX0FSIZ_PERIPHERAL_SPEC>`"]
pub type TX0FSIZ_PERIPHERAL = crate::Reg<tx0fsiz_peripheral::TX0FSIZ_PERIPHERAL_SPEC>;
#[doc = "Endpoint 0 transmit FIFO size (peripheral mode)"]
pub mod tx0fsiz_peripheral;
#[doc = "GNPTXSTS (r) register accessor: an alias for `Reg<GNPTXSTS_SPEC>`"]
pub type GNPTXSTS = crate::Reg<gnptxsts::GNPTXSTS_SPEC>;
#[doc = "OTG_HS nonperiodic transmit FIFO/queue status register"]
pub mod gnptxsts;
#[doc = "GCCFG (rw) register accessor: an alias for `Reg<GCCFG_SPEC>`"]
pub type GCCFG = crate::Reg<gccfg::GCCFG_SPEC>;
#[doc = "OTG_HS general core configuration register"]
pub mod gccfg;
#[doc = "CID (rw) register accessor: an alias for `Reg<CID_SPEC>`"]
pub type CID = crate::Reg<cid::CID_SPEC>;
#[doc = "OTG_HS core ID register"]
pub mod cid;
#[doc = "VID (r) register accessor: an alias for `Reg<VID_SPEC>`"]
pub type VID = crate::Reg<vid::VID_SPEC>;
#[doc = "OTG_HS vendor ID register"]
pub mod vid;
#[doc = "HW_DIRECTION (r) register accessor: an alias for `Reg<HW_DIRECTION_SPEC>`"]
pub type HW_DIRECTION = crate::Reg<hw_direction::HW_DIRECTION_SPEC>;
#[doc = "Direction"]
pub mod hw_direction;
#[doc = "HW_CONFIG0 (r) register accessor: an alias for `Reg<HW_CONFIG0_SPEC>`"]
pub type HW_CONFIG0 = crate::Reg<hw_config0::HW_CONFIG0_SPEC>;
#[doc = "Hardware Config 0"]
pub mod hw_config0;
#[doc = "HPTXFSIZ (rw) register accessor: an alias for `Reg<HPTXFSIZ_SPEC>`"]
pub type HPTXFSIZ = crate::Reg<hptxfsiz::HPTXFSIZ_SPEC>;
#[doc = "OTG_HS Host periodic transmit FIFO size register"]
pub mod hptxfsiz;
#[doc = "DIEPTXF1 (rw) register accessor: an alias for `Reg<DIEPTXF1_SPEC>`"]
pub type DIEPTXF1 = crate::Reg<dieptxf1::DIEPTXF1_SPEC>;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod dieptxf1;
#[doc = "DIEPTXF2 (rw) register accessor: an alias for `Reg<DIEPTXF2_SPEC>`"]
pub type DIEPTXF2 = crate::Reg<dieptxf2::DIEPTXF2_SPEC>;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod dieptxf2;
#[doc = "DIEPTXF3 (rw) register accessor: an alias for `Reg<DIEPTXF3_SPEC>`"]
pub type DIEPTXF3 = crate::Reg<dieptxf3::DIEPTXF3_SPEC>;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod dieptxf3;
#[doc = "DIEPTXF4 (rw) register accessor: an alias for `Reg<DIEPTXF4_SPEC>`"]
pub type DIEPTXF4 = crate::Reg<dieptxf4::DIEPTXF4_SPEC>;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod dieptxf4;
#[doc = "DIEPTXF5 (rw) register accessor: an alias for `Reg<DIEPTXF5_SPEC>`"]
pub type DIEPTXF5 = crate::Reg<dieptxf5::DIEPTXF5_SPEC>;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod dieptxf5;
#[doc = "DIEPTXF6 (rw) register accessor: an alias for `Reg<DIEPTXF6_SPEC>`"]
pub type DIEPTXF6 = crate::Reg<dieptxf6::DIEPTXF6_SPEC>;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod dieptxf6;
#[doc = "DIEPTXF7 (rw) register accessor: an alias for `Reg<DIEPTXF7_SPEC>`"]
pub type DIEPTXF7 = crate::Reg<dieptxf7::DIEPTXF7_SPEC>;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod dieptxf7;
#[doc = "GRXSTSR_Peripheral (r) register accessor: an alias for `Reg<GRXSTSR_PERIPHERAL_SPEC>`"]
pub type GRXSTSR_PERIPHERAL = crate::Reg<grxstsr_peripheral::GRXSTSR_PERIPHERAL_SPEC>;
#[doc = "OTG_HS Receive status debug read register (peripheral mode mode)"]
pub mod grxstsr_peripheral;
#[doc = "GRXSTSP_Peripheral (r) register accessor: an alias for `Reg<GRXSTSP_PERIPHERAL_SPEC>`"]
pub type GRXSTSP_PERIPHERAL = crate::Reg<grxstsp_peripheral::GRXSTSP_PERIPHERAL_SPEC>;
#[doc = "OTG_HS status read and pop register (peripheral mode)"]
pub mod grxstsp_peripheral;
