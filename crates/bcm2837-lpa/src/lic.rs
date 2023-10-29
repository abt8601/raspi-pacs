#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
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
#[doc = "BASIC_PENDING (r) register accessor: Basic pending info\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`basic_pending::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@basic_pending`]
module"]
pub type BASIC_PENDING = crate::Reg<basic_pending::BASIC_PENDING_SPEC>;
#[doc = "Basic pending info"]
pub mod basic_pending;
#[doc = "PENDING_1 (r) register accessor: Pending state for interrupts 1 - 31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pending_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pending_1`]
module"]
pub type PENDING_1 = crate::Reg<pending_1::PENDING_1_SPEC>;
#[doc = "Pending state for interrupts 1 - 31"]
pub mod pending_1;
#[doc = "PENDING_2 (r) register accessor: Pending state for interrupts 32 - 63\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pending_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pending_2`]
module"]
pub type PENDING_2 = crate::Reg<pending_2::PENDING_2_SPEC>;
#[doc = "Pending state for interrupts 32 - 63"]
pub mod pending_2;
#[doc = "FIQ_CONTROL (rw) register accessor: FIQ control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fiq_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fiq_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fiq_control`]
module"]
pub type FIQ_CONTROL = crate::Reg<fiq_control::FIQ_CONTROL_SPEC>;
#[doc = "FIQ control"]
pub mod fiq_control;
#[doc = "ENABLE_1 (rw) register accessor: Enable interrupts 1 - 31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable_1`]
module"]
pub type ENABLE_1 = crate::Reg<enable_1::ENABLE_1_SPEC>;
#[doc = "Enable interrupts 1 - 31"]
pub mod enable_1;
#[doc = "ENABLE_2 (rw) register accessor: Enable interrupts 32 - 63\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable_2`]
module"]
pub type ENABLE_2 = crate::Reg<enable_2::ENABLE_2_SPEC>;
#[doc = "Enable interrupts 32 - 63"]
pub mod enable_2;
#[doc = "ENABLE_BASIC (rw) register accessor: Enable basic interrupts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable_basic::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_basic::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable_basic`]
module"]
pub type ENABLE_BASIC = crate::Reg<enable_basic::ENABLE_BASIC_SPEC>;
#[doc = "Enable basic interrupts"]
pub mod enable_basic;
#[doc = "DISABLE_1 (rw) register accessor: Disable interrupts 1 - 31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`disable_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`disable_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@disable_1`]
module"]
pub type DISABLE_1 = crate::Reg<disable_1::DISABLE_1_SPEC>;
#[doc = "Disable interrupts 1 - 31"]
pub mod disable_1;
#[doc = "DISABLE_2 (rw) register accessor: Disable interrupts 32 - 63\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`disable_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`disable_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@disable_2`]
module"]
pub type DISABLE_2 = crate::Reg<disable_2::DISABLE_2_SPEC>;
#[doc = "Disable interrupts 32 - 63"]
pub mod disable_2;
#[doc = "DISABLE_BASIC (rw) register accessor: Disable basic interrupts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`disable_basic::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`disable_basic::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@disable_basic`]
module"]
pub type DISABLE_BASIC = crate::Reg<disable_basic::DISABLE_BASIC_SPEC>;
#[doc = "Disable basic interrupts"]
pub mod disable_basic;
