#[doc = "Register `GICD_ITARGETSR29` reader"]
pub struct R(crate::R<GICD_ITARGETSR29_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ITARGETSR29_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ITARGETSR29_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ITARGETSR29_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ITARGETSR29` writer"]
pub struct W(crate::W<GICD_ITARGETSR29_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ITARGETSR29_SPEC>;
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
impl From<crate::W<GICD_ITARGETSR29_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ITARGETSR29_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_4` reader - DMA 4"]
pub type DMA_4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA_4` writer - DMA 4"]
pub type DMA_4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR29_SPEC, u8, u8, 8, O>;
#[doc = "Field `DMA_5` reader - DMA 5"]
pub type DMA_5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA_5` writer - DMA 5"]
pub type DMA_5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR29_SPEC, u8, u8, 8, O>;
#[doc = "Field `DMA_6` reader - DMA 6"]
pub type DMA_6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA_6` writer - DMA 6"]
pub type DMA_6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR29_SPEC, u8, u8, 8, O>;
#[doc = "Field `DMA_7_8` reader - OR of DMA 7 and 8"]
pub type DMA_7_8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA_7_8` writer - OR of DMA 7 and 8"]
pub type DMA_7_8_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR29_SPEC, u8, u8, 8, O>;
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
impl W {
    #[doc = "Bits 0:7 - DMA 4"]
    #[inline(always)]
    #[must_use]
    pub fn dma_4(&mut self) -> DMA_4_W<0> {
        DMA_4_W::new(self)
    }
    #[doc = "Bits 8:15 - DMA 5"]
    #[inline(always)]
    #[must_use]
    pub fn dma_5(&mut self) -> DMA_5_W<8> {
        DMA_5_W::new(self)
    }
    #[doc = "Bits 16:23 - DMA 6"]
    #[inline(always)]
    #[must_use]
    pub fn dma_6(&mut self) -> DMA_6_W<16> {
        DMA_6_W::new(self)
    }
    #[doc = "Bits 24:31 - OR of DMA 7 and 8"]
    #[inline(always)]
    #[must_use]
    pub fn dma_7_8(&mut self) -> DMA_7_8_W<24> {
        DMA_7_8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Processor Target 116 - 119\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr29](index.html) module"]
pub struct GICD_ITARGETSR29_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR29_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_itargetsr29::R](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR29_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr29::W](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR29_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR29 to value 0"]
impl crate::Resettable for GICD_ITARGETSR29_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
