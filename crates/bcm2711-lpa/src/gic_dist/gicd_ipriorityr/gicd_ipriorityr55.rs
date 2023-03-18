#[doc = "Register `GICD_IPRIORITYR55` reader"]
pub struct R(crate::R<GICD_IPRIORITYR55_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_IPRIORITYR55_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_IPRIORITYR55_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_IPRIORITYR55_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_IPRIORITYR55` writer"]
pub struct W(crate::W<GICD_IPRIORITYR55_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_IPRIORITYR55_SPEC>;
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
impl From<crate::W<GICD_IPRIORITYR55_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_IPRIORITYR55_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT220` reader - Interrupt 220"]
pub type INT220_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT220` writer - Interrupt 220"]
pub type INT220_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR55_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT221` reader - Interrupt 221"]
pub type INT221_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT221` writer - Interrupt 221"]
pub type INT221_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR55_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT222` reader - Interrupt 222"]
pub type INT222_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT222` writer - Interrupt 222"]
pub type INT222_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR55_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT223` reader - Interrupt 223"]
pub type INT223_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT223` writer - Interrupt 223"]
pub type INT223_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR55_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 220"]
    #[inline(always)]
    pub fn int220(&self) -> INT220_R {
        INT220_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 221"]
    #[inline(always)]
    pub fn int221(&self) -> INT221_R {
        INT221_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 222"]
    #[inline(always)]
    pub fn int222(&self) -> INT222_R {
        INT222_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 223"]
    #[inline(always)]
    pub fn int223(&self) -> INT223_R {
        INT223_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 220"]
    #[inline(always)]
    #[must_use]
    pub fn int220(&mut self) -> INT220_W<0> {
        INT220_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 221"]
    #[inline(always)]
    #[must_use]
    pub fn int221(&mut self) -> INT221_W<8> {
        INT221_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 222"]
    #[inline(always)]
    #[must_use]
    pub fn int222(&mut self) -> INT222_W<16> {
        INT222_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 223"]
    #[inline(always)]
    #[must_use]
    pub fn int223(&mut self) -> INT223_W<24> {
        INT223_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority 220 - 223 (Lower is first)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr55](index.html) module"]
pub struct GICD_IPRIORITYR55_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR55_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_ipriorityr55::R](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR55_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr55::W](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR55_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR55 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR55_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
