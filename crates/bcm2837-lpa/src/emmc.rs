#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Argument for ACMD23 command"]
    pub arg2: ARG2,
    #[doc = "0x04 - Numer and size in bytes for data block to be transferred"]
    pub blksizecnt: BLKSIZECNT,
    #[doc = "0x08 - Argument for everything but ACMD23"]
    pub arg1: ARG1,
    #[doc = "0x0c - Issue commands to the card"]
    pub cmdtm: CMDTM,
    #[doc = "0x10 - Status bits of the response"]
    pub resp0: RESP0,
    #[doc = "0x14 - Bits 63:32 of CMD2 and CMD10 responses"]
    pub resp1: RESP1,
    #[doc = "0x18 - Bits 95:64 of CMD2 and CMD10 responses"]
    pub resp2: RESP2,
    #[doc = "0x1c - Bits 127:96 of CMD2 and CMD10 responses"]
    pub resp3: RESP3,
    #[doc = "0x20 - Data to/from the card"]
    pub data: DATA,
    #[doc = "0x24 - Status info for debugging"]
    pub status: STATUS,
    #[doc = "0x28 - Control"]
    pub control0: CONTROL0,
    #[doc = "0x2c - Configure"]
    pub control1: CONTROL1,
    #[doc = "0x30 - Interrupt flags"]
    pub interrupt: INTERRUPT,
    #[doc = "0x34 - Mask interrupts that change in INTERRUPT"]
    pub irpt_mask: IRPT_MASK,
    #[doc = "0x38 - Enable interrupt to core"]
    pub irpt_en: IRPT_EN,
    #[doc = "0x3c - Control 2"]
    pub control2: CONTROL2,
    _reserved16: [u8; 0x10],
    #[doc = "0x50 - Force an interrupt"]
    pub force_irpt: FORCE_IRPT,
    _reserved17: [u8; 0x1c],
    #[doc = "0x70 - Number of SD clock cycles to wait for boot"]
    pub boot_timeout: BOOT_TIMEOUT,
    #[doc = "0x74 - What submodules are accessed by the debug bus"]
    pub dbg_sel: DBG_SEL,
    _reserved19: [u8; 0x08],
    #[doc = "0x80 - Fine tune DMA request generation"]
    pub exrdfifo_cfg: EXRDFIFO_CFG,
    #[doc = "0x84 - Enable the extension data register"]
    pub exrdfifo_en: EXRDFIFO_EN,
    #[doc = "0x88 - Sample clock delay step duration"]
    pub tune_step: TUNE_STEP,
    #[doc = "0x8c - Sample clock delay step count for SDR"]
    pub tune_steps_std: TUNE_STEPS_STD,
    #[doc = "0x90 - Sample clock delay step count for DDR"]
    pub tune_steps_ddr: TUNE_STEPS_DDR,
    _reserved24: [u8; 0x5c],
    #[doc = "0xf0 - Interrupts in SPI mode depend on CS"]
    pub spi_int_spt: SPI_INT_SPT,
    _reserved25: [u8; 0x08],
    #[doc = "0xfc - Version information and slot interrupt status"]
    pub slotisr_ver: SLOTISR_VER,
}
#[doc = "ARG2 (rw) register accessor: an alias for `Reg<ARG2_SPEC>`"]
pub type ARG2 = crate::Reg<arg2::ARG2_SPEC>;
#[doc = "Argument for ACMD23 command"]
pub mod arg2;
#[doc = "BLKSIZECNT (rw) register accessor: an alias for `Reg<BLKSIZECNT_SPEC>`"]
pub type BLKSIZECNT = crate::Reg<blksizecnt::BLKSIZECNT_SPEC>;
#[doc = "Numer and size in bytes for data block to be transferred"]
pub mod blksizecnt;
#[doc = "ARG1 (rw) register accessor: an alias for `Reg<ARG1_SPEC>`"]
pub type ARG1 = crate::Reg<arg1::ARG1_SPEC>;
#[doc = "Argument for everything but ACMD23"]
pub mod arg1;
#[doc = "CMDTM (rw) register accessor: an alias for `Reg<CMDTM_SPEC>`"]
pub type CMDTM = crate::Reg<cmdtm::CMDTM_SPEC>;
#[doc = "Issue commands to the card"]
pub mod cmdtm;
#[doc = "RESP0 (rw) register accessor: an alias for `Reg<RESP0_SPEC>`"]
pub type RESP0 = crate::Reg<resp0::RESP0_SPEC>;
#[doc = "Status bits of the response"]
pub mod resp0;
#[doc = "RESP1 (rw) register accessor: an alias for `Reg<RESP1_SPEC>`"]
pub type RESP1 = crate::Reg<resp1::RESP1_SPEC>;
#[doc = "Bits 63:32 of CMD2 and CMD10 responses"]
pub mod resp1;
#[doc = "RESP2 (rw) register accessor: an alias for `Reg<RESP2_SPEC>`"]
pub type RESP2 = crate::Reg<resp2::RESP2_SPEC>;
#[doc = "Bits 95:64 of CMD2 and CMD10 responses"]
pub mod resp2;
#[doc = "RESP3 (rw) register accessor: an alias for `Reg<RESP3_SPEC>`"]
pub type RESP3 = crate::Reg<resp3::RESP3_SPEC>;
#[doc = "Bits 127:96 of CMD2 and CMD10 responses"]
pub mod resp3;
#[doc = "DATA (rw) register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Data to/from the card"]
pub mod data;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status info for debugging"]
pub mod status;
#[doc = "CONTROL0 (rw) register accessor: an alias for `Reg<CONTROL0_SPEC>`"]
pub type CONTROL0 = crate::Reg<control0::CONTROL0_SPEC>;
#[doc = "Control"]
pub mod control0;
#[doc = "CONTROL1 (rw) register accessor: an alias for `Reg<CONTROL1_SPEC>`"]
pub type CONTROL1 = crate::Reg<control1::CONTROL1_SPEC>;
#[doc = "Configure"]
pub mod control1;
#[doc = "INTERRUPT (rw) register accessor: an alias for `Reg<INTERRUPT_SPEC>`"]
pub type INTERRUPT = crate::Reg<interrupt::INTERRUPT_SPEC>;
#[doc = "Interrupt flags"]
pub mod interrupt;
#[doc = "IRPT_MASK (rw) register accessor: an alias for `Reg<IRPT_MASK_SPEC>`"]
pub type IRPT_MASK = crate::Reg<irpt_mask::IRPT_MASK_SPEC>;
#[doc = "Mask interrupts that change in INTERRUPT"]
pub mod irpt_mask;
#[doc = "IRPT_EN (rw) register accessor: an alias for `Reg<IRPT_EN_SPEC>`"]
pub type IRPT_EN = crate::Reg<irpt_en::IRPT_EN_SPEC>;
#[doc = "Enable interrupt to core"]
pub mod irpt_en;
#[doc = "CONTROL2 (rw) register accessor: an alias for `Reg<CONTROL2_SPEC>`"]
pub type CONTROL2 = crate::Reg<control2::CONTROL2_SPEC>;
#[doc = "Control 2"]
pub mod control2;
#[doc = "FORCE_IRPT (rw) register accessor: an alias for `Reg<FORCE_IRPT_SPEC>`"]
pub type FORCE_IRPT = crate::Reg<force_irpt::FORCE_IRPT_SPEC>;
#[doc = "Force an interrupt"]
pub mod force_irpt;
#[doc = "BOOT_TIMEOUT (rw) register accessor: an alias for `Reg<BOOT_TIMEOUT_SPEC>`"]
pub type BOOT_TIMEOUT = crate::Reg<boot_timeout::BOOT_TIMEOUT_SPEC>;
#[doc = "Number of SD clock cycles to wait for boot"]
pub mod boot_timeout;
#[doc = "DBG_SEL (rw) register accessor: an alias for `Reg<DBG_SEL_SPEC>`"]
pub type DBG_SEL = crate::Reg<dbg_sel::DBG_SEL_SPEC>;
#[doc = "What submodules are accessed by the debug bus"]
pub mod dbg_sel;
#[doc = "EXRDFIFO_CFG (rw) register accessor: an alias for `Reg<EXRDFIFO_CFG_SPEC>`"]
pub type EXRDFIFO_CFG = crate::Reg<exrdfifo_cfg::EXRDFIFO_CFG_SPEC>;
#[doc = "Fine tune DMA request generation"]
pub mod exrdfifo_cfg;
#[doc = "EXRDFIFO_EN (rw) register accessor: an alias for `Reg<EXRDFIFO_EN_SPEC>`"]
pub type EXRDFIFO_EN = crate::Reg<exrdfifo_en::EXRDFIFO_EN_SPEC>;
#[doc = "Enable the extension data register"]
pub mod exrdfifo_en;
#[doc = "TUNE_STEP (rw) register accessor: an alias for `Reg<TUNE_STEP_SPEC>`"]
pub type TUNE_STEP = crate::Reg<tune_step::TUNE_STEP_SPEC>;
#[doc = "Sample clock delay step duration"]
pub mod tune_step;
#[doc = "TUNE_STEPS_STD (rw) register accessor: an alias for `Reg<TUNE_STEPS_STD_SPEC>`"]
pub type TUNE_STEPS_STD = crate::Reg<tune_steps_std::TUNE_STEPS_STD_SPEC>;
#[doc = "Sample clock delay step count for SDR"]
pub mod tune_steps_std;
#[doc = "TUNE_STEPS_DDR (rw) register accessor: an alias for `Reg<TUNE_STEPS_DDR_SPEC>`"]
pub type TUNE_STEPS_DDR = crate::Reg<tune_steps_ddr::TUNE_STEPS_DDR_SPEC>;
#[doc = "Sample clock delay step count for DDR"]
pub mod tune_steps_ddr;
#[doc = "SPI_INT_SPT (rw) register accessor: an alias for `Reg<SPI_INT_SPT_SPEC>`"]
pub type SPI_INT_SPT = crate::Reg<spi_int_spt::SPI_INT_SPT_SPEC>;
#[doc = "Interrupts in SPI mode depend on CS"]
pub mod spi_int_spt;
#[doc = "SLOTISR_VER (rw) register accessor: an alias for `Reg<SLOTISR_VER_SPEC>`"]
pub type SLOTISR_VER = crate::Reg<slotisr_ver::SLOTISR_VER_SPEC>;
#[doc = "Version information and slot interrupt status"]
pub mod slotisr_ver;
