#[doc = "Register `BASIC_PENDING` reader"]
pub type R = crate::R<BASIC_PENDING_SPEC>;
#[doc = "Field `TIMER` reader - ARMC Timer"]
pub type TIMER_R = crate::BitReader;
#[doc = "Field `MAILBOX` reader - Mailbox"]
pub type MAILBOX_R = crate::BitReader;
#[doc = "Field `DOORBELL0` reader - Doorbell 0"]
pub type DOORBELL0_R = crate::BitReader;
#[doc = "Field `DOORBELL1` reader - Doorbell 1"]
pub type DOORBELL1_R = crate::BitReader;
#[doc = "Field `VPU0_HALTED` reader - VPU0 halted"]
pub type VPU0_HALTED_R = crate::BitReader;
#[doc = "Field `VPU1_HALTED` reader - VPU1 halted"]
pub type VPU1_HALTED_R = crate::BitReader;
#[doc = "Field `ARM_ADDRESS_ERROR` reader - ARM address error"]
pub type ARM_ADDRESS_ERROR_R = crate::BitReader;
#[doc = "Field `ARM_AXI_ERROR` reader - ARM AXI error"]
pub type ARM_AXI_ERROR_R = crate::BitReader;
#[doc = "Field `PENDING_1` reader - One or more bits are set in PENDING_1 (ignores 7, 9, 10, 18, 19)"]
pub type PENDING_1_R = crate::BitReader;
#[doc = "Field `PENDING_2` reader - One or more bits are set in PENDING_2 (ignores 53 - 57, 62)"]
pub type PENDING_2_R = crate::BitReader;
#[doc = "Field `JPEG` reader - JPEG"]
pub type JPEG_R = crate::BitReader;
#[doc = "Field `USB` reader - USB"]
pub type USB_R = crate::BitReader;
#[doc = "Field `V3D` reader - V3D"]
pub type V3D_R = crate::BitReader;
#[doc = "Field `DMA_2` reader - DMA 2"]
pub type DMA_2_R = crate::BitReader;
#[doc = "Field `DMA_3` reader - DMA 3"]
pub type DMA_3_R = crate::BitReader;
#[doc = "Field `I2C` reader - OR of all I2C"]
pub type I2C_R = crate::BitReader;
#[doc = "Field `SPI` reader - OR of all SPI"]
pub type SPI_R = crate::BitReader;
#[doc = "Field `PCM_I2S` reader - PCM/I2S"]
pub type PCM_I2S_R = crate::BitReader;
#[doc = "Field `SDHOST` reader - SDHOST"]
pub type SDHOST_R = crate::BitReader;
#[doc = "Field `UART` reader - OR of all PL011 UARTs"]
pub type UART_R = crate::BitReader;
#[doc = "Field `EMMC` reader - OR of EMMC and EMMC2"]
pub type EMMC_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - ARMC Timer"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mailbox"]
    #[inline(always)]
    pub fn mailbox(&self) -> MAILBOX_R {
        MAILBOX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Doorbell 0"]
    #[inline(always)]
    pub fn doorbell0(&self) -> DOORBELL0_R {
        DOORBELL0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Doorbell 1"]
    #[inline(always)]
    pub fn doorbell1(&self) -> DOORBELL1_R {
        DOORBELL1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VPU0 halted"]
    #[inline(always)]
    pub fn vpu0_halted(&self) -> VPU0_HALTED_R {
        VPU0_HALTED_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - VPU1 halted"]
    #[inline(always)]
    pub fn vpu1_halted(&self) -> VPU1_HALTED_R {
        VPU1_HALTED_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ARM address error"]
    #[inline(always)]
    pub fn arm_address_error(&self) -> ARM_ADDRESS_ERROR_R {
        ARM_ADDRESS_ERROR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ARM AXI error"]
    #[inline(always)]
    pub fn arm_axi_error(&self) -> ARM_AXI_ERROR_R {
        ARM_AXI_ERROR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - One or more bits are set in PENDING_1 (ignores 7, 9, 10, 18, 19)"]
    #[inline(always)]
    pub fn pending_1(&self) -> PENDING_1_R {
        PENDING_1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - One or more bits are set in PENDING_2 (ignores 53 - 57, 62)"]
    #[inline(always)]
    pub fn pending_2(&self) -> PENDING_2_R {
        PENDING_2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - JPEG"]
    #[inline(always)]
    pub fn jpeg(&self) -> JPEG_R {
        JPEG_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - USB"]
    #[inline(always)]
    pub fn usb(&self) -> USB_R {
        USB_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - V3D"]
    #[inline(always)]
    pub fn v3d(&self) -> V3D_R {
        V3D_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DMA 2"]
    #[inline(always)]
    pub fn dma_2(&self) -> DMA_2_R {
        DMA_2_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DMA 3"]
    #[inline(always)]
    pub fn dma_3(&self) -> DMA_3_R {
        DMA_3_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - OR of all I2C"]
    #[inline(always)]
    pub fn i2c(&self) -> I2C_R {
        I2C_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - OR of all SPI"]
    #[inline(always)]
    pub fn spi(&self) -> SPI_R {
        SPI_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PCM/I2S"]
    #[inline(always)]
    pub fn pcm_i2s(&self) -> PCM_I2S_R {
        PCM_I2S_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SDHOST"]
    #[inline(always)]
    pub fn sdhost(&self) -> SDHOST_R {
        SDHOST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - OR of all PL011 UARTs"]
    #[inline(always)]
    pub fn uart(&self) -> UART_R {
        UART_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - OR of EMMC and EMMC2"]
    #[inline(always)]
    pub fn emmc(&self) -> EMMC_R {
        EMMC_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BASIC_PENDING")
            .field("timer", &format_args!("{}", self.timer().bit()))
            .field("mailbox", &format_args!("{}", self.mailbox().bit()))
            .field("doorbell0", &format_args!("{}", self.doorbell0().bit()))
            .field("doorbell1", &format_args!("{}", self.doorbell1().bit()))
            .field("vpu0_halted", &format_args!("{}", self.vpu0_halted().bit()))
            .field("vpu1_halted", &format_args!("{}", self.vpu1_halted().bit()))
            .field(
                "arm_address_error",
                &format_args!("{}", self.arm_address_error().bit()),
            )
            .field(
                "arm_axi_error",
                &format_args!("{}", self.arm_axi_error().bit()),
            )
            .field("pending_1", &format_args!("{}", self.pending_1().bit()))
            .field("pending_2", &format_args!("{}", self.pending_2().bit()))
            .field("jpeg", &format_args!("{}", self.jpeg().bit()))
            .field("usb", &format_args!("{}", self.usb().bit()))
            .field("v3d", &format_args!("{}", self.v3d().bit()))
            .field("dma_2", &format_args!("{}", self.dma_2().bit()))
            .field("dma_3", &format_args!("{}", self.dma_3().bit()))
            .field("i2c", &format_args!("{}", self.i2c().bit()))
            .field("spi", &format_args!("{}", self.spi().bit()))
            .field("pcm_i2s", &format_args!("{}", self.pcm_i2s().bit()))
            .field("sdhost", &format_args!("{}", self.sdhost().bit()))
            .field("uart", &format_args!("{}", self.uart().bit()))
            .field("emmc", &format_args!("{}", self.emmc().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<BASIC_PENDING_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Basic pending info\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`basic_pending::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BASIC_PENDING_SPEC;
impl crate::RegisterSpec for BASIC_PENDING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`basic_pending::R`](R) reader structure"]
impl crate::Readable for BASIC_PENDING_SPEC {}
#[doc = "`reset()` method sets BASIC_PENDING to value 0"]
impl crate::Resettable for BASIC_PENDING_SPEC {
    const RESET_VALUE: u32 = 0;
}
