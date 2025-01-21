#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct RegisterBlock {
    gicd_ctlr: GICD_CTLR,
    gicd_typer: GICD_TYPER,
    gicd_iidr: GICD_IIDR,
    _reserved3: [u8; 0x74],
    gicd_igroupr: GICD_IGROUPR,
    _reserved4: [u8; 0x64],
    gicd_isenabler: GICD_ISENABLER,
    _reserved5: [u8; 0x64],
    gicd_icenabler: GICD_ICENABLER,
    _reserved6: [u8; 0x64],
    gicd_ispendr: GICD_ISPENDR,
    _reserved7: [u8; 0x64],
    gicd_icpendr: GICD_ICPENDR,
    _reserved8: [u8; 0x64],
    gicd_isactiver: GICD_ISACTIVER,
    _reserved9: [u8; 0x64],
    gicd_icactiver: GICD_ICACTIVER,
    _reserved10: [u8; 0x64],
    gicd_ipriorityr: GICD_IPRIORITYR,
    _reserved11: [u8; 0x0320],
    gicd_itargetsr: GICD_ITARGETSR,
    _reserved12: [u8; 0x0320],
    gicd_icfgr: GICD_ICFGR,
    _reserved13: [u8; 0xc8],
    gicd_ppisr: GICD_PPISR,
    gicd_spisr0: GICD_SPISR0,
    gicd_spisr1: GICD_SPISR1,
    gicd_spisr2: GICD_SPISR2,
    gicd_spisr3: GICD_SPISR3,
    gicd_spisr4: GICD_SPISR4,
    gicd_spisr5: GICD_SPISR5,
    _reserved20: [u8; 0x01e4],
    gicd_sgir: GICD_SGIR,
    _reserved21: [u8; 0x0c],
    gicd_cpendsgirn: GICD_CPENDSGIRN,
    _reserved22: [u8; 0x0c],
    gicd_spendsgirn: GICD_SPENDSGIRN,
    _reserved23: [u8; 0xac],
    gicd_pidr4: GICD_PIDR4,
    gicd_pidr5: GICD_PIDR5,
    gicd_pidr6: GICD_PIDR6,
    gicd_pidr7: GICD_PIDR7,
    gicd_pidr0: GICD_PIDR0,
    gicd_pidr1: GICD_PIDR1,
    gicd_pidr2: GICD_PIDR2,
    gicd_pidr3: GICD_PIDR3,
    gicd_cidr0: GICD_CIDR0,
    gicd_cidr1: GICD_CIDR1,
    gicd_cidr2: GICD_CIDR2,
    gicd_cidr3: GICD_CIDR3,
}
impl RegisterBlock {
    #[doc = "0x00 - Distributor Control Register"]
    #[inline(always)]
    pub const fn gicd_ctlr(&self) -> &GICD_CTLR {
        &self.gicd_ctlr
    }
    #[doc = "0x04 - Interrupt Controller Type Register"]
    #[inline(always)]
    pub const fn gicd_typer(&self) -> &GICD_TYPER {
        &self.gicd_typer
    }
    #[doc = "0x08 - Distributor Implementer Identification Register"]
    #[inline(always)]
    pub const fn gicd_iidr(&self) -> &GICD_IIDR {
        &self.gicd_iidr
    }
    #[doc = "0x80..0x9c - Interrupt Group Registers"]
    #[inline(always)]
    pub const fn gicd_igroupr(&self) -> &GICD_IGROUPR {
        &self.gicd_igroupr
    }
    #[doc = "0x100..0x11c - Interrupt Set-Enable Registers"]
    #[inline(always)]
    pub const fn gicd_isenabler(&self) -> &GICD_ISENABLER {
        &self.gicd_isenabler
    }
    #[doc = "0x180..0x19c - Interrupt Clear-Enable Registers"]
    #[inline(always)]
    pub const fn gicd_icenabler(&self) -> &GICD_ICENABLER {
        &self.gicd_icenabler
    }
    #[doc = "0x200..0x21c - Interrupt Set-Pending Registers"]
    #[inline(always)]
    pub const fn gicd_ispendr(&self) -> &GICD_ISPENDR {
        &self.gicd_ispendr
    }
    #[doc = "0x280..0x29c - Interrupt Clear-Pending Registers"]
    #[inline(always)]
    pub const fn gicd_icpendr(&self) -> &GICD_ICPENDR {
        &self.gicd_icpendr
    }
    #[doc = "0x300..0x31c - Interrupt Set-Active Registers"]
    #[inline(always)]
    pub const fn gicd_isactiver(&self) -> &GICD_ISACTIVER {
        &self.gicd_isactiver
    }
    #[doc = "0x380..0x39c - Interrupt Clear-Active Registers"]
    #[inline(always)]
    pub const fn gicd_icactiver(&self) -> &GICD_ICACTIVER {
        &self.gicd_icactiver
    }
    #[doc = "0x400..0x4e0 - Interrupt Priority"]
    #[inline(always)]
    pub const fn gicd_ipriorityr(&self) -> &GICD_IPRIORITYR {
        &self.gicd_ipriorityr
    }
    #[doc = "0x800..0x8e0 - Interrupt Processor Targets"]
    #[inline(always)]
    pub const fn gicd_itargetsr(&self) -> &GICD_ITARGETSR {
        &self.gicd_itargetsr
    }
    #[doc = "0xc00..0xc38 - Interrupt Configuration"]
    #[inline(always)]
    pub const fn gicd_icfgr(&self) -> &GICD_ICFGR {
        &self.gicd_icfgr
    }
    #[doc = "0xd00 - Private Peripheral Interrupt Status Register"]
    #[inline(always)]
    pub const fn gicd_ppisr(&self) -> &GICD_PPISR {
        &self.gicd_ppisr
    }
    #[doc = "0xd04 - Shared Peripheral Interrupt Status Registers"]
    #[inline(always)]
    pub const fn gicd_spisr0(&self) -> &GICD_SPISR0 {
        &self.gicd_spisr0
    }
    #[doc = "0xd08 - Shared Peripheral Interrupt Status Registers"]
    #[inline(always)]
    pub const fn gicd_spisr1(&self) -> &GICD_SPISR1 {
        &self.gicd_spisr1
    }
    #[doc = "0xd0c - Shared Peripheral Interrupt Status Registers"]
    #[inline(always)]
    pub const fn gicd_spisr2(&self) -> &GICD_SPISR2 {
        &self.gicd_spisr2
    }
    #[doc = "0xd10 - Shared Peripheral Interrupt Status Registers"]
    #[inline(always)]
    pub const fn gicd_spisr3(&self) -> &GICD_SPISR3 {
        &self.gicd_spisr3
    }
    #[doc = "0xd14 - Shared Peripheral Interrupt Status Registers"]
    #[inline(always)]
    pub const fn gicd_spisr4(&self) -> &GICD_SPISR4 {
        &self.gicd_spisr4
    }
    #[doc = "0xd18 - Shared Peripheral Interrupt Status Registers"]
    #[inline(always)]
    pub const fn gicd_spisr5(&self) -> &GICD_SPISR5 {
        &self.gicd_spisr5
    }
    #[doc = "0xf00 - Software Generated Interrupt Register"]
    #[inline(always)]
    pub const fn gicd_sgir(&self) -> &GICD_SGIR {
        &self.gicd_sgir
    }
    #[doc = "0xf10 - SGI Clear-Pending Registers"]
    #[inline(always)]
    pub const fn gicd_cpendsgirn(&self) -> &GICD_CPENDSGIRN {
        &self.gicd_cpendsgirn
    }
    #[doc = "0xf20 - SGI Set-Pending Registers"]
    #[inline(always)]
    pub const fn gicd_spendsgirn(&self) -> &GICD_SPENDSGIRN {
        &self.gicd_spendsgirn
    }
    #[doc = "0xfd0 - Peripheral ID 4"]
    #[inline(always)]
    pub const fn gicd_pidr4(&self) -> &GICD_PIDR4 {
        &self.gicd_pidr4
    }
    #[doc = "0xfd4 - Peripheral ID 5"]
    #[inline(always)]
    pub const fn gicd_pidr5(&self) -> &GICD_PIDR5 {
        &self.gicd_pidr5
    }
    #[doc = "0xfd8 - Peripheral ID 6"]
    #[inline(always)]
    pub const fn gicd_pidr6(&self) -> &GICD_PIDR6 {
        &self.gicd_pidr6
    }
    #[doc = "0xfdc - Peripheral ID 7"]
    #[inline(always)]
    pub const fn gicd_pidr7(&self) -> &GICD_PIDR7 {
        &self.gicd_pidr7
    }
    #[doc = "0xfe0 - Peripheral ID 0"]
    #[inline(always)]
    pub const fn gicd_pidr0(&self) -> &GICD_PIDR0 {
        &self.gicd_pidr0
    }
    #[doc = "0xfe4 - Peripheral ID 1"]
    #[inline(always)]
    pub const fn gicd_pidr1(&self) -> &GICD_PIDR1 {
        &self.gicd_pidr1
    }
    #[doc = "0xfe8 - Peripheral ID 2"]
    #[inline(always)]
    pub const fn gicd_pidr2(&self) -> &GICD_PIDR2 {
        &self.gicd_pidr2
    }
    #[doc = "0xfec - Peripheral ID 3"]
    #[inline(always)]
    pub const fn gicd_pidr3(&self) -> &GICD_PIDR3 {
        &self.gicd_pidr3
    }
    #[doc = "0xff0 - Component ID 0"]
    #[inline(always)]
    pub const fn gicd_cidr0(&self) -> &GICD_CIDR0 {
        &self.gicd_cidr0
    }
    #[doc = "0xff4 - Component ID 1"]
    #[inline(always)]
    pub const fn gicd_cidr1(&self) -> &GICD_CIDR1 {
        &self.gicd_cidr1
    }
    #[doc = "0xff8 - Component ID 2"]
    #[inline(always)]
    pub const fn gicd_cidr2(&self) -> &GICD_CIDR2 {
        &self.gicd_cidr2
    }
    #[doc = "0xffc - Component ID 3"]
    #[inline(always)]
    pub const fn gicd_cidr3(&self) -> &GICD_CIDR3 {
        &self.gicd_cidr3
    }
}
#[doc = "GICD_CTLR (rw) register accessor: Distributor Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ctlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ctlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ctlr`]
module"]
pub type GICD_CTLR = crate::Reg<gicd_ctlr::GICD_CTLR_SPEC>;
#[doc = "Distributor Control Register"]
pub mod gicd_ctlr;
#[doc = "GICD_TYPER (r) register accessor: Interrupt Controller Type Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_typer::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_typer`]
module"]
pub type GICD_TYPER = crate::Reg<gicd_typer::GICD_TYPER_SPEC>;
#[doc = "Interrupt Controller Type Register"]
pub mod gicd_typer;
#[doc = "GICD_IIDR (r) register accessor: Distributor Implementer Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_iidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_iidr`]
module"]
pub type GICD_IIDR = crate::Reg<gicd_iidr::GICD_IIDR_SPEC>;
#[doc = "Distributor Implementer Identification Register"]
pub mod gicd_iidr;
#[doc = "Interrupt Group Registers"]
pub use self::gicd_igroupr::GICD_IGROUPR;
#[doc = r"Cluster"]
#[doc = "Interrupt Group Registers"]
pub mod gicd_igroupr;
#[doc = "Interrupt Set-Enable Registers"]
pub use self::gicd_isenabler::GICD_ISENABLER;
#[doc = r"Cluster"]
#[doc = "Interrupt Set-Enable Registers"]
pub mod gicd_isenabler;
#[doc = "Interrupt Clear-Enable Registers"]
pub use self::gicd_icenabler::GICD_ICENABLER;
#[doc = r"Cluster"]
#[doc = "Interrupt Clear-Enable Registers"]
pub mod gicd_icenabler;
#[doc = "Interrupt Set-Pending Registers"]
pub use self::gicd_ispendr::GICD_ISPENDR;
#[doc = r"Cluster"]
#[doc = "Interrupt Set-Pending Registers"]
pub mod gicd_ispendr;
#[doc = "Interrupt Clear-Pending Registers"]
pub use self::gicd_icpendr::GICD_ICPENDR;
#[doc = r"Cluster"]
#[doc = "Interrupt Clear-Pending Registers"]
pub mod gicd_icpendr;
#[doc = "Interrupt Set-Active Registers"]
pub use self::gicd_isactiver::GICD_ISACTIVER;
#[doc = r"Cluster"]
#[doc = "Interrupt Set-Active Registers"]
pub mod gicd_isactiver;
#[doc = "Interrupt Clear-Active Registers"]
pub use self::gicd_icactiver::GICD_ICACTIVER;
#[doc = r"Cluster"]
#[doc = "Interrupt Clear-Active Registers"]
pub mod gicd_icactiver;
#[doc = "Interrupt Priority"]
pub use self::gicd_ipriorityr::GICD_IPRIORITYR;
#[doc = r"Cluster"]
#[doc = "Interrupt Priority"]
pub mod gicd_ipriorityr;
#[doc = "Interrupt Processor Targets"]
pub use self::gicd_itargetsr::GICD_ITARGETSR;
#[doc = r"Cluster"]
#[doc = "Interrupt Processor Targets"]
pub mod gicd_itargetsr;
#[doc = "Interrupt Configuration"]
pub use self::gicd_icfgr::GICD_ICFGR;
#[doc = r"Cluster"]
#[doc = "Interrupt Configuration"]
pub mod gicd_icfgr;
#[doc = "GICD_PPISR (rw) register accessor: Private Peripheral Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ppisr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ppisr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_ppisr`]
module"]
pub type GICD_PPISR = crate::Reg<gicd_ppisr::GICD_PPISR_SPEC>;
#[doc = "Private Peripheral Interrupt Status Register"]
pub mod gicd_ppisr;
#[doc = "GICD_SPISR0 (rw) register accessor: Shared Peripheral Interrupt Status Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_spisr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_spisr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_spisr0`]
module"]
pub type GICD_SPISR0 = crate::Reg<gicd_spisr0::GICD_SPISR0_SPEC>;
#[doc = "Shared Peripheral Interrupt Status Registers"]
pub mod gicd_spisr0;
#[doc = "GICD_SPISR1 (rw) register accessor: Shared Peripheral Interrupt Status Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_spisr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_spisr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_spisr1`]
module"]
pub type GICD_SPISR1 = crate::Reg<gicd_spisr1::GICD_SPISR1_SPEC>;
#[doc = "Shared Peripheral Interrupt Status Registers"]
pub mod gicd_spisr1;
#[doc = "GICD_SPISR2 (rw) register accessor: Shared Peripheral Interrupt Status Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_spisr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_spisr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_spisr2`]
module"]
pub type GICD_SPISR2 = crate::Reg<gicd_spisr2::GICD_SPISR2_SPEC>;
#[doc = "Shared Peripheral Interrupt Status Registers"]
pub mod gicd_spisr2;
#[doc = "GICD_SPISR3 (rw) register accessor: Shared Peripheral Interrupt Status Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_spisr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_spisr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_spisr3`]
module"]
pub type GICD_SPISR3 = crate::Reg<gicd_spisr3::GICD_SPISR3_SPEC>;
#[doc = "Shared Peripheral Interrupt Status Registers"]
pub mod gicd_spisr3;
#[doc = "GICD_SPISR4 (rw) register accessor: Shared Peripheral Interrupt Status Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_spisr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_spisr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_spisr4`]
module"]
pub type GICD_SPISR4 = crate::Reg<gicd_spisr4::GICD_SPISR4_SPEC>;
#[doc = "Shared Peripheral Interrupt Status Registers"]
pub mod gicd_spisr4;
#[doc = "GICD_SPISR5 (rw) register accessor: Shared Peripheral Interrupt Status Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_spisr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_spisr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_spisr5`]
module"]
pub type GICD_SPISR5 = crate::Reg<gicd_spisr5::GICD_SPISR5_SPEC>;
#[doc = "Shared Peripheral Interrupt Status Registers"]
pub mod gicd_spisr5;
#[doc = "GICD_SGIR (w) register accessor: Software Generated Interrupt Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_sgir::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_sgir`]
module"]
pub type GICD_SGIR = crate::Reg<gicd_sgir::GICD_SGIR_SPEC>;
#[doc = "Software Generated Interrupt Register"]
pub mod gicd_sgir;
#[doc = "GICD_CPENDSGIRn (rw) register accessor: SGI Clear-Pending Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_cpendsgirn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_cpendsgirn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_cpendsgirn`]
module"]
pub type GICD_CPENDSGIRN = crate::Reg<gicd_cpendsgirn::GICD_CPENDSGIRN_SPEC>;
#[doc = "SGI Clear-Pending Registers"]
pub mod gicd_cpendsgirn;
#[doc = "GICD_SPENDSGIRn (rw) register accessor: SGI Set-Pending Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_spendsgirn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_spendsgirn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_spendsgirn`]
module"]
pub type GICD_SPENDSGIRN = crate::Reg<gicd_spendsgirn::GICD_SPENDSGIRN_SPEC>;
#[doc = "SGI Set-Pending Registers"]
pub mod gicd_spendsgirn;
#[doc = "GICD_PIDR4 (r) register accessor: Peripheral ID 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_pidr4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_pidr4`]
module"]
pub type GICD_PIDR4 = crate::Reg<gicd_pidr4::GICD_PIDR4_SPEC>;
#[doc = "Peripheral ID 4"]
pub mod gicd_pidr4;
#[doc = "GICD_PIDR5 (r) register accessor: Peripheral ID 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_pidr5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_pidr5`]
module"]
pub type GICD_PIDR5 = crate::Reg<gicd_pidr5::GICD_PIDR5_SPEC>;
#[doc = "Peripheral ID 5"]
pub mod gicd_pidr5;
#[doc = "GICD_PIDR6 (r) register accessor: Peripheral ID 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_pidr6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_pidr6`]
module"]
pub type GICD_PIDR6 = crate::Reg<gicd_pidr6::GICD_PIDR6_SPEC>;
#[doc = "Peripheral ID 6"]
pub mod gicd_pidr6;
#[doc = "GICD_PIDR7 (r) register accessor: Peripheral ID 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_pidr7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_pidr7`]
module"]
pub type GICD_PIDR7 = crate::Reg<gicd_pidr7::GICD_PIDR7_SPEC>;
#[doc = "Peripheral ID 7"]
pub mod gicd_pidr7;
#[doc = "GICD_PIDR0 (r) register accessor: Peripheral ID 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_pidr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_pidr0`]
module"]
pub type GICD_PIDR0 = crate::Reg<gicd_pidr0::GICD_PIDR0_SPEC>;
#[doc = "Peripheral ID 0"]
pub mod gicd_pidr0;
#[doc = "GICD_PIDR1 (r) register accessor: Peripheral ID 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_pidr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_pidr1`]
module"]
pub type GICD_PIDR1 = crate::Reg<gicd_pidr1::GICD_PIDR1_SPEC>;
#[doc = "Peripheral ID 1"]
pub mod gicd_pidr1;
#[doc = "GICD_PIDR2 (r) register accessor: Peripheral ID 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_pidr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_pidr2`]
module"]
pub type GICD_PIDR2 = crate::Reg<gicd_pidr2::GICD_PIDR2_SPEC>;
#[doc = "Peripheral ID 2"]
pub mod gicd_pidr2;
#[doc = "GICD_PIDR3 (r) register accessor: Peripheral ID 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_pidr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_pidr3`]
module"]
pub type GICD_PIDR3 = crate::Reg<gicd_pidr3::GICD_PIDR3_SPEC>;
#[doc = "Peripheral ID 3"]
pub mod gicd_pidr3;
#[doc = "GICD_CIDR0 (r) register accessor: Component ID 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_cidr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_cidr0`]
module"]
pub type GICD_CIDR0 = crate::Reg<gicd_cidr0::GICD_CIDR0_SPEC>;
#[doc = "Component ID 0"]
pub mod gicd_cidr0;
#[doc = "GICD_CIDR1 (r) register accessor: Component ID 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_cidr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_cidr1`]
module"]
pub type GICD_CIDR1 = crate::Reg<gicd_cidr1::GICD_CIDR1_SPEC>;
#[doc = "Component ID 1"]
pub mod gicd_cidr1;
#[doc = "GICD_CIDR2 (r) register accessor: Component ID 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_cidr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_cidr2`]
module"]
pub type GICD_CIDR2 = crate::Reg<gicd_cidr2::GICD_CIDR2_SPEC>;
#[doc = "Component ID 2"]
pub mod gicd_cidr2;
#[doc = "GICD_CIDR3 (r) register accessor: Component ID 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_cidr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gicd_cidr3`]
module"]
pub type GICD_CIDR3 = crate::Reg<gicd_cidr3::GICD_CIDR3_SPEC>;
#[doc = "Component ID 3"]
pub mod gicd_cidr3;
