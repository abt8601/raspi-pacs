#[doc = "Register `GICD_IPRIORITYR13` reader"]
pub struct R(crate::R<GICD_IPRIORITYR13_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_IPRIORITYR13_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_IPRIORITYR13_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_IPRIORITYR13_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_IPRIORITYR13` writer"]
pub struct W(crate::W<GICD_IPRIORITYR13_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_IPRIORITYR13_SPEC>;
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
impl From<crate::W<GICD_IPRIORITYR13_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_IPRIORITYR13_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT52` reader - Interrupt 52"]
pub type INT52_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT52` writer - Interrupt 52"]
pub type INT52_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR13_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT53` reader - Interrupt 53"]
pub type INT53_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT53` writer - Interrupt 53"]
pub type INT53_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR13_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT54` reader - Interrupt 54"]
pub type INT54_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT54` writer - Interrupt 54"]
pub type INT54_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR13_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT55` reader - Interrupt 55"]
pub type INT55_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT55` writer - Interrupt 55"]
pub type INT55_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR13_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 52"]
    #[inline(always)]
    pub fn int52(&self) -> INT52_R {
        INT52_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 53"]
    #[inline(always)]
    pub fn int53(&self) -> INT53_R {
        INT53_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 54"]
    #[inline(always)]
    pub fn int54(&self) -> INT54_R {
        INT54_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 55"]
    #[inline(always)]
    pub fn int55(&self) -> INT55_R {
        INT55_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 52"]
    #[inline(always)]
    #[must_use]
    pub fn int52(&mut self) -> INT52_W<0> {
        INT52_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 53"]
    #[inline(always)]
    #[must_use]
    pub fn int53(&mut self) -> INT53_W<8> {
        INT53_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 54"]
    #[inline(always)]
    #[must_use]
    pub fn int54(&mut self) -> INT54_W<16> {
        INT54_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 55"]
    #[inline(always)]
    #[must_use]
    pub fn int55(&mut self) -> INT55_W<24> {
        INT55_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority 52 - 55 (Lower is first)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr13](index.html) module"]
pub struct GICD_IPRIORITYR13_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_ipriorityr13::R](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR13_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr13::W](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR13_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR13 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR13_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
