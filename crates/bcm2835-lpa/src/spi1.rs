#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control 0"]
    pub cntl0: CNTL0,
    #[doc = "0x04 - Control 1"]
    pub cntl1: CNTL1,
    #[doc = "0x08 - Status"]
    pub stat: STAT,
    #[doc = "0x0c - Read the RXFIFO without removing an entry"]
    pub peek: PEEK,
    #[doc = "0x10..0x20 - Writing to the FIFO will deassert CS at the end of the access"]
    pub io: [IO; 4],
    #[doc = "0x20..0x30 - Writing to the FIFO will maintain CS at the end of the access"]
    pub txhold: [TXHOLD; 4],
}
#[doc = "CNTL0 (rw) register accessor: an alias for `Reg<CNTL0_SPEC>`"]
pub type CNTL0 = crate::Reg<cntl0::CNTL0_SPEC>;
#[doc = "Control 0"]
pub mod cntl0;
#[doc = "CNTL1 (rw) register accessor: an alias for `Reg<CNTL1_SPEC>`"]
pub type CNTL1 = crate::Reg<cntl1::CNTL1_SPEC>;
#[doc = "Control 1"]
pub mod cntl1;
#[doc = "STAT (rw) register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Status"]
pub mod stat;
#[doc = "PEEK (r) register accessor: an alias for `Reg<PEEK_SPEC>`"]
pub type PEEK = crate::Reg<peek::PEEK_SPEC>;
#[doc = "Read the RXFIFO without removing an entry"]
pub mod peek;
#[doc = "IO (rw) register accessor: an alias for `Reg<IO_SPEC>`"]
pub type IO = crate::Reg<io::IO_SPEC>;
#[doc = "Writing to the FIFO will deassert CS at the end of the access"]
pub mod io;
#[doc = "TXHOLD (rw) register accessor: an alias for `Reg<TXHOLD_SPEC>`"]
pub type TXHOLD = crate::Reg<txhold::TXHOLD_SPEC>;
#[doc = "Writing to the FIFO will maintain CS at the end of the access"]
pub mod txhold;
