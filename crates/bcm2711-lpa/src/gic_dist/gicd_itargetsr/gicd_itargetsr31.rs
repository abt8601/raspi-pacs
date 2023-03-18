#[doc = "Register `GICD_ITARGETSR31` reader"]
pub struct R(crate::R<GICD_ITARGETSR31_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ITARGETSR31_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ITARGETSR31_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ITARGETSR31_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ITARGETSR31` writer"]
pub struct W(crate::W<GICD_ITARGETSR31_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ITARGETSR31_SPEC>;
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
impl From<crate::W<GICD_ITARGETSR31_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ITARGETSR31_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_14` reader - DMA 14"]
pub type DMA_14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA_14` writer - DMA 14"]
pub type DMA_14_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR31_SPEC, u8, u8, 8, O>;
#[doc = "Field `AUX` reader - OR of UART1, SPI1 and SPI2"]
pub type AUX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AUX` writer - OR of UART1, SPI1 and SPI2"]
pub type AUX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GICD_ITARGETSR31_SPEC, u8, u8, 8, O>;
#[doc = "Field `ARM` reader - ARM"]
pub type ARM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ARM` writer - ARM"]
pub type ARM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GICD_ITARGETSR31_SPEC, u8, u8, 8, O>;
#[doc = "Field `DMA_15` reader - DMA 15"]
pub type DMA_15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA_15` writer - DMA 15"]
pub type DMA_15_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR31_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - DMA 14"]
    #[inline(always)]
    pub fn dma_14(&self) -> DMA_14_R {
        DMA_14_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - OR of UART1, SPI1 and SPI2"]
    #[inline(always)]
    pub fn aux(&self) -> AUX_R {
        AUX_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - ARM"]
    #[inline(always)]
    pub fn arm(&self) -> ARM_R {
        ARM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DMA 15"]
    #[inline(always)]
    pub fn dma_15(&self) -> DMA_15_R {
        DMA_15_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DMA 14"]
    #[inline(always)]
    #[must_use]
    pub fn dma_14(&mut self) -> DMA_14_W<0> {
        DMA_14_W::new(self)
    }
    #[doc = "Bits 8:15 - OR of UART1, SPI1 and SPI2"]
    #[inline(always)]
    #[must_use]
    pub fn aux(&mut self) -> AUX_W<8> {
        AUX_W::new(self)
    }
    #[doc = "Bits 16:23 - ARM"]
    #[inline(always)]
    #[must_use]
    pub fn arm(&mut self) -> ARM_W<16> {
        ARM_W::new(self)
    }
    #[doc = "Bits 24:31 - DMA 15"]
    #[inline(always)]
    #[must_use]
    pub fn dma_15(&mut self) -> DMA_15_W<24> {
        DMA_15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Processor Target 124 - 127\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr31](index.html) module"]
pub struct GICD_ITARGETSR31_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR31_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_itargetsr31::R](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR31_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr31::W](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR31_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR31 to value 0"]
impl crate::Resettable for GICD_ITARGETSR31_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
