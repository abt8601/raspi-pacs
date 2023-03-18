#[doc = r"Register block"]
#[repr(C)]
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
#[doc = "C (rw) register accessor: an alias for `Reg<C_SPEC>`"]
pub type C = crate::Reg<c::C_SPEC>;
#[doc = "Control"]
pub mod c;
#[doc = "S (rw) register accessor: an alias for `Reg<S_SPEC>`"]
pub type S = crate::Reg<s::S_SPEC>;
#[doc = "Status"]
pub mod s;
#[doc = "DLEN (rw) register accessor: an alias for `Reg<DLEN_SPEC>`"]
pub type DLEN = crate::Reg<dlen::DLEN_SPEC>;
#[doc = "Data length"]
pub mod dlen;
#[doc = "A (rw) register accessor: an alias for `Reg<A_SPEC>`"]
pub type A = crate::Reg<a::A_SPEC>;
#[doc = "Slave address"]
pub mod a;
#[doc = "FIFO (rw) register accessor: an alias for `Reg<FIFO_SPEC>`"]
pub type FIFO = crate::Reg<fifo::FIFO_SPEC>;
#[doc = "Data FIFO"]
pub mod fifo;
#[doc = "DIV (rw) register accessor: an alias for `Reg<DIV_SPEC>`"]
pub type DIV = crate::Reg<div::DIV_SPEC>;
#[doc = "Clock divider"]
pub mod div;
#[doc = "DEL (rw) register accessor: an alias for `Reg<DEL_SPEC>`"]
pub type DEL = crate::Reg<del::DEL_SPEC>;
#[doc = "Data delay (Values must be under CDIV / 2)"]
pub mod del;
#[doc = "CLKT (rw) register accessor: an alias for `Reg<CLKT_SPEC>`"]
pub type CLKT = crate::Reg<clkt::CLKT_SPEC>;
#[doc = "Clock stretch timeout (broken on 283x)"]
pub mod clkt;
