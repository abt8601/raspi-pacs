#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control and Status"]
    pub cs: CS,
    #[doc = "0x04 - FIFO access"]
    pub fifo: FIFO,
    #[doc = "0x08 - Clock divider"]
    pub clk: CLK,
    #[doc = "0x0c - Data length"]
    pub dlen: DLEN,
    #[doc = "0x10 - LoSSI output hold delay"]
    pub ltoh: LTOH,
    #[doc = "0x14 - "]
    pub dc: DC,
}
#[doc = "CS (rw) register accessor: an alias for `Reg<CS_SPEC>`"]
pub type CS = crate::Reg<cs::CS_SPEC>;
#[doc = "Control and Status"]
pub mod cs;
#[doc = "FIFO (rw) register accessor: an alias for `Reg<FIFO_SPEC>`"]
pub type FIFO = crate::Reg<fifo::FIFO_SPEC>;
#[doc = "FIFO access"]
pub mod fifo;
#[doc = "CLK (rw) register accessor: an alias for `Reg<CLK_SPEC>`"]
pub type CLK = crate::Reg<clk::CLK_SPEC>;
#[doc = "Clock divider"]
pub mod clk;
#[doc = "DLEN (rw) register accessor: an alias for `Reg<DLEN_SPEC>`"]
pub type DLEN = crate::Reg<dlen::DLEN_SPEC>;
#[doc = "Data length"]
pub mod dlen;
#[doc = "LTOH (rw) register accessor: an alias for `Reg<LTOH_SPEC>`"]
pub type LTOH = crate::Reg<ltoh::LTOH_SPEC>;
#[doc = "LoSSI output hold delay"]
pub mod ltoh;
#[doc = "DC (rw) register accessor: an alias for `Reg<DC_SPEC>`"]
pub type DC = crate::Reg<dc::DC_SPEC>;
#[doc = ""]
pub mod dc;
