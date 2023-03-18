#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Data Register"]
    pub dr: DR,
    _reserved_1_ecr: [u8; 0x04],
    _reserved2: [u8; 0x10],
    #[doc = "0x18 - Flag Register"]
    pub fr: FR,
    _reserved3: [u8; 0x08],
    #[doc = "0x24 - Integer Baud Rate Register"]
    pub ibrd: IBRD,
    #[doc = "0x28 - Fractional Baud Rate Register"]
    pub fbrd: FBRD,
    #[doc = "0x2c - Line Control Register"]
    pub lcr_h: LCR_H,
    #[doc = "0x30 - Control Register"]
    pub cr: CR,
    #[doc = "0x34 - Interrupt FIFO Level Select Register"]
    pub ifls: IFLS,
    #[doc = "0x38 - Interrupt Mask set_Clear Register"]
    pub imsc: IMSC,
    #[doc = "0x3c - Raw Interrupt Status Register"]
    pub ris: RIS,
    #[doc = "0x40 - Masked Interrupt Status Register"]
    pub mis: MIS,
    #[doc = "0x44 - Interrupt Clear Register"]
    pub icr: ICR,
    #[doc = "0x48 - DMA Control Register"]
    pub dmacr: DMACR,
}
impl RegisterBlock {
    #[doc = "0x04 - Error Clear Register"]
    #[inline(always)]
    pub const fn ecr(&self) -> &ECR {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x04 - Receive Status Register"]
    #[inline(always)]
    pub const fn rsr(&self) -> &RSR {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
}
#[doc = "DR (rw) register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "Data Register"]
pub mod dr;
#[doc = "RSR (r) register accessor: an alias for `Reg<RSR_SPEC>`"]
pub type RSR = crate::Reg<rsr::RSR_SPEC>;
#[doc = "Receive Status Register"]
pub mod rsr;
#[doc = "ECR (w) register accessor: an alias for `Reg<ECR_SPEC>`"]
pub type ECR = crate::Reg<ecr::ECR_SPEC>;
#[doc = "Error Clear Register"]
pub mod ecr;
#[doc = "FR (rw) register accessor: an alias for `Reg<FR_SPEC>`"]
pub type FR = crate::Reg<fr::FR_SPEC>;
#[doc = "Flag Register"]
pub mod fr;
#[doc = "IBRD (rw) register accessor: an alias for `Reg<IBRD_SPEC>`"]
pub type IBRD = crate::Reg<ibrd::IBRD_SPEC>;
#[doc = "Integer Baud Rate Register"]
pub mod ibrd;
#[doc = "FBRD (rw) register accessor: an alias for `Reg<FBRD_SPEC>`"]
pub type FBRD = crate::Reg<fbrd::FBRD_SPEC>;
#[doc = "Fractional Baud Rate Register"]
pub mod fbrd;
#[doc = "LCR_H (rw) register accessor: an alias for `Reg<LCR_H_SPEC>`"]
pub type LCR_H = crate::Reg<lcr_h::LCR_H_SPEC>;
#[doc = "Line Control Register"]
pub mod lcr_h;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "IFLS (rw) register accessor: an alias for `Reg<IFLS_SPEC>`"]
pub type IFLS = crate::Reg<ifls::IFLS_SPEC>;
#[doc = "Interrupt FIFO Level Select Register"]
pub mod ifls;
#[doc = "IMSC (rw) register accessor: an alias for `Reg<IMSC_SPEC>`"]
pub type IMSC = crate::Reg<imsc::IMSC_SPEC>;
#[doc = "Interrupt Mask set_Clear Register"]
pub mod imsc;
#[doc = "RIS (r) register accessor: an alias for `Reg<RIS_SPEC>`"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "Raw Interrupt Status Register"]
pub mod ris;
#[doc = "MIS (r) register accessor: an alias for `Reg<MIS_SPEC>`"]
pub type MIS = crate::Reg<mis::MIS_SPEC>;
#[doc = "Masked Interrupt Status Register"]
pub mod mis;
#[doc = "ICR (w) register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "Interrupt Clear Register"]
pub mod icr;
#[doc = "DMACR (rw) register accessor: an alias for `Reg<DMACR_SPEC>`"]
pub type DMACR = crate::Reg<dmacr::DMACR_SPEC>;
#[doc = "DMA Control Register"]
pub mod dmacr;
