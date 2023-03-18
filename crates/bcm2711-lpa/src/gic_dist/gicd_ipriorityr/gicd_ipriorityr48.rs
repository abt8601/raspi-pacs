#[doc = "Register `GICD_IPRIORITYR48` reader"]
pub struct R(crate::R<GICD_IPRIORITYR48_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_IPRIORITYR48_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_IPRIORITYR48_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_IPRIORITYR48_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_IPRIORITYR48` writer"]
pub struct W(crate::W<GICD_IPRIORITYR48_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_IPRIORITYR48_SPEC>;
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
impl From<crate::W<GICD_IPRIORITYR48_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_IPRIORITYR48_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT192` reader - Interrupt 192"]
pub type INT192_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT192` writer - Interrupt 192"]
pub type INT192_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR48_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT193` reader - Interrupt 193"]
pub type INT193_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT193` writer - Interrupt 193"]
pub type INT193_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR48_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT194` reader - Interrupt 194"]
pub type INT194_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT194` writer - Interrupt 194"]
pub type INT194_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR48_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT195` reader - Interrupt 195"]
pub type INT195_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT195` writer - Interrupt 195"]
pub type INT195_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR48_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 192"]
    #[inline(always)]
    pub fn int192(&self) -> INT192_R {
        INT192_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 193"]
    #[inline(always)]
    pub fn int193(&self) -> INT193_R {
        INT193_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 194"]
    #[inline(always)]
    pub fn int194(&self) -> INT194_R {
        INT194_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 195"]
    #[inline(always)]
    pub fn int195(&self) -> INT195_R {
        INT195_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 192"]
    #[inline(always)]
    #[must_use]
    pub fn int192(&mut self) -> INT192_W<0> {
        INT192_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 193"]
    #[inline(always)]
    #[must_use]
    pub fn int193(&mut self) -> INT193_W<8> {
        INT193_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 194"]
    #[inline(always)]
    #[must_use]
    pub fn int194(&mut self) -> INT194_W<16> {
        INT194_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 195"]
    #[inline(always)]
    #[must_use]
    pub fn int195(&mut self) -> INT195_W<24> {
        INT195_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority 192 - 195 (Lower is first)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr48](index.html) module"]
pub struct GICD_IPRIORITYR48_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR48_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_ipriorityr48::R](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR48_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr48::W](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR48_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR48 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR48_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
