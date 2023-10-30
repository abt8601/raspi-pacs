#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub c: C,
    #[doc = "0x04 - Status"]
    pub s: S,
    #[doc = "0x08 - Data length"]
    pub dlen: DLEN,
    #[doc = "0x0c - Slave address"]
    pub a: A,
    #[doc = "0x10 - Data FIFO"]
    pub fifo: FIFO,
    #[doc = "0x14 - Clock divider"]
    pub div: DIV,
    #[doc = "0x18 - Data delay (Values must be under CDIV / 2)"]
    pub del: DEL,
    #[doc = "0x1c - Clock stretch timeout (broken on 283x)"]
    pub clkt: CLKT,
}
#[doc = "C (rw) register accessor: Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c`]
module"]
pub type C = crate::Reg<c::C_SPEC>;
#[doc = "Control"]
pub mod c;
#[doc = "S (rw) register accessor: Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s`]
module"]
pub type S = crate::Reg<s::S_SPEC>;
#[doc = "Status"]
pub mod s;
#[doc = "DLEN (rw) register accessor: Data length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dlen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dlen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dlen`]
module"]
pub type DLEN = crate::Reg<dlen::DLEN_SPEC>;
#[doc = "Data length"]
pub mod dlen;
#[doc = "A (rw) register accessor: Slave address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a`]
module"]
pub type A = crate::Reg<a::A_SPEC>;
#[doc = "Slave address"]
pub mod a;
#[doc = "FIFO (rw) register accessor: Data FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo`]
module"]
pub type FIFO = crate::Reg<fifo::FIFO_SPEC>;
#[doc = "Data FIFO"]
pub mod fifo;
#[doc = "DIV (rw) register accessor: Clock divider\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`div::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`div::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@div`]
module"]
pub type DIV = crate::Reg<div::DIV_SPEC>;
#[doc = "Clock divider"]
pub mod div;
#[doc = "DEL (rw) register accessor: Data delay (Values must be under CDIV / 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`del::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`del::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@del`]
module"]
pub type DEL = crate::Reg<del::DEL_SPEC>;
#[doc = "Data delay (Values must be under CDIV / 2)"]
pub mod del;
#[doc = "CLKT (rw) register accessor: Clock stretch timeout (broken on 283x)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkt`]
module"]
pub type CLKT = crate::Reg<clkt::CLKT_SPEC>;
#[doc = "Clock stretch timeout (broken on 283x)"]
pub mod clkt;
