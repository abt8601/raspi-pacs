#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct RegisterBlock {
    dr: DR,
    _reserved_1_ecr: [u8; 0x04],
    _reserved2: [u8; 0x10],
    fr: FR,
    _reserved3: [u8; 0x08],
    ibrd: IBRD,
    fbrd: FBRD,
    lcr_h: LCR_H,
    cr: CR,
    ifls: IFLS,
    imsc: IMSC,
    ris: RIS,
    mis: MIS,
    icr: ICR,
    dmacr: DMACR,
}
impl RegisterBlock {
    #[doc = "0x00 - Data Register"]
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
    #[doc = "0x04 - Error Clear Register"]
    #[inline(always)]
    pub const fn ecr(&self) -> &ECR {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - Receive Status Register"]
    #[inline(always)]
    pub const fn rsr(&self) -> &RSR {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x18 - Flag Register"]
    #[inline(always)]
    pub const fn fr(&self) -> &FR {
        &self.fr
    }
    #[doc = "0x24 - Integer Baud Rate Register"]
    #[inline(always)]
    pub const fn ibrd(&self) -> &IBRD {
        &self.ibrd
    }
    #[doc = "0x28 - Fractional Baud Rate Register"]
    #[inline(always)]
    pub const fn fbrd(&self) -> &FBRD {
        &self.fbrd
    }
    #[doc = "0x2c - Line Control Register"]
    #[inline(always)]
    pub const fn lcr_h(&self) -> &LCR_H {
        &self.lcr_h
    }
    #[doc = "0x30 - Control Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x34 - Interrupt FIFO Level Select Register"]
    #[inline(always)]
    pub const fn ifls(&self) -> &IFLS {
        &self.ifls
    }
    #[doc = "0x38 - Interrupt Mask set_Clear Register"]
    #[inline(always)]
    pub const fn imsc(&self) -> &IMSC {
        &self.imsc
    }
    #[doc = "0x3c - Raw Interrupt Status Register"]
    #[inline(always)]
    pub const fn ris(&self) -> &RIS {
        &self.ris
    }
    #[doc = "0x40 - Masked Interrupt Status Register"]
    #[inline(always)]
    pub const fn mis(&self) -> &MIS {
        &self.mis
    }
    #[doc = "0x44 - Interrupt Clear Register"]
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    #[doc = "0x48 - DMA Control Register"]
    #[inline(always)]
    pub const fn dmacr(&self) -> &DMACR {
        &self.dmacr
    }
}
#[doc = "DR (rw) register accessor: Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`]
module"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "Data Register"]
pub mod dr;
#[doc = "RSR (r) register accessor: Receive Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsr`]
module"]
pub type RSR = crate::Reg<rsr::RSR_SPEC>;
#[doc = "Receive Status Register"]
pub mod rsr;
#[doc = "ECR (w) register accessor: Error Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecr`]
module"]
pub type ECR = crate::Reg<ecr::ECR_SPEC>;
#[doc = "Error Clear Register"]
pub mod ecr;
#[doc = "FR (rw) register accessor: Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fr`]
module"]
pub type FR = crate::Reg<fr::FR_SPEC>;
#[doc = "Flag Register"]
pub mod fr;
#[doc = "IBRD (rw) register accessor: Integer Baud Rate Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ibrd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ibrd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibrd`]
module"]
pub type IBRD = crate::Reg<ibrd::IBRD_SPEC>;
#[doc = "Integer Baud Rate Register"]
pub mod ibrd;
#[doc = "FBRD (rw) register accessor: Fractional Baud Rate Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fbrd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fbrd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fbrd`]
module"]
pub type FBRD = crate::Reg<fbrd::FBRD_SPEC>;
#[doc = "Fractional Baud Rate Register"]
pub mod fbrd;
#[doc = "LCR_H (rw) register accessor: Line Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcr_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcr_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcr_h`]
module"]
pub type LCR_H = crate::Reg<lcr_h::LCR_H_SPEC>;
#[doc = "Line Control Register"]
pub mod lcr_h;
#[doc = "CR (rw) register accessor: Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "IFLS (rw) register accessor: Interrupt FIFO Level Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ifls::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifls::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifls`]
module"]
pub type IFLS = crate::Reg<ifls::IFLS_SPEC>;
#[doc = "Interrupt FIFO Level Select Register"]
pub mod ifls;
#[doc = "IMSC (rw) register accessor: Interrupt Mask set_Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imsc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imsc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imsc`]
module"]
pub type IMSC = crate::Reg<imsc::IMSC_SPEC>;
#[doc = "Interrupt Mask set_Clear Register"]
pub mod imsc;
#[doc = "RIS (r) register accessor: Raw Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris`]
module"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "Raw Interrupt Status Register"]
pub mod ris;
#[doc = "MIS (r) register accessor: Masked Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mis`]
module"]
pub type MIS = crate::Reg<mis::MIS_SPEC>;
#[doc = "Masked Interrupt Status Register"]
pub mod mis;
#[doc = "ICR (w) register accessor: Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "Interrupt Clear Register"]
pub mod icr;
#[doc = "DMACR (rw) register accessor: DMA Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacr`]
module"]
pub type DMACR = crate::Reg<dmacr::DMACR_SPEC>;
#[doc = "DMA Control Register"]
pub mod dmacr;
