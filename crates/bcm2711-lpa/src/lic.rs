#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0200],
    #[doc = "0x200 - Basic pending info"]
    pub basic_pending: BASIC_PENDING,
    #[doc = "0x204 - Pending state for interrupts 1 - 31"]
    pub pending_1: PENDING_1,
    #[doc = "0x208 - Pending state for interrupts 32 - 63"]
    pub pending_2: PENDING_2,
    #[doc = "0x20c - FIQ control"]
    pub fiq_control: FIQ_CONTROL,
    #[doc = "0x210 - Enable interrupts 1 - 31"]
    pub enable_1: ENABLE_1,
    #[doc = "0x214 - Enable interrupts 32 - 63"]
    pub enable_2: ENABLE_2,
    #[doc = "0x218 - Enable basic interrupts"]
    pub enable_basic: ENABLE_BASIC,
    #[doc = "0x21c - Disable interrupts 1 - 31"]
    pub disable_1: DISABLE_1,
    #[doc = "0x220 - Disable interrupts 32 - 63"]
    pub disable_2: DISABLE_2,
    #[doc = "0x224 - Disable basic interrupts"]
    pub disable_basic: DISABLE_BASIC,
}
#[doc = "BASIC_PENDING (r) register accessor: an alias for `Reg<BASIC_PENDING_SPEC>`"]
pub type BASIC_PENDING = crate::Reg<basic_pending::BASIC_PENDING_SPEC>;
#[doc = "Basic pending info"]
pub mod basic_pending;
#[doc = "PENDING_1 (r) register accessor: an alias for `Reg<PENDING_1_SPEC>`"]
pub type PENDING_1 = crate::Reg<pending_1::PENDING_1_SPEC>;
#[doc = "Pending state for interrupts 1 - 31"]
pub mod pending_1;
#[doc = "PENDING_2 (r) register accessor: an alias for `Reg<PENDING_2_SPEC>`"]
pub type PENDING_2 = crate::Reg<pending_2::PENDING_2_SPEC>;
#[doc = "Pending state for interrupts 32 - 63"]
pub mod pending_2;
#[doc = "FIQ_CONTROL (rw) register accessor: an alias for `Reg<FIQ_CONTROL_SPEC>`"]
pub type FIQ_CONTROL = crate::Reg<fiq_control::FIQ_CONTROL_SPEC>;
#[doc = "FIQ control"]
pub mod fiq_control;
#[doc = "ENABLE_1 (rw) register accessor: an alias for `Reg<ENABLE_1_SPEC>`"]
pub type ENABLE_1 = crate::Reg<enable_1::ENABLE_1_SPEC>;
#[doc = "Enable interrupts 1 - 31"]
pub mod enable_1;
#[doc = "ENABLE_2 (rw) register accessor: an alias for `Reg<ENABLE_2_SPEC>`"]
pub type ENABLE_2 = crate::Reg<enable_2::ENABLE_2_SPEC>;
#[doc = "Enable interrupts 32 - 63"]
pub mod enable_2;
#[doc = "ENABLE_BASIC (rw) register accessor: an alias for `Reg<ENABLE_BASIC_SPEC>`"]
pub type ENABLE_BASIC = crate::Reg<enable_basic::ENABLE_BASIC_SPEC>;
#[doc = "Enable basic interrupts"]
pub mod enable_basic;
#[doc = "DISABLE_1 (rw) register accessor: an alias for `Reg<DISABLE_1_SPEC>`"]
pub type DISABLE_1 = crate::Reg<disable_1::DISABLE_1_SPEC>;
#[doc = "Disable interrupts 1 - 31"]
pub mod disable_1;
#[doc = "DISABLE_2 (rw) register accessor: an alias for `Reg<DISABLE_2_SPEC>`"]
pub type DISABLE_2 = crate::Reg<disable_2::DISABLE_2_SPEC>;
#[doc = "Disable interrupts 32 - 63"]
pub mod disable_2;
#[doc = "DISABLE_BASIC (rw) register accessor: an alias for `Reg<DISABLE_BASIC_SPEC>`"]
pub type DISABLE_BASIC = crate::Reg<disable_basic::DISABLE_BASIC_SPEC>;
#[doc = "Disable basic interrupts"]
pub mod disable_basic;
