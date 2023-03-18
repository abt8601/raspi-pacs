#[doc = "Register `GICD_IPRIORITYR53` reader"]
pub struct R(crate::R<GICD_IPRIORITYR53_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_IPRIORITYR53_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_IPRIORITYR53_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_IPRIORITYR53_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_IPRIORITYR53` writer"]
pub struct W(crate::W<GICD_IPRIORITYR53_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_IPRIORITYR53_SPEC>;
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
impl From<crate::W<GICD_IPRIORITYR53_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_IPRIORITYR53_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT212` reader - Interrupt 212"]
pub type INT212_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT212` writer - Interrupt 212"]
pub type INT212_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR53_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT213` reader - Interrupt 213"]
pub type INT213_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT213` writer - Interrupt 213"]
pub type INT213_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR53_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT214` reader - Interrupt 214"]
pub type INT214_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT214` writer - Interrupt 214"]
pub type INT214_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR53_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT215` reader - Interrupt 215"]
pub type INT215_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT215` writer - Interrupt 215"]
pub type INT215_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR53_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 212"]
    #[inline(always)]
    pub fn int212(&self) -> INT212_R {
        INT212_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 213"]
    #[inline(always)]
    pub fn int213(&self) -> INT213_R {
        INT213_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 214"]
    #[inline(always)]
    pub fn int214(&self) -> INT214_R {
        INT214_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 215"]
    #[inline(always)]
    pub fn int215(&self) -> INT215_R {
        INT215_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 212"]
    #[inline(always)]
    #[must_use]
    pub fn int212(&mut self) -> INT212_W<0> {
        INT212_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 213"]
    #[inline(always)]
    #[must_use]
    pub fn int213(&mut self) -> INT213_W<8> {
        INT213_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 214"]
    #[inline(always)]
    #[must_use]
    pub fn int214(&mut self) -> INT214_W<16> {
        INT214_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 215"]
    #[inline(always)]
    #[must_use]
    pub fn int215(&mut self) -> INT215_W<24> {
        INT215_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority 212 - 215 (Lower is first)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr53](index.html) module"]
pub struct GICD_IPRIORITYR53_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR53_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_ipriorityr53::R](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR53_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr53::W](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR53_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR53 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR53_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
