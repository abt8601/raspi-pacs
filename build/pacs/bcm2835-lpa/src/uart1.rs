#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct RegisterBlock {
    _reserved_0_io: [u8; 0x04],
    _reserved_1_ier: [u8; 0x04],
    iir: IIR,
    lcr: LCR,
    mcr: MCR,
    lsr: LSR,
    msr: MSR,
    scratch: SCRATCH,
    _reserved8: [u8; 0x03],
    cntl: CNTL,
    stat: STAT,
    baud: BAUD,
}
impl RegisterBlock {
    #[doc = "0x00 - Lower bits of baudrate when DLAB is set"]
    #[inline(always)]
    pub const fn baudl(&self) -> &BAUDL {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x00 - I/O Data"]
    #[inline(always)]
    pub const fn io(&self) -> &IO {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x04 - High bits of baudrate when DLAB is set"]
    #[inline(always)]
    pub const fn baudh(&self) -> &BAUDH {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - Interrupt Enable"]
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x08 - Interrupt Identify"]
    #[inline(always)]
    pub const fn iir(&self) -> &IIR {
        &self.iir
    }
    #[doc = "0x0c - Line control"]
    #[inline(always)]
    pub const fn lcr(&self) -> &LCR {
        &self.lcr
    }
    #[doc = "0x10 - Modem Control"]
    #[inline(always)]
    pub const fn mcr(&self) -> &MCR {
        &self.mcr
    }
    #[doc = "0x14 - Line Status"]
    #[inline(always)]
    pub const fn lsr(&self) -> &LSR {
        &self.lsr
    }
    #[doc = "0x18 - Modem Status"]
    #[inline(always)]
    pub const fn msr(&self) -> &MSR {
        &self.msr
    }
    #[doc = "0x1c - Scratch"]
    #[inline(always)]
    pub const fn scratch(&self) -> &SCRATCH {
        &self.scratch
    }
    #[doc = "0x20 - Control"]
    #[inline(always)]
    pub const fn cntl(&self) -> &CNTL {
        &self.cntl
    }
    #[doc = "0x24 - Status"]
    #[inline(always)]
    pub const fn stat(&self) -> &STAT {
        &self.stat
    }
    #[doc = "0x28 - Baudrate"]
    #[inline(always)]
    pub const fn baud(&self) -> &BAUD {
        &self.baud
    }
}
#[doc = "IO (rw) register accessor: I/O Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`io::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`io::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io`]
module"]
pub type IO = crate::Reg<io::IO_SPEC>;
#[doc = "I/O Data"]
pub mod io;
#[doc = "BAUDL (rw) register accessor: Lower bits of baudrate when DLAB is set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`baudl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`baudl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@baudl`]
module"]
pub type BAUDL = crate::Reg<baudl::BAUDL_SPEC>;
#[doc = "Lower bits of baudrate when DLAB is set"]
pub mod baudl;
#[doc = "IER (rw) register accessor: Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable"]
pub mod ier;
#[doc = "BAUDH (rw) register accessor: High bits of baudrate when DLAB is set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`baudh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`baudh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@baudh`]
module"]
pub type BAUDH = crate::Reg<baudh::BAUDH_SPEC>;
#[doc = "High bits of baudrate when DLAB is set"]
pub mod baudh;
#[doc = "IIR (rw) register accessor: Interrupt Identify\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iir`]
module"]
pub type IIR = crate::Reg<iir::IIR_SPEC>;
#[doc = "Interrupt Identify"]
pub mod iir;
#[doc = "LCR (rw) register accessor: Line control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcr`]
module"]
pub type LCR = crate::Reg<lcr::LCR_SPEC>;
#[doc = "Line control"]
pub mod lcr;
#[doc = "MCR (rw) register accessor: Modem Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr`]
module"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "Modem Control"]
pub mod mcr;
#[doc = "LSR (rw) register accessor: Line Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsr`]
module"]
pub type LSR = crate::Reg<lsr::LSR_SPEC>;
#[doc = "Line Status"]
pub mod lsr;
#[doc = "MSR (rw) register accessor: Modem Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msr`]
module"]
pub type MSR = crate::Reg<msr::MSR_SPEC>;
#[doc = "Modem Status"]
pub mod msr;
#[doc = "SCRATCH (rw) register accessor: Scratch\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scratch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scratch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scratch`]
module"]
pub type SCRATCH = crate::Reg<scratch::SCRATCH_SPEC>;
#[doc = "Scratch"]
pub mod scratch;
#[doc = "CNTL (rw) register accessor: Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntl`]
module"]
pub type CNTL = crate::Reg<cntl::CNTL_SPEC>;
#[doc = "Control"]
pub mod cntl;
#[doc = "STAT (rw) register accessor: Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Status"]
pub mod stat;
#[doc = "BAUD (rw) register accessor: Baudrate\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`baud::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`baud::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@baud`]
module"]
pub type BAUD = crate::Reg<baud::BAUD_SPEC>;
#[doc = "Baudrate"]
pub mod baud;
