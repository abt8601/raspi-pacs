#[doc = "Register `GICD_IPRIORITYR8` reader"]
pub struct R(crate::R<GICD_IPRIORITYR8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_IPRIORITYR8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_IPRIORITYR8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_IPRIORITYR8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_IPRIORITYR8` writer"]
pub struct W(crate::W<GICD_IPRIORITYR8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_IPRIORITYR8_SPEC>;
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
impl From<crate::W<GICD_IPRIORITYR8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_IPRIORITYR8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT32` reader - Interrupt 32"]
pub type INT32_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT32` writer - Interrupt 32"]
pub type INT32_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR8_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT33` reader - Interrupt 33"]
pub type INT33_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT33` writer - Interrupt 33"]
pub type INT33_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR8_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT34` reader - Interrupt 34"]
pub type INT34_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT34` writer - Interrupt 34"]
pub type INT34_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR8_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT35` reader - Interrupt 35"]
pub type INT35_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT35` writer - Interrupt 35"]
pub type INT35_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR8_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 32"]
    #[inline(always)]
    pub fn int32(&self) -> INT32_R {
        INT32_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 33"]
    #[inline(always)]
    pub fn int33(&self) -> INT33_R {
        INT33_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 34"]
    #[inline(always)]
    pub fn int34(&self) -> INT34_R {
        INT34_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 35"]
    #[inline(always)]
    pub fn int35(&self) -> INT35_R {
        INT35_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 32"]
    #[inline(always)]
    #[must_use]
    pub fn int32(&mut self) -> INT32_W<0> {
        INT32_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 33"]
    #[inline(always)]
    #[must_use]
    pub fn int33(&mut self) -> INT33_W<8> {
        INT33_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 34"]
    #[inline(always)]
    #[must_use]
    pub fn int34(&mut self) -> INT34_W<16> {
        INT34_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 35"]
    #[inline(always)]
    #[must_use]
    pub fn int35(&mut self) -> INT35_W<24> {
        INT35_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority 32 - 35 (Lower is first)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr8](index.html) module"]
pub struct GICD_IPRIORITYR8_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_ipriorityr8::R](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr8::W](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR8_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR8 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
