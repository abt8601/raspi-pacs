#[doc = "Register `GICD_IPRIORITYR40` reader"]
pub struct R(crate::R<GICD_IPRIORITYR40_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_IPRIORITYR40_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_IPRIORITYR40_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_IPRIORITYR40_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_IPRIORITYR40` writer"]
pub struct W(crate::W<GICD_IPRIORITYR40_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_IPRIORITYR40_SPEC>;
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
impl From<crate::W<GICD_IPRIORITYR40_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_IPRIORITYR40_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT160` reader - Interrupt 160"]
pub type INT160_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT160` writer - Interrupt 160"]
pub type INT160_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR40_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT161` reader - Interrupt 161"]
pub type INT161_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT161` writer - Interrupt 161"]
pub type INT161_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR40_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT162` reader - Interrupt 162"]
pub type INT162_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT162` writer - Interrupt 162"]
pub type INT162_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR40_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT163` reader - Interrupt 163"]
pub type INT163_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT163` writer - Interrupt 163"]
pub type INT163_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR40_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 160"]
    #[inline(always)]
    pub fn int160(&self) -> INT160_R {
        INT160_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 161"]
    #[inline(always)]
    pub fn int161(&self) -> INT161_R {
        INT161_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 162"]
    #[inline(always)]
    pub fn int162(&self) -> INT162_R {
        INT162_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 163"]
    #[inline(always)]
    pub fn int163(&self) -> INT163_R {
        INT163_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 160"]
    #[inline(always)]
    #[must_use]
    pub fn int160(&mut self) -> INT160_W<0> {
        INT160_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 161"]
    #[inline(always)]
    #[must_use]
    pub fn int161(&mut self) -> INT161_W<8> {
        INT161_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 162"]
    #[inline(always)]
    #[must_use]
    pub fn int162(&mut self) -> INT162_W<16> {
        INT162_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 163"]
    #[inline(always)]
    #[must_use]
    pub fn int163(&mut self) -> INT163_W<24> {
        INT163_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority 160 - 163 (Lower is first)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr40](index.html) module"]
pub struct GICD_IPRIORITYR40_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR40_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_ipriorityr40::R](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR40_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr40::W](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR40_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR40 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR40_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
