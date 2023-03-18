#[doc = "Register `GICD_IPRIORITYR3` reader"]
pub struct R(crate::R<GICD_IPRIORITYR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_IPRIORITYR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_IPRIORITYR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_IPRIORITYR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_IPRIORITYR3` writer"]
pub struct W(crate::W<GICD_IPRIORITYR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_IPRIORITYR3_SPEC>;
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
impl From<crate::W<GICD_IPRIORITYR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_IPRIORITYR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT12` reader - Interrupt 12"]
pub type INT12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT12` writer - Interrupt 12"]
pub type INT12_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR3_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT13` reader - Interrupt 13"]
pub type INT13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT13` writer - Interrupt 13"]
pub type INT13_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR3_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT14` reader - Interrupt 14"]
pub type INT14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT14` writer - Interrupt 14"]
pub type INT14_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR3_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT15` reader - Interrupt 15"]
pub type INT15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT15` writer - Interrupt 15"]
pub type INT15_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR3_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 12"]
    #[inline(always)]
    pub fn int12(&self) -> INT12_R {
        INT12_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 13"]
    #[inline(always)]
    pub fn int13(&self) -> INT13_R {
        INT13_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 14"]
    #[inline(always)]
    pub fn int14(&self) -> INT14_R {
        INT14_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 15"]
    #[inline(always)]
    pub fn int15(&self) -> INT15_R {
        INT15_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 12"]
    #[inline(always)]
    #[must_use]
    pub fn int12(&mut self) -> INT12_W<0> {
        INT12_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 13"]
    #[inline(always)]
    #[must_use]
    pub fn int13(&mut self) -> INT13_W<8> {
        INT13_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 14"]
    #[inline(always)]
    #[must_use]
    pub fn int14(&mut self) -> INT14_W<16> {
        INT14_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 15"]
    #[inline(always)]
    #[must_use]
    pub fn int15(&mut self) -> INT15_W<24> {
        INT15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority 12 - 15 (Lower is first)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr3](index.html) module"]
pub struct GICD_IPRIORITYR3_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_ipriorityr3::R](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr3::W](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR3 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
