#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct RegisterBlock {
    #[doc = "0x00 - CPU Interface Control"]
    pub gicc_ctlr: GICC_CTLR,
    #[doc = "0x04 - Interrupt Priority Mask"]
    pub gicc_pmr: GICC_PMR,
    #[doc = "0x08 - Binary Point"]
    pub gicc_bpr: GICC_BPR,
    #[doc = "0x0c - Interrupt Acknowledge"]
    pub gicc_iar: GICC_IAR,
    #[doc = "0x10 - End of Interrupt"]
    pub gicc_eoir: GICC_EOIR,
    #[doc = "0x14 - Running Priority"]
    pub gicc_rpr: GICC_RPR,
    #[doc = "0x18 - Highest Priority Pending Interrupt"]
    pub gicc_hppir: GICC_HPPIR,
    #[doc = "0x1c - Aliased Binary Point"]
    pub gicc_abpr: GICC_ABPR,
    #[doc = "0x20 - Aliased Interrupt Acknowledge"]
    pub gicc_aiar: GICC_AIAR,
    #[doc = "0x24 - Aliased End of Interrupt"]
    pub gicc_aeoir: GICC_AEOIR,
    #[doc = "0x28 - Aliased Highest Priority Pending Interrupt"]
    pub gicc_ahppir: GICC_AHPPIR,
    _reserved11: [u8; 0xa4],
    #[doc = "0xd0 - Active Priority"]
    pub gicc_apr0: GICC_APR0,
    _reserved12: [u8; 0x0c],
    #[doc = "0xe0 - Non-Secure Active Priority"]
    pub gicc_nsapr0: GICC_NSAPR0,
    _reserved13: [u8; 0x18],
    #[doc = "0xfc - CPU Interface Identification Register"]
    pub gicc_iidr: GICC_IIDR,
    _reserved14: [u8; 0x0f00],
    #[doc = "0x1000 - Deactivate Interrupt"]
    pub gicc_dir: GICC_DIR,
}
#[doc = "GICC_CTLR (rw) register accessor: CPU Interface Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicc_ctlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicc_ctlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicc_ctlr`]
module"]
pub type GICC_CTLR = crate::Reg<gicc_ctlr::GICC_CTLR_SPEC>;
#[doc = "CPU Interface Control"]
pub mod gicc_ctlr;
#[doc = "GICC_PMR (rw) register accessor: Interrupt Priority Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicc_pmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicc_pmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicc_pmr`]
module"]
pub type GICC_PMR = crate::Reg<gicc_pmr::GICC_PMR_SPEC>;
#[doc = "Interrupt Priority Mask"]
pub mod gicc_pmr;
#[doc = "GICC_BPR (rw) register accessor: Binary Point\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicc_bpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicc_bpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicc_bpr`]
module"]
pub type GICC_BPR = crate::Reg<gicc_bpr::GICC_BPR_SPEC>;
#[doc = "Binary Point"]
pub mod gicc_bpr;
#[doc = "GICC_IAR (r) register accessor: Interrupt Acknowledge\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicc_iar::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicc_iar`]
module"]
pub type GICC_IAR = crate::Reg<gicc_iar::GICC_IAR_SPEC>;
#[doc = "Interrupt Acknowledge"]
pub mod gicc_iar;
#[doc = "GICC_EOIR (w) register accessor: End of Interrupt\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicc_eoir::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicc_eoir`]
module"]
pub type GICC_EOIR = crate::Reg<gicc_eoir::GICC_EOIR_SPEC>;
#[doc = "End of Interrupt"]
pub mod gicc_eoir;
#[doc = "GICC_RPR (r) register accessor: Running Priority\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicc_rpr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicc_rpr`]
module"]
pub type GICC_RPR = crate::Reg<gicc_rpr::GICC_RPR_SPEC>;
#[doc = "Running Priority"]
pub mod gicc_rpr;
#[doc = "GICC_HPPIR (rw) register accessor: Highest Priority Pending Interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicc_hppir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicc_hppir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicc_hppir`]
module"]
pub type GICC_HPPIR = crate::Reg<gicc_hppir::GICC_HPPIR_SPEC>;
#[doc = "Highest Priority Pending Interrupt"]
pub mod gicc_hppir;
#[doc = "GICC_ABPR (rw) register accessor: Aliased Binary Point\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicc_abpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicc_abpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicc_abpr`]
module"]
pub type GICC_ABPR = crate::Reg<gicc_abpr::GICC_ABPR_SPEC>;
#[doc = "Aliased Binary Point"]
pub mod gicc_abpr;
#[doc = "GICC_AIAR (r) register accessor: Aliased Interrupt Acknowledge\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicc_aiar::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicc_aiar`]
module"]
pub type GICC_AIAR = crate::Reg<gicc_aiar::GICC_AIAR_SPEC>;
#[doc = "Aliased Interrupt Acknowledge"]
pub mod gicc_aiar;
#[doc = "GICC_AEOIR (w) register accessor: Aliased End of Interrupt\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicc_aeoir::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicc_aeoir`]
module"]
pub type GICC_AEOIR = crate::Reg<gicc_aeoir::GICC_AEOIR_SPEC>;
#[doc = "Aliased End of Interrupt"]
pub mod gicc_aeoir;
#[doc = "GICC_AHPPIR (r) register accessor: Aliased Highest Priority Pending Interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicc_ahppir::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicc_ahppir`]
module"]
pub type GICC_AHPPIR = crate::Reg<gicc_ahppir::GICC_AHPPIR_SPEC>;
#[doc = "Aliased Highest Priority Pending Interrupt"]
pub mod gicc_ahppir;
#[doc = "GICC_APR0 (rw) register accessor: Active Priority\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicc_apr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicc_apr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicc_apr0`]
module"]
pub type GICC_APR0 = crate::Reg<gicc_apr0::GICC_APR0_SPEC>;
#[doc = "Active Priority"]
pub mod gicc_apr0;
#[doc = "GICC_NSAPR0 (rw) register accessor: Non-Secure Active Priority\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicc_nsapr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicc_nsapr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicc_nsapr0`]
module"]
pub type GICC_NSAPR0 = crate::Reg<gicc_nsapr0::GICC_NSAPR0_SPEC>;
#[doc = "Non-Secure Active Priority"]
pub mod gicc_nsapr0;
#[doc = "GICC_IIDR (rw) register accessor: CPU Interface Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicc_iidr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicc_iidr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicc_iidr`]
module"]
pub type GICC_IIDR = crate::Reg<gicc_iidr::GICC_IIDR_SPEC>;
#[doc = "CPU Interface Identification Register"]
pub mod gicc_iidr;
#[doc = "GICC_DIR (w) register accessor: Deactivate Interrupt\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicc_dir::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicc_dir`]
module"]
pub type GICC_DIR = crate::Reg<gicc_dir::GICC_DIR_SPEC>;
#[doc = "Deactivate Interrupt"]
pub mod gicc_dir;
