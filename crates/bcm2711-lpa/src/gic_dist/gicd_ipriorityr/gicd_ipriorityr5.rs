#[doc = "Register `GICD_IPRIORITYR5` reader"]
pub struct R(crate::R<GICD_IPRIORITYR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_IPRIORITYR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_IPRIORITYR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_IPRIORITYR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_IPRIORITYR5` writer"]
pub struct W(crate::W<GICD_IPRIORITYR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_IPRIORITYR5_SPEC>;
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
impl From<crate::W<GICD_IPRIORITYR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_IPRIORITYR5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT20` reader - Interrupt 20"]
pub type INT20_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT20` writer - Interrupt 20"]
pub type INT20_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR5_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT21` reader - Interrupt 21"]
pub type INT21_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT21` writer - Interrupt 21"]
pub type INT21_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR5_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT22` reader - Interrupt 22"]
pub type INT22_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT22` writer - Interrupt 22"]
pub type INT22_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR5_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT23` reader - Interrupt 23"]
pub type INT23_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT23` writer - Interrupt 23"]
pub type INT23_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR5_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 20"]
    #[inline(always)]
    pub fn int20(&self) -> INT20_R {
        INT20_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 21"]
    #[inline(always)]
    pub fn int21(&self) -> INT21_R {
        INT21_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 22"]
    #[inline(always)]
    pub fn int22(&self) -> INT22_R {
        INT22_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 23"]
    #[inline(always)]
    pub fn int23(&self) -> INT23_R {
        INT23_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 20"]
    #[inline(always)]
    #[must_use]
    pub fn int20(&mut self) -> INT20_W<0> {
        INT20_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 21"]
    #[inline(always)]
    #[must_use]
    pub fn int21(&mut self) -> INT21_W<8> {
        INT21_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 22"]
    #[inline(always)]
    #[must_use]
    pub fn int22(&mut self) -> INT22_W<16> {
        INT22_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 23"]
    #[inline(always)]
    #[must_use]
    pub fn int23(&mut self) -> INT23_W<24> {
        INT23_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority 20 - 23 (Lower is first)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr5](index.html) module"]
pub struct GICD_IPRIORITYR5_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_ipriorityr5::R](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr5::W](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR5 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
