#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct RegisterBlock {
    cntl0: CNTL0,
    cntl1: CNTL1,
    stat: STAT,
    peek: PEEK,
    io: [IO; 4],
    txhold: [TXHOLD; 4],
}
impl RegisterBlock {
    #[doc = "0x00 - Control 0"]
    #[inline(always)]
    pub const fn cntl0(&self) -> &CNTL0 {
        &self.cntl0
    }
    #[doc = "0x04 - Control 1"]
    #[inline(always)]
    pub const fn cntl1(&self) -> &CNTL1 {
        &self.cntl1
    }
    #[doc = "0x08 - Status"]
    #[inline(always)]
    pub const fn stat(&self) -> &STAT {
        &self.stat
    }
    #[doc = "0x0c - Read the RXFIFO without removing an entry"]
    #[inline(always)]
    pub const fn peek(&self) -> &PEEK {
        &self.peek
    }
    #[doc = "0x10..0x20 - Writing to the FIFO will deassert CS at the end of the access"]
    #[inline(always)]
    pub const fn io(&self, n: usize) -> &IO {
        &self.io[n]
    }
    #[doc = "0x20..0x30 - Writing to the FIFO will maintain CS at the end of the access"]
    #[inline(always)]
    pub const fn txhold(&self, n: usize) -> &TXHOLD {
        &self.txhold[n]
    }
}
#[doc = "CNTL0 (rw) register accessor: Control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntl0`]
module"]
pub type CNTL0 = crate::Reg<cntl0::CNTL0_SPEC>;
#[doc = "Control 0"]
pub mod cntl0;
#[doc = "CNTL1 (rw) register accessor: Control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntl1`]
module"]
pub type CNTL1 = crate::Reg<cntl1::CNTL1_SPEC>;
#[doc = "Control 1"]
pub mod cntl1;
#[doc = "STAT (rw) register accessor: Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Status"]
pub mod stat;
#[doc = "PEEK (r) register accessor: Read the RXFIFO without removing an entry\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peek::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peek`]
module"]
pub type PEEK = crate::Reg<peek::PEEK_SPEC>;
#[doc = "Read the RXFIFO without removing an entry"]
pub mod peek;
#[doc = "IO (rw) register accessor: Writing to the FIFO will deassert CS at the end of the access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`io::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`io::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io`]
module"]
pub type IO = crate::Reg<io::IO_SPEC>;
#[doc = "Writing to the FIFO will deassert CS at the end of the access"]
pub mod io;
#[doc = "TXHOLD (rw) register accessor: Writing to the FIFO will maintain CS at the end of the access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txhold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txhold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txhold`]
module"]
pub type TXHOLD = crate::Reg<txhold::TXHOLD_SPEC>;
#[doc = "Writing to the FIFO will maintain CS at the end of the access"]
pub mod txhold;
