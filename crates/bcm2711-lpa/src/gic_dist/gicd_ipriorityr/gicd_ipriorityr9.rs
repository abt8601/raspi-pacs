#[doc = "Register `GICD_IPRIORITYR9` reader"]
pub struct R(crate::R<GICD_IPRIORITYR9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_IPRIORITYR9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_IPRIORITYR9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_IPRIORITYR9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_IPRIORITYR9` writer"]
pub struct W(crate::W<GICD_IPRIORITYR9_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_IPRIORITYR9_SPEC>;
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
impl From<crate::W<GICD_IPRIORITYR9_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_IPRIORITYR9_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT36` reader - Interrupt 36"]
pub type INT36_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT36` writer - Interrupt 36"]
pub type INT36_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR9_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT37` reader - Interrupt 37"]
pub type INT37_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT37` writer - Interrupt 37"]
pub type INT37_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR9_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT38` reader - Interrupt 38"]
pub type INT38_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT38` writer - Interrupt 38"]
pub type INT38_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR9_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT39` reader - Interrupt 39"]
pub type INT39_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT39` writer - Interrupt 39"]
pub type INT39_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR9_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 36"]
    #[inline(always)]
    pub fn int36(&self) -> INT36_R {
        INT36_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 37"]
    #[inline(always)]
    pub fn int37(&self) -> INT37_R {
        INT37_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 38"]
    #[inline(always)]
    pub fn int38(&self) -> INT38_R {
        INT38_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 39"]
    #[inline(always)]
    pub fn int39(&self) -> INT39_R {
        INT39_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 36"]
    #[inline(always)]
    #[must_use]
    pub fn int36(&mut self) -> INT36_W<0> {
        INT36_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 37"]
    #[inline(always)]
    #[must_use]
    pub fn int37(&mut self) -> INT37_W<8> {
        INT37_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 38"]
    #[inline(always)]
    #[must_use]
    pub fn int38(&mut self) -> INT38_W<16> {
        INT38_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 39"]
    #[inline(always)]
    #[must_use]
    pub fn int39(&mut self) -> INT39_W<24> {
        INT39_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority 36 - 39 (Lower is first)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr9](index.html) module"]
pub struct GICD_IPRIORITYR9_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_ipriorityr9::R](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR9_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr9::W](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR9_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR9 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR9_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
