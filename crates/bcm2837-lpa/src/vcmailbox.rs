#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct RegisterBlock {
    read: READ,
    _reserved1: [u8; 0x0c],
    peek0: PEEK0,
    sender0: SENDER0,
    status0: STATUS0,
    config0: CONFIG0,
    write: WRITE,
    _reserved6: [u8; 0x0c],
    peek1: PEEK1,
    sender1: SENDER1,
    status1: STATUS1,
    config1: CONFIG1,
}
impl RegisterBlock {
    #[doc = "0x00 - Read messages from the VideoCore"]
    #[inline(always)]
    pub const fn read(&self) -> &READ {
        &self.read
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn peek0(&self) -> &PEEK0 {
        &self.peek0
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn sender0(&self) -> &SENDER0 {
        &self.sender0
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn status0(&self) -> &STATUS0 {
        &self.status0
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn config0(&self) -> &CONFIG0 {
        &self.config0
    }
    #[doc = "0x20 - Write messages to the VideoCore"]
    #[inline(always)]
    pub const fn write(&self) -> &WRITE {
        &self.write
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn peek1(&self) -> &PEEK1 {
        &self.peek1
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn sender1(&self) -> &SENDER1 {
        &self.sender1
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn status1(&self) -> &STATUS1 {
        &self.status1
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn config1(&self) -> &CONFIG1 {
        &self.config1
    }
}
#[doc = "READ (r) register accessor: Read messages from the VideoCore\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`read::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@read`]
module"]
pub type READ = crate::Reg<read::READ_SPEC>;
#[doc = "Read messages from the VideoCore"]
pub mod read;
#[doc = "PEEK0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peek0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peek0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peek0`]
module"]
pub type PEEK0 = crate::Reg<peek0::PEEK0_SPEC>;
#[doc = ""]
pub mod peek0;
#[doc = "SENDER0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sender0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sender0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sender0`]
module"]
pub type SENDER0 = crate::Reg<sender0::SENDER0_SPEC>;
#[doc = ""]
pub mod sender0;
#[doc = "STATUS0 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status0`]
module"]
pub type STATUS0 = crate::Reg<status0::STATUS0_SPEC>;
#[doc = ""]
pub mod status0;
#[doc = "CONFIG0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config0::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config0`]
module"]
pub type CONFIG0 = crate::Reg<config0::CONFIG0_SPEC>;
#[doc = ""]
pub mod config0;
#[doc = "WRITE (w) register accessor: Write messages to the VideoCore\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`write::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@write`]
module"]
pub type WRITE = crate::Reg<write::WRITE_SPEC>;
#[doc = "Write messages to the VideoCore"]
pub mod write;
#[doc = "PEEK1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peek1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peek1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peek1`]
module"]
pub type PEEK1 = crate::Reg<peek1::PEEK1_SPEC>;
#[doc = ""]
pub mod peek1;
#[doc = "SENDER1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sender1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sender1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sender1`]
module"]
pub type SENDER1 = crate::Reg<sender1::SENDER1_SPEC>;
#[doc = ""]
pub mod sender1;
#[doc = "STATUS1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status1`]
module"]
pub type STATUS1 = crate::Reg<status1::STATUS1_SPEC>;
#[doc = ""]
pub mod status1;
#[doc = "CONFIG1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config1::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config1`]
module"]
pub type CONFIG1 = crate::Reg<config1::CONFIG1_SPEC>;
#[doc = ""]
pub mod config1;
