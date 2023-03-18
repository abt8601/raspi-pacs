#[doc = "Register `GICD_IPRIORITYR28` reader"]
pub struct R(crate::R<GICD_IPRIORITYR28_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_IPRIORITYR28_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_IPRIORITYR28_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_IPRIORITYR28_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_IPRIORITYR28` writer"]
pub struct W(crate::W<GICD_IPRIORITYR28_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_IPRIORITYR28_SPEC>;
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
impl From<crate::W<GICD_IPRIORITYR28_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_IPRIORITYR28_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_0` reader - DMA 0"]
pub type DMA_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA_0` writer - DMA 0"]
pub type DMA_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR28_SPEC, u8, u8, 8, O>;
#[doc = "Field `DMA_1` reader - DMA 1"]
pub type DMA_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA_1` writer - DMA 1"]
pub type DMA_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR28_SPEC, u8, u8, 8, O>;
#[doc = "Field `DMA_2` reader - DMA 2"]
pub type DMA_2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA_2` writer - DMA 2"]
pub type DMA_2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR28_SPEC, u8, u8, 8, O>;
#[doc = "Field `DMA_3` reader - DMA 3"]
pub type DMA_3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA_3` writer - DMA 3"]
pub type DMA_3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR28_SPEC, u8, u8, 8, O>;
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
impl W {
    #[doc = "Bits 0:7 - DMA 0"]
    #[inline(always)]
    #[must_use]
    pub fn dma_0(&mut self) -> DMA_0_W<0> {
        DMA_0_W::new(self)
    }
    #[doc = "Bits 8:15 - DMA 1"]
    #[inline(always)]
    #[must_use]
    pub fn dma_1(&mut self) -> DMA_1_W<8> {
        DMA_1_W::new(self)
    }
    #[doc = "Bits 16:23 - DMA 2"]
    #[inline(always)]
    #[must_use]
    pub fn dma_2(&mut self) -> DMA_2_W<16> {
        DMA_2_W::new(self)
    }
    #[doc = "Bits 24:31 - DMA 3"]
    #[inline(always)]
    #[must_use]
    pub fn dma_3(&mut self) -> DMA_3_W<24> {
        DMA_3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority 112 - 115 (Lower is first)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr28](index.html) module"]
pub struct GICD_IPRIORITYR28_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR28_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_ipriorityr28::R](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR28_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr28::W](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR28_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR28 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR28_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
