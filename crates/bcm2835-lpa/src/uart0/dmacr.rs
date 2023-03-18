#[doc = "Register `DMACR` reader"]
pub struct R(crate::R<DMACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACR` writer"]
pub struct W(crate::W<DMACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACR_SPEC>;
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
impl From<crate::W<DMACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXDMAE` reader - RXDMAE"]
pub type RXDMAE_R = crate::BitReader<bool>;
#[doc = "Field `RXDMAE` writer - RXDMAE"]
pub type RXDMAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMACR_SPEC, bool, O>;
#[doc = "Field `TXDMAE` reader - TXDMAE"]
pub type TXDMAE_R = crate::BitReader<bool>;
#[doc = "Field `TXDMAE` writer - TXDMAE"]
pub type TXDMAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMACR_SPEC, bool, O>;
#[doc = "Field `DMAONERR` reader - DMAONERR"]
pub type DMAONERR_R = crate::BitReader<bool>;
#[doc = "Field `DMAONERR` writer - DMAONERR"]
pub type DMAONERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMACR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - RXDMAE"]
    #[inline(always)]
    pub fn rxdmae(&self) -> RXDMAE_R {
        RXDMAE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXDMAE"]
    #[inline(always)]
    pub fn txdmae(&self) -> TXDMAE_R {
        TXDMAE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMAONERR"]
    #[inline(always)]
    pub fn dmaonerr(&self) -> DMAONERR_R {
        DMAONERR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXDMAE"]
    #[inline(always)]
    #[must_use]
    pub fn rxdmae(&mut self) -> RXDMAE_W<0> {
        RXDMAE_W::new(self)
    }
    #[doc = "Bit 1 - TXDMAE"]
    #[inline(always)]
    #[must_use]
    pub fn txdmae(&mut self) -> TXDMAE_W<1> {
        TXDMAE_W::new(self)
    }
    #[doc = "Bit 2 - DMAONERR"]
    #[inline(always)]
    #[must_use]
    pub fn dmaonerr(&mut self) -> DMAONERR_W<2> {
        DMAONERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacr](index.html) module"]
pub struct DMACR_SPEC;
impl crate::RegisterSpec for DMACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmacr::R](R) reader structure"]
impl crate::Readable for DMACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmacr::W](W) writer structure"]
impl crate::Writable for DMACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMACR to value 0"]
impl crate::Resettable for DMACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
