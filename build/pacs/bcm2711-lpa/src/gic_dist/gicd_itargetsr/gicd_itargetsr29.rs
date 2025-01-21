#[doc = "Register `GICD_ITARGETSR29` reader"]
pub type R = crate::R<GICD_ITARGETSR29_SPEC>;
#[doc = "Register `GICD_ITARGETSR29` writer"]
pub type W = crate::W<GICD_ITARGETSR29_SPEC>;
#[doc = "Field `DMA_4` reader - DMA 4"]
pub type DMA_4_R = crate::FieldReader;
#[doc = "Field `DMA_4` writer - DMA 4"]
pub type DMA_4_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DMA_5` reader - DMA 5"]
pub type DMA_5_R = crate::FieldReader;
#[doc = "Field `DMA_5` writer - DMA 5"]
pub type DMA_5_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DMA_6` reader - DMA 6"]
pub type DMA_6_R = crate::FieldReader;
#[doc = "Field `DMA_6` writer - DMA 6"]
pub type DMA_6_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DMA_7_8` reader - OR of DMA 7 and 8"]
pub type DMA_7_8_R = crate::FieldReader;
#[doc = "Field `DMA_7_8` writer - OR of DMA 7 and 8"]
pub type DMA_7_8_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DMA 4"]
    #[inline(always)]
    pub fn dma_4(&self) -> DMA_4_R {
        DMA_4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DMA 5"]
    #[inline(always)]
    pub fn dma_5(&self) -> DMA_5_R {
        DMA_5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DMA 6"]
    #[inline(always)]
    pub fn dma_6(&self) -> DMA_6_R {
        DMA_6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - OR of DMA 7 and 8"]
    #[inline(always)]
    pub fn dma_7_8(&self) -> DMA_7_8_R {
        DMA_7_8_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR29")
            .field("dma_4", &format_args!("{}", self.dma_4().bits()))
            .field("dma_5", &format_args!("{}", self.dma_5().bits()))
            .field("dma_6", &format_args!("{}", self.dma_6().bits()))
            .field("dma_7_8", &format_args!("{}", self.dma_7_8().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_ITARGETSR29_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - DMA 4"]
    #[inline(always)]
    #[must_use]
    pub fn dma_4(&mut self) -> DMA_4_W<GICD_ITARGETSR29_SPEC> {
        DMA_4_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - DMA 5"]
    #[inline(always)]
    #[must_use]
    pub fn dma_5(&mut self) -> DMA_5_W<GICD_ITARGETSR29_SPEC> {
        DMA_5_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - DMA 6"]
    #[inline(always)]
    #[must_use]
    pub fn dma_6(&mut self) -> DMA_6_W<GICD_ITARGETSR29_SPEC> {
        DMA_6_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - OR of DMA 7 and 8"]
    #[inline(always)]
    #[must_use]
    pub fn dma_7_8(&mut self) -> DMA_7_8_W<GICD_ITARGETSR29_SPEC> {
        DMA_7_8_W::new(self, 24)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Processor Target 116 - 119\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr29::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr29::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ITARGETSR29_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR29_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr29::R`](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR29_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr29::W`](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR29_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR29 to value 0"]
impl crate::Resettable for GICD_ITARGETSR29_SPEC {
    const RESET_VALUE: u32 = 0;
}
