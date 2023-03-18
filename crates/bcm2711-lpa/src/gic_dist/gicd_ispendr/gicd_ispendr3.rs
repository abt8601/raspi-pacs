#[doc = "Register `GICD_ISPENDR3` reader"]
pub struct R(crate::R<GICD_ISPENDR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ISPENDR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ISPENDR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ISPENDR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ISPENDR3` writer"]
pub struct W(crate::W<GICD_ISPENDR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ISPENDR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<GICD_ISPENDR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ISPENDR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_0` reader - Timer 0"]
pub type TIMER_0_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_0` writer - Timer 0"]
pub type TIMER_0_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISPENDR3_SPEC, bool, O>;
#[doc = "Field `TIMER_1` reader - Timer 1"]
pub type TIMER_1_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_1` writer - Timer 1"]
pub type TIMER_1_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISPENDR3_SPEC, bool, O>;
#[doc = "Field `TIMER_2` reader - Timer 2"]
pub type TIMER_2_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_2` writer - Timer 2"]
pub type TIMER_2_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISPENDR3_SPEC, bool, O>;
#[doc = "Field `TIMER_3` reader - Timer 3"]
pub type TIMER_3_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_3` writer - Timer 3"]
pub type TIMER_3_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISPENDR3_SPEC, bool, O>;
#[doc = "Field `H264_0` reader - H264 0"]
pub type H264_0_R = crate::BitReader<bool>;
#[doc = "Field `H264_0` writer - H264 0"]
pub type H264_0_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISPENDR3_SPEC, bool, O>;
#[doc = "Field `H264_1` reader - H264 1"]
pub type H264_1_R = crate::BitReader<bool>;
#[doc = "Field `H264_1` writer - H264 1"]
pub type H264_1_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISPENDR3_SPEC, bool, O>;
#[doc = "Field `H264_2` reader - H264 2"]
pub type H264_2_R = crate::BitReader<bool>;
#[doc = "Field `H264_2` writer - H264 2"]
pub type H264_2_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISPENDR3_SPEC, bool, O>;
#[doc = "Field `JPEG` reader - JPEG"]
pub type JPEG_R = crate::BitReader<bool>;
#[doc = "Field `JPEG` writer - JPEG"]
pub type JPEG_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISPENDR3_SPEC, bool, O>;
#[doc = "Field `ISP` reader - ISP"]
pub type ISP_R = crate::BitReader<bool>;
#[doc = "Field `ISP` writer - ISP"]
pub type ISP_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISPENDR3_SPEC, bool, O>;
#[doc = "Field `USB` reader - USB"]
pub type USB_R = crate::BitReader<bool>;
#[doc = "Field `USB` writer - USB"]
pub type USB_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISPENDR3_SPEC, bool, O>;
#[doc = "Field `V3D` reader - V3D"]
pub type V3D_R = crate::BitReader<bool>;
#[doc = "Field `V3D` writer - V3D"]
pub type V3D_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISPENDR3_SPEC, bool, O>;
#[doc = "Field `TRANSPOSER` reader - Transposer"]
pub type TRANSPOSER_R = crate::BitReader<bool>;
#[doc = "Field `TRANSPOSER` writer - Transposer"]
pub type TRANSPOSER_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISPENDR3_SPEC, bool, O>;
#[doc = "Field `MULTICORE_SYNC_0` reader - Multicore Sync 0"]
pub type MULTICORE_SYNC_0_R = crate::BitReader<bool>;
#[doc = "Field `MULTICORE_SYNC_0` writer - Multicore Sync 0"]
pub type MULTICORE_SYNC_0_W<'a, const O: u8> =
    crate::BitWriter1S<'a, u32, GICD_ISPENDR3_SPEC, bool, O>;
#[doc = "Field `MULTICORE_SYNC_1` reader - Multicore Sync 1"]
pub type MULTICORE_SYNC_1_R = crate::BitReader<bool>;
#[doc = "Field `MULTICORE_SYNC_1` writer - Multicore Sync 1"]
pub type MULTICORE_SYNC_1_W<'a, const O: u8> =
    crate::BitWriter1S<'a, u32, GICD_ISPENDR3_SPEC, bool, O>;
#[doc = "Field `MULTICORE_SYNC_2` reader - Multicore Sync 2"]
pub type MULTICORE_SYNC_2_R = crate::BitReader<bool>;
#[doc = "Field `MULTICORE_SYNC_2` writer - Multicore Sync 2"]
pub type MULTICORE_SYNC_2_W<'a, const O: u8> =
    crate::BitWriter1S<'a, u32, GICD_ISPENDR3_SPEC, bool, O>;
#[doc = "Field `MULTICORE_SYNC_3` reader - Multicore Sync 3"]
pub type MULTICORE_SYNC_3_R = crate::BitReader<bool>;
#[doc = "Field `MULTICORE_SYNC_3` writer - Multicore Sync 3"]
pub type MULTICORE_SYNC_3_W<'a, const O: u8> =
    crate::BitWriter1S<'a, u32, GICD_ISPENDR3_SPEC, bool, O>;
#[doc = "Field `DMA_0` reader - DMA 0"]
pub type DMA_0_R = crate::BitReader<bool>;
#[doc = "Field `DMA_0` writer - DMA 0"]
pub type DMA_0_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISPENDR3_SPEC, bool, O>;
#[doc = "Field `DMA_1` reader - DMA 1"]
pub type DMA_1_R = crate::BitReader<bool>;
#[doc = "Field `DMA_1` writer - DMA 1"]
pub type DMA_1_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISPENDR3_SPEC, bool, O>;
#[doc = "Field `DMA_2` reader - DMA 2"]
pub type DMA_2_R = crate::BitReader<bool>;
#[doc = "Field `DMA_2` writer - DMA 2"]
pub type DMA_2_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISPENDR3_SPEC, bool, O>;
#[doc = "Field `DMA_3` reader - DMA 3"]
pub type DMA_3_R = crate::BitReader<bool>;
#[doc = "Field `DMA_3` writer - DMA 3"]
pub type DMA_3_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISPENDR3_SPEC, bool, O>;
#[doc = "Field `DMA_4` reader - DMA 4"]
pub type DMA_4_R = crate::BitReader<bool>;
#[doc = "Field `DMA_4` writer - DMA 4"]
pub type DMA_4_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISPENDR3_SPEC, bool, O>;
#[doc = "Field `DMA_5` reader - DMA 5"]
pub type DMA_5_R = crate::BitReader<bool>;
#[doc = "Field `DMA_5` writer - DMA 5"]
pub type DMA_5_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISPENDR3_SPEC, bool, O>;
#[doc = "Field `DMA_6` reader - DMA 6"]
pub type DMA_6_R = crate::BitReader<bool>;
#[doc = "Field `DMA_6` writer - DMA 6"]
pub type DMA_6_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISPENDR3_SPEC, bool, O>;
#[doc = "Field `DMA_7_8` reader - OR of DMA 7 and 8"]
pub type DMA_7_8_R = crate::BitReader<bool>;
#[doc = "Field `DMA_7_8` writer - OR of DMA 7 and 8"]
pub type DMA_7_8_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISPENDR3_SPEC, bool, O>;
#[doc = "Field `DMA_9_10` reader - OR of DMA 9 and 10"]
pub type DMA_9_10_R = crate::BitReader<bool>;
#[doc = "Field `DMA_9_10` writer - OR of DMA 9 and 10"]
pub type DMA_9_10_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISPENDR3_SPEC, bool, O>;
#[doc = "Field `DMA_11` reader - DMA 11"]
pub type DMA_11_R = crate::BitReader<bool>;
#[doc = "Field `DMA_11` writer - DMA 11"]
pub type DMA_11_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISPENDR3_SPEC, bool, O>;
#[doc = "Field `DMA_12` reader - DMA 12"]
pub type DMA_12_R = crate::BitReader<bool>;
#[doc = "Field `DMA_12` writer - DMA 12"]
pub type DMA_12_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISPENDR3_SPEC, bool, O>;
#[doc = "Field `DMA_13` reader - DMA 13"]
pub type DMA_13_R = crate::BitReader<bool>;
#[doc = "Field `DMA_13` writer - DMA 13"]
pub type DMA_13_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISPENDR3_SPEC, bool, O>;
#[doc = "Field `DMA_14` reader - DMA 14"]
pub type DMA_14_R = crate::BitReader<bool>;
#[doc = "Field `DMA_14` writer - DMA 14"]
pub type DMA_14_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISPENDR3_SPEC, bool, O>;
#[doc = "Field `AUX` reader - OR of UART1, SPI1 and SPI2"]
pub type AUX_R = crate::BitReader<bool>;
#[doc = "Field `AUX` writer - OR of UART1, SPI1 and SPI2"]
pub type AUX_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISPENDR3_SPEC, bool, O>;
#[doc = "Field `ARM` reader - ARM"]
pub type ARM_R = crate::BitReader<bool>;
#[doc = "Field `ARM` writer - ARM"]
pub type ARM_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISPENDR3_SPEC, bool, O>;
#[doc = "Field `DMA_15` reader - DMA 15"]
pub type DMA_15_R = crate::BitReader<bool>;
#[doc = "Field `DMA_15` writer - DMA 15"]
pub type DMA_15_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISPENDR3_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Timer 0"]
    #[inline(always)]
    pub fn timer_0(&self) -> TIMER_0_R {
        TIMER_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer 1"]
    #[inline(always)]
    pub fn timer_1(&self) -> TIMER_1_R {
        TIMER_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer 2"]
    #[inline(always)]
    pub fn timer_2(&self) -> TIMER_2_R {
        TIMER_2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer 3"]
    #[inline(always)]
    pub fn timer_3(&self) -> TIMER_3_R {
        TIMER_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - H264 0"]
    #[inline(always)]
    pub fn h264_0(&self) -> H264_0_R {
        H264_0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - H264 1"]
    #[inline(always)]
    pub fn h264_1(&self) -> H264_1_R {
        H264_1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - H264 2"]
    #[inline(always)]
    pub fn h264_2(&self) -> H264_2_R {
        H264_2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - JPEG"]
    #[inline(always)]
    pub fn jpeg(&self) -> JPEG_R {
        JPEG_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ISP"]
    #[inline(always)]
    pub fn isp(&self) -> ISP_R {
        ISP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - USB"]
    #[inline(always)]
    pub fn usb(&self) -> USB_R {
        USB_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - V3D"]
    #[inline(always)]
    pub fn v3d(&self) -> V3D_R {
        V3D_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transposer"]
    #[inline(always)]
    pub fn transposer(&self) -> TRANSPOSER_R {
        TRANSPOSER_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Multicore Sync 0"]
    #[inline(always)]
    pub fn multicore_sync_0(&self) -> MULTICORE_SYNC_0_R {
        MULTICORE_SYNC_0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Multicore Sync 1"]
    #[inline(always)]
    pub fn multicore_sync_1(&self) -> MULTICORE_SYNC_1_R {
        MULTICORE_SYNC_1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Multicore Sync 2"]
    #[inline(always)]
    pub fn multicore_sync_2(&self) -> MULTICORE_SYNC_2_R {
        MULTICORE_SYNC_2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Multicore Sync 3"]
    #[inline(always)]
    pub fn multicore_sync_3(&self) -> MULTICORE_SYNC_3_R {
        MULTICORE_SYNC_3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - DMA 0"]
    #[inline(always)]
    pub fn dma_0(&self) -> DMA_0_R {
        DMA_0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DMA 1"]
    #[inline(always)]
    pub fn dma_1(&self) -> DMA_1_R {
        DMA_1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DMA 2"]
    #[inline(always)]
    pub fn dma_2(&self) -> DMA_2_R {
        DMA_2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DMA 3"]
    #[inline(always)]
    pub fn dma_3(&self) -> DMA_3_R {
        DMA_3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - DMA 4"]
    #[inline(always)]
    pub fn dma_4(&self) -> DMA_4_R {
        DMA_4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DMA 5"]
    #[inline(always)]
    pub fn dma_5(&self) -> DMA_5_R {
        DMA_5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DMA 6"]
    #[inline(always)]
    pub fn dma_6(&self) -> DMA_6_R {
        DMA_6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - OR of DMA 7 and 8"]
    #[inline(always)]
    pub fn dma_7_8(&self) -> DMA_7_8_R {
        DMA_7_8_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - OR of DMA 9 and 10"]
    #[inline(always)]
    pub fn dma_9_10(&self) -> DMA_9_10_R {
        DMA_9_10_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DMA 11"]
    #[inline(always)]
    pub fn dma_11(&self) -> DMA_11_R {
        DMA_11_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DMA 12"]
    #[inline(always)]
    pub fn dma_12(&self) -> DMA_12_R {
        DMA_12_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DMA 13"]
    #[inline(always)]
    pub fn dma_13(&self) -> DMA_13_R {
        DMA_13_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DMA 14"]
    #[inline(always)]
    pub fn dma_14(&self) -> DMA_14_R {
        DMA_14_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - OR of UART1, SPI1 and SPI2"]
    #[inline(always)]
    pub fn aux(&self) -> AUX_R {
        AUX_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - ARM"]
    #[inline(always)]
    pub fn arm(&self) -> ARM_R {
        ARM_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DMA 15"]
    #[inline(always)]
    pub fn dma_15(&self) -> DMA_15_R {
        DMA_15_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer 0"]
    #[inline(always)]
    #[must_use]
    pub fn timer_0(&mut self) -> TIMER_0_W<0> {
        TIMER_0_W::new(self)
    }
    #[doc = "Bit 1 - Timer 1"]
    #[inline(always)]
    #[must_use]
    pub fn timer_1(&mut self) -> TIMER_1_W<1> {
        TIMER_1_W::new(self)
    }
    #[doc = "Bit 2 - Timer 2"]
    #[inline(always)]
    #[must_use]
    pub fn timer_2(&mut self) -> TIMER_2_W<2> {
        TIMER_2_W::new(self)
    }
    #[doc = "Bit 3 - Timer 3"]
    #[inline(always)]
    #[must_use]
    pub fn timer_3(&mut self) -> TIMER_3_W<3> {
        TIMER_3_W::new(self)
    }
    #[doc = "Bit 4 - H264 0"]
    #[inline(always)]
    #[must_use]
    pub fn h264_0(&mut self) -> H264_0_W<4> {
        H264_0_W::new(self)
    }
    #[doc = "Bit 5 - H264 1"]
    #[inline(always)]
    #[must_use]
    pub fn h264_1(&mut self) -> H264_1_W<5> {
        H264_1_W::new(self)
    }
    #[doc = "Bit 6 - H264 2"]
    #[inline(always)]
    #[must_use]
    pub fn h264_2(&mut self) -> H264_2_W<6> {
        H264_2_W::new(self)
    }
    #[doc = "Bit 7 - JPEG"]
    #[inline(always)]
    #[must_use]
    pub fn jpeg(&mut self) -> JPEG_W<7> {
        JPEG_W::new(self)
    }
    #[doc = "Bit 8 - ISP"]
    #[inline(always)]
    #[must_use]
    pub fn isp(&mut self) -> ISP_W<8> {
        ISP_W::new(self)
    }
    #[doc = "Bit 9 - USB"]
    #[inline(always)]
    #[must_use]
    pub fn usb(&mut self) -> USB_W<9> {
        USB_W::new(self)
    }
    #[doc = "Bit 10 - V3D"]
    #[inline(always)]
    #[must_use]
    pub fn v3d(&mut self) -> V3D_W<10> {
        V3D_W::new(self)
    }
    #[doc = "Bit 11 - Transposer"]
    #[inline(always)]
    #[must_use]
    pub fn transposer(&mut self) -> TRANSPOSER_W<11> {
        TRANSPOSER_W::new(self)
    }
    #[doc = "Bit 12 - Multicore Sync 0"]
    #[inline(always)]
    #[must_use]
    pub fn multicore_sync_0(&mut self) -> MULTICORE_SYNC_0_W<12> {
        MULTICORE_SYNC_0_W::new(self)
    }
    #[doc = "Bit 13 - Multicore Sync 1"]
    #[inline(always)]
    #[must_use]
    pub fn multicore_sync_1(&mut self) -> MULTICORE_SYNC_1_W<13> {
        MULTICORE_SYNC_1_W::new(self)
    }
    #[doc = "Bit 14 - Multicore Sync 2"]
    #[inline(always)]
    #[must_use]
    pub fn multicore_sync_2(&mut self) -> MULTICORE_SYNC_2_W<14> {
        MULTICORE_SYNC_2_W::new(self)
    }
    #[doc = "Bit 15 - Multicore Sync 3"]
    #[inline(always)]
    #[must_use]
    pub fn multicore_sync_3(&mut self) -> MULTICORE_SYNC_3_W<15> {
        MULTICORE_SYNC_3_W::new(self)
    }
    #[doc = "Bit 16 - DMA 0"]
    #[inline(always)]
    #[must_use]
    pub fn dma_0(&mut self) -> DMA_0_W<16> {
        DMA_0_W::new(self)
    }
    #[doc = "Bit 17 - DMA 1"]
    #[inline(always)]
    #[must_use]
    pub fn dma_1(&mut self) -> DMA_1_W<17> {
        DMA_1_W::new(self)
    }
    #[doc = "Bit 18 - DMA 2"]
    #[inline(always)]
    #[must_use]
    pub fn dma_2(&mut self) -> DMA_2_W<18> {
        DMA_2_W::new(self)
    }
    #[doc = "Bit 19 - DMA 3"]
    #[inline(always)]
    #[must_use]
    pub fn dma_3(&mut self) -> DMA_3_W<19> {
        DMA_3_W::new(self)
    }
    #[doc = "Bit 20 - DMA 4"]
    #[inline(always)]
    #[must_use]
    pub fn dma_4(&mut self) -> DMA_4_W<20> {
        DMA_4_W::new(self)
    }
    #[doc = "Bit 21 - DMA 5"]
    #[inline(always)]
    #[must_use]
    pub fn dma_5(&mut self) -> DMA_5_W<21> {
        DMA_5_W::new(self)
    }
    #[doc = "Bit 22 - DMA 6"]
    #[inline(always)]
    #[must_use]
    pub fn dma_6(&mut self) -> DMA_6_W<22> {
        DMA_6_W::new(self)
    }
    #[doc = "Bit 23 - OR of DMA 7 and 8"]
    #[inline(always)]
    #[must_use]
    pub fn dma_7_8(&mut self) -> DMA_7_8_W<23> {
        DMA_7_8_W::new(self)
    }
    #[doc = "Bit 24 - OR of DMA 9 and 10"]
    #[inline(always)]
    #[must_use]
    pub fn dma_9_10(&mut self) -> DMA_9_10_W<24> {
        DMA_9_10_W::new(self)
    }
    #[doc = "Bit 25 - DMA 11"]
    #[inline(always)]
    #[must_use]
    pub fn dma_11(&mut self) -> DMA_11_W<25> {
        DMA_11_W::new(self)
    }
    #[doc = "Bit 26 - DMA 12"]
    #[inline(always)]
    #[must_use]
    pub fn dma_12(&mut self) -> DMA_12_W<26> {
        DMA_12_W::new(self)
    }
    #[doc = "Bit 27 - DMA 13"]
    #[inline(always)]
    #[must_use]
    pub fn dma_13(&mut self) -> DMA_13_W<27> {
        DMA_13_W::new(self)
    }
    #[doc = "Bit 28 - DMA 14"]
    #[inline(always)]
    #[must_use]
    pub fn dma_14(&mut self) -> DMA_14_W<28> {
        DMA_14_W::new(self)
    }
    #[doc = "Bit 29 - OR of UART1, SPI1 and SPI2"]
    #[inline(always)]
    #[must_use]
    pub fn aux(&mut self) -> AUX_W<29> {
        AUX_W::new(self)
    }
    #[doc = "Bit 30 - ARM"]
    #[inline(always)]
    #[must_use]
    pub fn arm(&mut self) -> ARM_W<30> {
        ARM_W::new(self)
    }
    #[doc = "Bit 31 - DMA 15"]
    #[inline(always)]
    #[must_use]
    pub fn dma_15(&mut self) -> DMA_15_W<31> {
        DMA_15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Set-Pending\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ispendr3](index.html) module"]
pub struct GICD_ISPENDR3_SPEC;
impl crate::RegisterSpec for GICD_ISPENDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_ispendr3::R](R) reader structure"]
impl crate::Readable for GICD_ISPENDR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_ispendr3::W](W) writer structure"]
impl crate::Writable for GICD_ISPENDR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xffff_ffff;
}
#[doc = "`reset()` method sets GICD_ISPENDR3 to value 0"]
impl crate::Resettable for GICD_ISPENDR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
