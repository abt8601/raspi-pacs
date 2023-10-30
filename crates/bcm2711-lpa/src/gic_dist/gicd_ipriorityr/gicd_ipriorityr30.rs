#[doc = "Register `GICD_IPRIORITYR30` reader"]
pub type R = crate::R<GICD_IPRIORITYR30_SPEC>;
#[doc = "Register `GICD_IPRIORITYR30` writer"]
pub type W = crate::W<GICD_IPRIORITYR30_SPEC>;
#[doc = "Field `DMA_9_10` reader - OR of DMA 9 and 10"]
pub type DMA_9_10_R = crate::FieldReader;
#[doc = "Field `DMA_9_10` writer - OR of DMA 9 and 10"]
pub type DMA_9_10_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `DMA_11` reader - DMA 11"]
pub type DMA_11_R = crate::FieldReader;
#[doc = "Field `DMA_11` writer - DMA 11"]
pub type DMA_11_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `DMA_12` reader - DMA 12"]
pub type DMA_12_R = crate::FieldReader;
#[doc = "Field `DMA_12` writer - DMA 12"]
pub type DMA_12_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `DMA_13` reader - DMA 13"]
pub type DMA_13_R = crate::FieldReader;
#[doc = "Field `DMA_13` writer - DMA 13"]
pub type DMA_13_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - OR of DMA 9 and 10"]
    #[inline(always)]
    pub fn dma_9_10(&self) -> DMA_9_10_R {
        DMA_9_10_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DMA 11"]
    #[inline(always)]
    pub fn dma_11(&self) -> DMA_11_R {
        DMA_11_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DMA 12"]
    #[inline(always)]
    pub fn dma_12(&self) -> DMA_12_R {
        DMA_12_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DMA 13"]
    #[inline(always)]
    pub fn dma_13(&self) -> DMA_13_R {
        DMA_13_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR30")
            .field("dma_9_10", &format_args!("{}", self.dma_9_10().bits()))
            .field("dma_11", &format_args!("{}", self.dma_11().bits()))
            .field("dma_12", &format_args!("{}", self.dma_12().bits()))
            .field("dma_13", &format_args!("{}", self.dma_13().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_IPRIORITYR30_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - OR of DMA 9 and 10"]
    #[inline(always)]
    #[must_use]
    pub fn dma_9_10(&mut self) -> DMA_9_10_W<GICD_IPRIORITYR30_SPEC, 0> {
        DMA_9_10_W::new(self)
    }
    #[doc = "Bits 8:15 - DMA 11"]
    #[inline(always)]
    #[must_use]
    pub fn dma_11(&mut self) -> DMA_11_W<GICD_IPRIORITYR30_SPEC, 8> {
        DMA_11_W::new(self)
    }
    #[doc = "Bits 16:23 - DMA 12"]
    #[inline(always)]
    #[must_use]
    pub fn dma_12(&mut self) -> DMA_12_W<GICD_IPRIORITYR30_SPEC, 16> {
        DMA_12_W::new(self)
    }
    #[doc = "Bits 24:31 - DMA 13"]
    #[inline(always)]
    #[must_use]
    pub fn dma_13(&mut self) -> DMA_13_W<GICD_IPRIORITYR30_SPEC, 24> {
        DMA_13_W::new(self)
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
#[doc = "Interrupt Priority 120 - 123 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr30::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr30::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_IPRIORITYR30_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR30_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr30::R`](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR30_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr30::W`](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR30_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR30 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR30_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
