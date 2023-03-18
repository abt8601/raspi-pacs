#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt status"]
    pub irq: IRQ,
    #[doc = "0x04 - Enable sub-peripherals"]
    pub enables: ENABLES,
}
#[doc = "IRQ (rw) register accessor: an alias for `Reg<IRQ_SPEC>`"]
pub type IRQ = crate::Reg<irq::IRQ_SPEC>;
#[doc = "Interrupt status"]
pub mod irq;
#[doc = "ENABLES (rw) register accessor: an alias for `Reg<ENABLES_SPEC>`"]
pub type ENABLES = crate::Reg<enables::ENABLES_SPEC>;
#[doc = "Enable sub-peripherals"]
pub mod enables;
