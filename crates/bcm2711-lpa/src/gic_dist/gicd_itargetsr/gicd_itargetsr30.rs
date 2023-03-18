#[doc = "Register `GICD_ITARGETSR30` reader"]
pub struct R(crate::R<GICD_ITARGETSR30_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ITARGETSR30_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ITARGETSR30_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ITARGETSR30_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ITARGETSR30` writer"]
pub struct W(crate::W<GICD_ITARGETSR30_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ITARGETSR30_SPEC>;
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
impl From<crate::W<GICD_ITARGETSR30_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ITARGETSR30_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_9_10` reader - OR of DMA 9 and 10"]
pub type DMA_9_10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA_9_10` writer - OR of DMA 9 and 10"]
pub type DMA_9_10_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR30_SPEC, u8, u8, 8, O>;
#[doc = "Field `DMA_11` reader - DMA 11"]
pub type DMA_11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA_11` writer - DMA 11"]
pub type DMA_11_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR30_SPEC, u8, u8, 8, O>;
#[doc = "Field `DMA_12` reader - DMA 12"]
pub type DMA_12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA_12` writer - DMA 12"]
pub type DMA_12_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR30_SPEC, u8, u8, 8, O>;
#[doc = "Field `DMA_13` reader - DMA 13"]
pub type DMA_13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA_13` writer - DMA 13"]
pub type DMA_13_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR30_SPEC, u8, u8, 8, O>;
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
impl W {
    #[doc = "Bits 0:7 - OR of DMA 9 and 10"]
    #[inline(always)]
    #[must_use]
    pub fn dma_9_10(&mut self) -> DMA_9_10_W<0> {
        DMA_9_10_W::new(self)
    }
    #[doc = "Bits 8:15 - DMA 11"]
    #[inline(always)]
    #[must_use]
    pub fn dma_11(&mut self) -> DMA_11_W<8> {
        DMA_11_W::new(self)
    }
    #[doc = "Bits 16:23 - DMA 12"]
    #[inline(always)]
    #[must_use]
    pub fn dma_12(&mut self) -> DMA_12_W<16> {
        DMA_12_W::new(self)
    }
    #[doc = "Bits 24:31 - DMA 13"]
    #[inline(always)]
    #[must_use]
    pub fn dma_13(&mut self) -> DMA_13_W<24> {
        DMA_13_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Processor Target 120 - 123\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr30](index.html) module"]
pub struct GICD_ITARGETSR30_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR30_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_itargetsr30::R](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR30_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr30::W](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR30_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR30 to value 0"]
impl crate::Resettable for GICD_ITARGETSR30_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
