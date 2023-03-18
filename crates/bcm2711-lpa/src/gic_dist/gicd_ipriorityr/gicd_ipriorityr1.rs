#[doc = "Register `GICD_IPRIORITYR1` reader"]
pub struct R(crate::R<GICD_IPRIORITYR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_IPRIORITYR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_IPRIORITYR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_IPRIORITYR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_IPRIORITYR1` writer"]
pub struct W(crate::W<GICD_IPRIORITYR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_IPRIORITYR1_SPEC>;
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
impl From<crate::W<GICD_IPRIORITYR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_IPRIORITYR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT4` reader - Interrupt 4"]
pub type INT4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT4` writer - Interrupt 4"]
pub type INT4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GICD_IPRIORITYR1_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT5` reader - Interrupt 5"]
pub type INT5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT5` writer - Interrupt 5"]
pub type INT5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GICD_IPRIORITYR1_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT6` reader - Interrupt 6"]
pub type INT6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT6` writer - Interrupt 6"]
pub type INT6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GICD_IPRIORITYR1_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT7` reader - Interrupt 7"]
pub type INT7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT7` writer - Interrupt 7"]
pub type INT7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GICD_IPRIORITYR1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 4"]
    #[inline(always)]
    pub fn int4(&self) -> INT4_R {
        INT4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 5"]
    #[inline(always)]
    pub fn int5(&self) -> INT5_R {
        INT5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 6"]
    #[inline(always)]
    pub fn int6(&self) -> INT6_R {
        INT6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 7"]
    #[inline(always)]
    pub fn int7(&self) -> INT7_R {
        INT7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 4"]
    #[inline(always)]
    #[must_use]
    pub fn int4(&mut self) -> INT4_W<0> {
        INT4_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 5"]
    #[inline(always)]
    #[must_use]
    pub fn int5(&mut self) -> INT5_W<8> {
        INT5_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 6"]
    #[inline(always)]
    #[must_use]
    pub fn int6(&mut self) -> INT6_W<16> {
        INT6_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 7"]
    #[inline(always)]
    #[must_use]
    pub fn int7(&mut self) -> INT7_W<24> {
        INT7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority 4 - 7 (Lower is first)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr1](index.html) module"]
pub struct GICD_IPRIORITYR1_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_ipriorityr1::R](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr1::W](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR1 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
