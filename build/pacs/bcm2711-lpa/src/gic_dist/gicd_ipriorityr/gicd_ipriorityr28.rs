#[doc = "Register `GICD_IPRIORITYR28` reader"]
pub type R = crate::R<GICD_IPRIORITYR28_SPEC>;
#[doc = "Register `GICD_IPRIORITYR28` writer"]
pub type W = crate::W<GICD_IPRIORITYR28_SPEC>;
#[doc = "Field `DMA_0` reader - DMA 0"]
pub type DMA_0_R = crate::FieldReader;
#[doc = "Field `DMA_0` writer - DMA 0"]
pub type DMA_0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DMA_1` reader - DMA 1"]
pub type DMA_1_R = crate::FieldReader;
#[doc = "Field `DMA_1` writer - DMA 1"]
pub type DMA_1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DMA_2` reader - DMA 2"]
pub type DMA_2_R = crate::FieldReader;
#[doc = "Field `DMA_2` writer - DMA 2"]
pub type DMA_2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DMA_3` reader - DMA 3"]
pub type DMA_3_R = crate::FieldReader;
#[doc = "Field `DMA_3` writer - DMA 3"]
pub type DMA_3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DMA 0"]
    #[inline(always)]
    pub fn dma_0(&self) -> DMA_0_R {
        DMA_0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DMA 1"]
    #[inline(always)]
    pub fn dma_1(&self) -> DMA_1_R {
        DMA_1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DMA 2"]
    #[inline(always)]
    pub fn dma_2(&self) -> DMA_2_R {
        DMA_2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DMA 3"]
    #[inline(always)]
    pub fn dma_3(&self) -> DMA_3_R {
        DMA_3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR28")
            .field("dma_0", &format_args!("{}", self.dma_0().bits()))
            .field("dma_1", &format_args!("{}", self.dma_1().bits()))
            .field("dma_2", &format_args!("{}", self.dma_2().bits()))
            .field("dma_3", &format_args!("{}", self.dma_3().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_IPRIORITYR28_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - DMA 0"]
    #[inline(always)]
    #[must_use]
    pub fn dma_0(&mut self) -> DMA_0_W<GICD_IPRIORITYR28_SPEC> {
        DMA_0_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - DMA 1"]
    #[inline(always)]
    #[must_use]
    pub fn dma_1(&mut self) -> DMA_1_W<GICD_IPRIORITYR28_SPEC> {
        DMA_1_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - DMA 2"]
    #[inline(always)]
    #[must_use]
    pub fn dma_2(&mut self) -> DMA_2_W<GICD_IPRIORITYR28_SPEC> {
        DMA_2_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - DMA 3"]
    #[inline(always)]
    #[must_use]
    pub fn dma_3(&mut self) -> DMA_3_W<GICD_IPRIORITYR28_SPEC> {
        DMA_3_W::new(self, 24)
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
#[doc = "Interrupt Priority 112 - 115 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr28::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr28::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_IPRIORITYR28_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR28_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr28::R`](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR28_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr28::W`](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR28_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR28 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR28_SPEC {
    const RESET_VALUE: u32 = 0;
}
