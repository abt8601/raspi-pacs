#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Read messages from the VideoCore"]
    pub read: READ,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - "]
    pub peek0: PEEK0,
    #[doc = "0x14 - "]
    pub sender0: SENDER0,
    #[doc = "0x18 - "]
    pub status0: STATUS0,
    #[doc = "0x1c - "]
    pub config0: CONFIG0,
    #[doc = "0x20 - Write messages to the VideoCore"]
    pub write: WRITE,
    _reserved6: [u8; 0x0c],
    #[doc = "0x30 - "]
    pub peek1: PEEK1,
    #[doc = "0x34 - "]
    pub sender1: SENDER1,
    #[doc = "0x38 - "]
    pub status1: STATUS1,
    #[doc = "0x3c - "]
    pub config1: CONFIG1,
}
#[doc = "READ (r) register accessor: an alias for `Reg<READ_SPEC>`"]
pub type READ = crate::Reg<read::READ_SPEC>;
#[doc = "Read messages from the VideoCore"]
pub mod read;
#[doc = "PEEK0 (rw) register accessor: an alias for `Reg<PEEK0_SPEC>`"]
pub type PEEK0 = crate::Reg<peek0::PEEK0_SPEC>;
#[doc = ""]
pub mod peek0;
#[doc = "SENDER0 (rw) register accessor: an alias for `Reg<SENDER0_SPEC>`"]
pub type SENDER0 = crate::Reg<sender0::SENDER0_SPEC>;
#[doc = ""]
pub mod sender0;
#[doc = "STATUS0 (r) register accessor: an alias for `Reg<STATUS0_SPEC>`"]
pub type STATUS0 = crate::Reg<status0::STATUS0_SPEC>;
#[doc = ""]
pub mod status0;
#[doc = "CONFIG0 (rw) register accessor: an alias for `Reg<CONFIG0_SPEC>`"]
pub type CONFIG0 = crate::Reg<config0::CONFIG0_SPEC>;
#[doc = ""]
pub mod config0;
#[doc = "WRITE (w) register accessor: an alias for `Reg<WRITE_SPEC>`"]
pub type WRITE = crate::Reg<write::WRITE_SPEC>;
#[doc = "Write messages to the VideoCore"]
pub mod write;
#[doc = "PEEK1 (rw) register accessor: an alias for `Reg<PEEK1_SPEC>`"]
pub type PEEK1 = crate::Reg<peek1::PEEK1_SPEC>;
#[doc = ""]
pub mod peek1;
#[doc = "SENDER1 (rw) register accessor: an alias for `Reg<SENDER1_SPEC>`"]
pub type SENDER1 = crate::Reg<sender1::SENDER1_SPEC>;
#[doc = ""]
pub mod sender1;
#[doc = "STATUS1 (rw) register accessor: an alias for `Reg<STATUS1_SPEC>`"]
pub type STATUS1 = crate::Reg<status1::STATUS1_SPEC>;
#[doc = ""]
pub mod status1;
#[doc = "CONFIG1 (rw) register accessor: an alias for `Reg<CONFIG1_SPEC>`"]
pub type CONFIG1 = crate::Reg<config1::CONFIG1_SPEC>;
#[doc = ""]
pub mod config1;
