#[doc = "Register `DMAC` reader"]
pub struct R(crate::R<DMAC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAC` writer"]
pub struct W(crate::W<DMAC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAC_SPEC>;
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
impl From<crate::W<DMAC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DREQ` reader - DMA threshold for DREQ signal"]
pub type DREQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DREQ` writer - DMA threshold for DREQ signal"]
pub type DREQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMAC_SPEC, u8, u8, 8, O>;
#[doc = "Field `PANIC` reader - DMA threshold for panic signal"]
pub type PANIC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PANIC` writer - DMA threshold for panic signal"]
pub type PANIC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMAC_SPEC, u8, u8, 8, O>;
#[doc = "Field `ENAB` reader - DMA enabled"]
pub type ENAB_R = crate::BitReader<bool>;
#[doc = "Field `ENAB` writer - DMA enabled"]
pub type ENAB_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAC_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - DMA threshold for DREQ signal"]
    #[inline(always)]
    pub fn dreq(&self) -> DREQ_R {
        DREQ_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DMA threshold for panic signal"]
    #[inline(always)]
    pub fn panic(&self) -> PANIC_R {
        PANIC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - DMA enabled"]
    #[inline(always)]
    pub fn enab(&self) -> ENAB_R {
        ENAB_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - DMA threshold for DREQ signal"]
    #[inline(always)]
    #[must_use]
    pub fn dreq(&mut self) -> DREQ_W<0> {
        DREQ_W::new(self)
    }
    #[doc = "Bits 8:15 - DMA threshold for panic signal"]
    #[inline(always)]
    #[must_use]
    pub fn panic(&mut self) -> PANIC_W<8> {
        PANIC_W::new(self)
    }
    #[doc = "Bit 31 - DMA enabled"]
    #[inline(always)]
    #[must_use]
    pub fn enab(&mut self) -> ENAB_W<31> {
        ENAB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac](index.html) module"]
pub struct DMAC_SPEC;
impl crate::RegisterSpec for DMAC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac::R](R) reader structure"]
impl crate::Readable for DMAC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmac::W](W) writer structure"]
impl crate::Writable for DMAC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAC to value 0"]
impl crate::Resettable for DMAC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
