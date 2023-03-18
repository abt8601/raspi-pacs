#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_io: [u8; 0x04],
    _reserved_1_ier: [u8; 0x04],
    #[doc = "0x08 - Interrupt Identify"]
    pub iir: IIR,
    #[doc = "0x0c - Line control"]
    pub lcr: LCR,
    #[doc = "0x10 - Modem Control"]
    pub mcr: MCR,
    #[doc = "0x14 - Line Status"]
    pub lsr: LSR,
    #[doc = "0x18 - Modem Status"]
    pub msr: MSR,
    #[doc = "0x1c - Scratch"]
    pub scratch: SCRATCH,
    _reserved8: [u8; 0x03],
    #[doc = "0x20 - Control"]
    pub cntl: CNTL,
    #[doc = "0x24 - Status"]
    pub stat: STAT,
    #[doc = "0x28 - Baudrate"]
    pub baud: BAUD,
}
impl RegisterBlock {
    #[doc = "0x00 - Lower bits of baudrate when DLAB is set"]
    #[inline(always)]
    pub const fn baudl(&self) -> &BAUDL {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00 - I/O Data"]
    #[inline(always)]
    pub const fn io(&self) -> &IO {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x04 - High bits of baudrate when DLAB is set"]
    #[inline(always)]
    pub const fn baudh(&self) -> &BAUDH {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x04 - Interrupt Enable"]
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
}
#[doc = "IO (rw) register accessor: an alias for `Reg<IO_SPEC>`"]
pub type IO = crate::Reg<io::IO_SPEC>;
#[doc = "I/O Data"]
pub mod io;
#[doc = "BAUDL (rw) register accessor: an alias for `Reg<BAUDL_SPEC>`"]
pub type BAUDL = crate::Reg<baudl::BAUDL_SPEC>;
#[doc = "Lower bits of baudrate when DLAB is set"]
pub mod baudl;
#[doc = "IER (rw) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable"]
pub mod ier;
#[doc = "BAUDH (rw) register accessor: an alias for `Reg<BAUDH_SPEC>`"]
pub type BAUDH = crate::Reg<baudh::BAUDH_SPEC>;
#[doc = "High bits of baudrate when DLAB is set"]
pub mod baudh;
#[doc = "IIR (rw) register accessor: an alias for `Reg<IIR_SPEC>`"]
pub type IIR = crate::Reg<iir::IIR_SPEC>;
#[doc = "Interrupt Identify"]
pub mod iir;
#[doc = "LCR (rw) register accessor: an alias for `Reg<LCR_SPEC>`"]
pub type LCR = crate::Reg<lcr::LCR_SPEC>;
#[doc = "Line control"]
pub mod lcr;
#[doc = "MCR (rw) register accessor: an alias for `Reg<MCR_SPEC>`"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "Modem Control"]
pub mod mcr;
#[doc = "LSR (rw) register accessor: an alias for `Reg<LSR_SPEC>`"]
pub type LSR = crate::Reg<lsr::LSR_SPEC>;
#[doc = "Line Status"]
pub mod lsr;
#[doc = "MSR (rw) register accessor: an alias for `Reg<MSR_SPEC>`"]
pub type MSR = crate::Reg<msr::MSR_SPEC>;
#[doc = "Modem Status"]
pub mod msr;
#[doc = "SCRATCH (rw) register accessor: an alias for `Reg<SCRATCH_SPEC>`"]
pub type SCRATCH = crate::Reg<scratch::SCRATCH_SPEC>;
#[doc = "Scratch"]
pub mod scratch;
#[doc = "CNTL (rw) register accessor: an alias for `Reg<CNTL_SPEC>`"]
pub type CNTL = crate::Reg<cntl::CNTL_SPEC>;
#[doc = "Control"]
pub mod cntl;
#[doc = "STAT (rw) register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Status"]
pub mod stat;
#[doc = "BAUD (rw) register accessor: an alias for `Reg<BAUD_SPEC>`"]
pub type BAUD = crate::Reg<baud::BAUD_SPEC>;
#[doc = "Baudrate"]
pub mod baud;
