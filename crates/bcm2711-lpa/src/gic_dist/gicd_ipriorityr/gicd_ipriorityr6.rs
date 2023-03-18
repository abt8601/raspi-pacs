#[doc = "Register `GICD_IPRIORITYR6` reader"]
pub struct R(crate::R<GICD_IPRIORITYR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_IPRIORITYR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_IPRIORITYR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_IPRIORITYR6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_IPRIORITYR6` writer"]
pub struct W(crate::W<GICD_IPRIORITYR6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_IPRIORITYR6_SPEC>;
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
impl From<crate::W<GICD_IPRIORITYR6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_IPRIORITYR6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT24` reader - Interrupt 24"]
pub type INT24_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT24` writer - Interrupt 24"]
pub type INT24_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR6_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT25` reader - Interrupt 25"]
pub type INT25_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT25` writer - Interrupt 25"]
pub type INT25_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR6_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT26` reader - Interrupt 26"]
pub type INT26_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT26` writer - Interrupt 26"]
pub type INT26_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR6_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT27` reader - Interrupt 27"]
pub type INT27_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT27` writer - Interrupt 27"]
pub type INT27_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR6_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 24"]
    #[inline(always)]
    pub fn int24(&self) -> INT24_R {
        INT24_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 25"]
    #[inline(always)]
    pub fn int25(&self) -> INT25_R {
        INT25_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 26"]
    #[inline(always)]
    pub fn int26(&self) -> INT26_R {
        INT26_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 27"]
    #[inline(always)]
    pub fn int27(&self) -> INT27_R {
        INT27_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 24"]
    #[inline(always)]
    #[must_use]
    pub fn int24(&mut self) -> INT24_W<0> {
        INT24_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 25"]
    #[inline(always)]
    #[must_use]
    pub fn int25(&mut self) -> INT25_W<8> {
        INT25_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 26"]
    #[inline(always)]
    #[must_use]
    pub fn int26(&mut self) -> INT26_W<16> {
        INT26_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 27"]
    #[inline(always)]
    #[must_use]
    pub fn int27(&mut self) -> INT27_W<24> {
        INT27_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority 24 - 27 (Lower is first)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr6](index.html) module"]
pub struct GICD_IPRIORITYR6_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_ipriorityr6::R](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr6::W](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR6 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
