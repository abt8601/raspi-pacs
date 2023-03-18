#[doc = "Register `GICD_IPRIORITYR14` reader"]
pub struct R(crate::R<GICD_IPRIORITYR14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_IPRIORITYR14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_IPRIORITYR14_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_IPRIORITYR14_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_IPRIORITYR14` writer"]
pub struct W(crate::W<GICD_IPRIORITYR14_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_IPRIORITYR14_SPEC>;
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
impl From<crate::W<GICD_IPRIORITYR14_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_IPRIORITYR14_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT56` reader - Interrupt 56"]
pub type INT56_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT56` writer - Interrupt 56"]
pub type INT56_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR14_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT57` reader - Interrupt 57"]
pub type INT57_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT57` writer - Interrupt 57"]
pub type INT57_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR14_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT58` reader - Interrupt 58"]
pub type INT58_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT58` writer - Interrupt 58"]
pub type INT58_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR14_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT59` reader - Interrupt 59"]
pub type INT59_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT59` writer - Interrupt 59"]
pub type INT59_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR14_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 56"]
    #[inline(always)]
    pub fn int56(&self) -> INT56_R {
        INT56_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 57"]
    #[inline(always)]
    pub fn int57(&self) -> INT57_R {
        INT57_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 58"]
    #[inline(always)]
    pub fn int58(&self) -> INT58_R {
        INT58_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 59"]
    #[inline(always)]
    pub fn int59(&self) -> INT59_R {
        INT59_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 56"]
    #[inline(always)]
    #[must_use]
    pub fn int56(&mut self) -> INT56_W<0> {
        INT56_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 57"]
    #[inline(always)]
    #[must_use]
    pub fn int57(&mut self) -> INT57_W<8> {
        INT57_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 58"]
    #[inline(always)]
    #[must_use]
    pub fn int58(&mut self) -> INT58_W<16> {
        INT58_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 59"]
    #[inline(always)]
    #[must_use]
    pub fn int59(&mut self) -> INT59_W<24> {
        INT59_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority 56 - 59 (Lower is first)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr14](index.html) module"]
pub struct GICD_IPRIORITYR14_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_ipriorityr14::R](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR14_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr14::W](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR14_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR14 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR14_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
