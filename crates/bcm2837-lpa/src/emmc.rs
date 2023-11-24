#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct RegisterBlock {
    arg2: ARG2,
    blksizecnt: BLKSIZECNT,
    arg1: ARG1,
    cmdtm: CMDTM,
    resp0: RESP0,
    resp1: RESP1,
    resp2: RESP2,
    resp3: RESP3,
    data: DATA,
    status: STATUS,
    control0: CONTROL0,
    control1: CONTROL1,
    interrupt: INTERRUPT,
    irpt_mask: IRPT_MASK,
    irpt_en: IRPT_EN,
    control2: CONTROL2,
    _reserved16: [u8; 0x10],
    force_irpt: FORCE_IRPT,
    _reserved17: [u8; 0x1c],
    boot_timeout: BOOT_TIMEOUT,
    dbg_sel: DBG_SEL,
    _reserved19: [u8; 0x08],
    exrdfifo_cfg: EXRDFIFO_CFG,
    exrdfifo_en: EXRDFIFO_EN,
    tune_step: TUNE_STEP,
    tune_steps_std: TUNE_STEPS_STD,
    tune_steps_ddr: TUNE_STEPS_DDR,
    _reserved24: [u8; 0x5c],
    spi_int_spt: SPI_INT_SPT,
    _reserved25: [u8; 0x08],
    slotisr_ver: SLOTISR_VER,
}
impl RegisterBlock {
    #[doc = "0x00 - Argument for ACMD23 command"]
    #[inline(always)]
    pub const fn arg2(&self) -> &ARG2 {
        &self.arg2
    }
    #[doc = "0x04 - Numer and size in bytes for data block to be transferred"]
    #[inline(always)]
    pub const fn blksizecnt(&self) -> &BLKSIZECNT {
        &self.blksizecnt
    }
    #[doc = "0x08 - Argument for everything but ACMD23"]
    #[inline(always)]
    pub const fn arg1(&self) -> &ARG1 {
        &self.arg1
    }
    #[doc = "0x0c - Issue commands to the card"]
    #[inline(always)]
    pub const fn cmdtm(&self) -> &CMDTM {
        &self.cmdtm
    }
    #[doc = "0x10 - Status bits of the response"]
    #[inline(always)]
    pub const fn resp0(&self) -> &RESP0 {
        &self.resp0
    }
    #[doc = "0x14 - Bits 63:32 of CMD2 and CMD10 responses"]
    #[inline(always)]
    pub const fn resp1(&self) -> &RESP1 {
        &self.resp1
    }
    #[doc = "0x18 - Bits 95:64 of CMD2 and CMD10 responses"]
    #[inline(always)]
    pub const fn resp2(&self) -> &RESP2 {
        &self.resp2
    }
    #[doc = "0x1c - Bits 127:96 of CMD2 and CMD10 responses"]
    #[inline(always)]
    pub const fn resp3(&self) -> &RESP3 {
        &self.resp3
    }
    #[doc = "0x20 - Data to/from the card"]
    #[inline(always)]
    pub const fn data(&self) -> &DATA {
        &self.data
    }
    #[doc = "0x24 - Status info for debugging"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x28 - Control"]
    #[inline(always)]
    pub const fn control0(&self) -> &CONTROL0 {
        &self.control0
    }
    #[doc = "0x2c - Configure"]
    #[inline(always)]
    pub const fn control1(&self) -> &CONTROL1 {
        &self.control1
    }
    #[doc = "0x30 - Interrupt flags"]
    #[inline(always)]
    pub const fn interrupt(&self) -> &INTERRUPT {
        &self.interrupt
    }
    #[doc = "0x34 - Mask interrupts that change in INTERRUPT"]
    #[inline(always)]
    pub const fn irpt_mask(&self) -> &IRPT_MASK {
        &self.irpt_mask
    }
    #[doc = "0x38 - Enable interrupt to core"]
    #[inline(always)]
    pub const fn irpt_en(&self) -> &IRPT_EN {
        &self.irpt_en
    }
    #[doc = "0x3c - Control 2"]
    #[inline(always)]
    pub const fn control2(&self) -> &CONTROL2 {
        &self.control2
    }
    #[doc = "0x50 - Force an interrupt"]
    #[inline(always)]
    pub const fn force_irpt(&self) -> &FORCE_IRPT {
        &self.force_irpt
    }
    #[doc = "0x70 - Number of SD clock cycles to wait for boot"]
    #[inline(always)]
    pub const fn boot_timeout(&self) -> &BOOT_TIMEOUT {
        &self.boot_timeout
    }
    #[doc = "0x74 - What submodules are accessed by the debug bus"]
    #[inline(always)]
    pub const fn dbg_sel(&self) -> &DBG_SEL {
        &self.dbg_sel
    }
    #[doc = "0x80 - Fine tune DMA request generation"]
    #[inline(always)]
    pub const fn exrdfifo_cfg(&self) -> &EXRDFIFO_CFG {
        &self.exrdfifo_cfg
    }
    #[doc = "0x84 - Enable the extension data register"]
    #[inline(always)]
    pub const fn exrdfifo_en(&self) -> &EXRDFIFO_EN {
        &self.exrdfifo_en
    }
    #[doc = "0x88 - Sample clock delay step duration"]
    #[inline(always)]
    pub const fn tune_step(&self) -> &TUNE_STEP {
        &self.tune_step
    }
    #[doc = "0x8c - Sample clock delay step count for SDR"]
    #[inline(always)]
    pub const fn tune_steps_std(&self) -> &TUNE_STEPS_STD {
        &self.tune_steps_std
    }
    #[doc = "0x90 - Sample clock delay step count for DDR"]
    #[inline(always)]
    pub const fn tune_steps_ddr(&self) -> &TUNE_STEPS_DDR {
        &self.tune_steps_ddr
    }
    #[doc = "0xf0 - Interrupts in SPI mode depend on CS"]
    #[inline(always)]
    pub const fn spi_int_spt(&self) -> &SPI_INT_SPT {
        &self.spi_int_spt
    }
    #[doc = "0xfc - Version information and slot interrupt status"]
    #[inline(always)]
    pub const fn slotisr_ver(&self) -> &SLOTISR_VER {
        &self.slotisr_ver
    }
}
#[doc = "ARG2 (rw) register accessor: Argument for ACMD23 command\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arg2`]
module"]
pub type ARG2 = crate::Reg<arg2::ARG2_SPEC>;
#[doc = "Argument for ACMD23 command"]
pub mod arg2;
#[doc = "BLKSIZECNT (rw) register accessor: Numer and size in bytes for data block to be transferred\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blksizecnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blksizecnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blksizecnt`]
module"]
pub type BLKSIZECNT = crate::Reg<blksizecnt::BLKSIZECNT_SPEC>;
#[doc = "Numer and size in bytes for data block to be transferred"]
pub mod blksizecnt;
#[doc = "ARG1 (rw) register accessor: Argument for everything but ACMD23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arg1`]
module"]
pub type ARG1 = crate::Reg<arg1::ARG1_SPEC>;
#[doc = "Argument for everything but ACMD23"]
pub mod arg1;
#[doc = "CMDTM (rw) register accessor: Issue commands to the card\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdtm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdtm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdtm`]
module"]
pub type CMDTM = crate::Reg<cmdtm::CMDTM_SPEC>;
#[doc = "Issue commands to the card"]
pub mod cmdtm;
#[doc = "RESP0 (rw) register accessor: Status bits of the response\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`resp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp0`]
module"]
pub type RESP0 = crate::Reg<resp0::RESP0_SPEC>;
#[doc = "Status bits of the response"]
pub mod resp0;
#[doc = "RESP1 (rw) register accessor: Bits 63:32 of CMD2 and CMD10 responses\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`resp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp1`]
module"]
pub type RESP1 = crate::Reg<resp1::RESP1_SPEC>;
#[doc = "Bits 63:32 of CMD2 and CMD10 responses"]
pub mod resp1;
#[doc = "RESP2 (rw) register accessor: Bits 95:64 of CMD2 and CMD10 responses\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`resp2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp2`]
module"]
pub type RESP2 = crate::Reg<resp2::RESP2_SPEC>;
#[doc = "Bits 95:64 of CMD2 and CMD10 responses"]
pub mod resp2;
#[doc = "RESP3 (rw) register accessor: Bits 127:96 of CMD2 and CMD10 responses\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`resp3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp3`]
module"]
pub type RESP3 = crate::Reg<resp3::RESP3_SPEC>;
#[doc = "Bits 127:96 of CMD2 and CMD10 responses"]
pub mod resp3;
#[doc = "DATA (rw) register accessor: Data to/from the card\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Data to/from the card"]
pub mod data;
#[doc = "STATUS (rw) register accessor: Status info for debugging\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status info for debugging"]
pub mod status;
#[doc = "CONTROL0 (rw) register accessor: Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`control0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`control0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control0`]
module"]
pub type CONTROL0 = crate::Reg<control0::CONTROL0_SPEC>;
#[doc = "Control"]
pub mod control0;
#[doc = "CONTROL1 (rw) register accessor: Configure\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`control1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`control1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control1`]
module"]
pub type CONTROL1 = crate::Reg<control1::CONTROL1_SPEC>;
#[doc = "Configure"]
pub mod control1;
#[doc = "INTERRUPT (rw) register accessor: Interrupt flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrupt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt`]
module"]
pub type INTERRUPT = crate::Reg<interrupt::INTERRUPT_SPEC>;
#[doc = "Interrupt flags"]
pub mod interrupt;
#[doc = "IRPT_MASK (rw) register accessor: Mask interrupts that change in INTERRUPT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irpt_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irpt_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irpt_mask`]
module"]
pub type IRPT_MASK = crate::Reg<irpt_mask::IRPT_MASK_SPEC>;
#[doc = "Mask interrupts that change in INTERRUPT"]
pub mod irpt_mask;
#[doc = "IRPT_EN (rw) register accessor: Enable interrupt to core\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irpt_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irpt_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irpt_en`]
module"]
pub type IRPT_EN = crate::Reg<irpt_en::IRPT_EN_SPEC>;
#[doc = "Enable interrupt to core"]
pub mod irpt_en;
#[doc = "CONTROL2 (rw) register accessor: Control 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`control2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`control2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control2`]
module"]
pub type CONTROL2 = crate::Reg<control2::CONTROL2_SPEC>;
#[doc = "Control 2"]
pub mod control2;
#[doc = "FORCE_IRPT (rw) register accessor: Force an interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`force_irpt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`force_irpt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@force_irpt`]
module"]
pub type FORCE_IRPT = crate::Reg<force_irpt::FORCE_IRPT_SPEC>;
#[doc = "Force an interrupt"]
pub mod force_irpt;
#[doc = "BOOT_TIMEOUT (rw) register accessor: Number of SD clock cycles to wait for boot\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`boot_timeout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`boot_timeout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@boot_timeout`]
module"]
pub type BOOT_TIMEOUT = crate::Reg<boot_timeout::BOOT_TIMEOUT_SPEC>;
#[doc = "Number of SD clock cycles to wait for boot"]
pub mod boot_timeout;
#[doc = "DBG_SEL (rw) register accessor: What submodules are accessed by the debug bus\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbg_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbg_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg_sel`]
module"]
pub type DBG_SEL = crate::Reg<dbg_sel::DBG_SEL_SPEC>;
#[doc = "What submodules are accessed by the debug bus"]
pub mod dbg_sel;
#[doc = "EXRDFIFO_CFG (rw) register accessor: Fine tune DMA request generation\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exrdfifo_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exrdfifo_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exrdfifo_cfg`]
module"]
pub type EXRDFIFO_CFG = crate::Reg<exrdfifo_cfg::EXRDFIFO_CFG_SPEC>;
#[doc = "Fine tune DMA request generation"]
pub mod exrdfifo_cfg;
#[doc = "EXRDFIFO_EN (rw) register accessor: Enable the extension data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exrdfifo_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exrdfifo_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exrdfifo_en`]
module"]
pub type EXRDFIFO_EN = crate::Reg<exrdfifo_en::EXRDFIFO_EN_SPEC>;
#[doc = "Enable the extension data register"]
pub mod exrdfifo_en;
#[doc = "TUNE_STEP (rw) register accessor: Sample clock delay step duration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tune_step::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tune_step::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tune_step`]
module"]
pub type TUNE_STEP = crate::Reg<tune_step::TUNE_STEP_SPEC>;
#[doc = "Sample clock delay step duration"]
pub mod tune_step;
#[doc = "TUNE_STEPS_STD (rw) register accessor: Sample clock delay step count for SDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tune_steps_std::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tune_steps_std::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tune_steps_std`]
module"]
pub type TUNE_STEPS_STD = crate::Reg<tune_steps_std::TUNE_STEPS_STD_SPEC>;
#[doc = "Sample clock delay step count for SDR"]
pub mod tune_steps_std;
#[doc = "TUNE_STEPS_DDR (rw) register accessor: Sample clock delay step count for DDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tune_steps_ddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tune_steps_ddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tune_steps_ddr`]
module"]
pub type TUNE_STEPS_DDR = crate::Reg<tune_steps_ddr::TUNE_STEPS_DDR_SPEC>;
#[doc = "Sample clock delay step count for DDR"]
pub mod tune_steps_ddr;
#[doc = "SPI_INT_SPT (rw) register accessor: Interrupts in SPI mode depend on CS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_int_spt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_int_spt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_int_spt`]
module"]
pub type SPI_INT_SPT = crate::Reg<spi_int_spt::SPI_INT_SPT_SPEC>;
#[doc = "Interrupts in SPI mode depend on CS"]
pub mod spi_int_spt;
#[doc = "SLOTISR_VER (rw) register accessor: Version information and slot interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slotisr_ver::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slotisr_ver::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slotisr_ver`]
module"]
pub type SLOTISR_VER = crate::Reg<slotisr_ver::SLOTISR_VER_SPEC>;
#[doc = "Version information and slot interrupt status"]
pub mod slotisr_ver;
