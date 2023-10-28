#[doc = "Register `GICD_ITARGETSR31` reader"]
pub type R = crate::R<GICD_ITARGETSR31_SPEC>;
#[doc = "Register `GICD_ITARGETSR31` writer"]
pub type W = crate::W<GICD_ITARGETSR31_SPEC>;
#[doc = "Field `DMA_14` reader - DMA 14"]
pub type DMA_14_R = crate::FieldReader;
#[doc = "Field `DMA_14` writer - DMA 14"]
pub type DMA_14_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `AUX` reader - OR of UART1, SPI1 and SPI2"]
pub type AUX_R = crate::FieldReader;
#[doc = "Field `AUX` writer - OR of UART1, SPI1 and SPI2"]
pub type AUX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `ARM` reader - ARM"]
pub type ARM_R = crate::FieldReader;
#[doc = "Field `ARM` writer - ARM"]
pub type ARM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `DMA_15` reader - DMA 15"]
pub type DMA_15_R = crate::FieldReader;
#[doc = "Field `DMA_15` writer - DMA 15"]
pub type DMA_15_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR31")
            .field("dma_14", &format_args!("{}", self.dma_14().bits()))
            .field("aux", &format_args!("{}", self.aux().bits()))
            .field("arm", &format_args!("{}", self.arm().bits()))
            .field("dma_15", &format_args!("{}", self.dma_15().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_ITARGETSR31_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - DMA 14"]
    #[inline(always)]
    #[must_use]
    pub fn dma_14(&mut self) -> DMA_14_W<GICD_ITARGETSR31_SPEC, 0> {
        DMA_14_W::new(self)
    }
    #[doc = "Bits 8:15 - OR of UART1, SPI1 and SPI2"]
    #[inline(always)]
    #[must_use]
    pub fn aux(&mut self) -> AUX_W<GICD_ITARGETSR31_SPEC, 8> {
        AUX_W::new(self)
    }
    #[doc = "Bits 16:23 - ARM"]
    #[inline(always)]
    #[must_use]
    pub fn arm(&mut self) -> ARM_W<GICD_ITARGETSR31_SPEC, 16> {
        ARM_W::new(self)
    }
    #[doc = "Bits 24:31 - DMA 15"]
    #[inline(always)]
    #[must_use]
    pub fn dma_15(&mut self) -> DMA_15_W<GICD_ITARGETSR31_SPEC, 24> {
        DMA_15_W::new(self)
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
#[doc = "Interrupt Processor Target 124 - 127\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr31::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr31::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ITARGETSR31_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR31_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr31::R`](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR31_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr31::W`](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR31_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR31 to value 0"]
impl crate::Resettable for GICD_ITARGETSR31_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
