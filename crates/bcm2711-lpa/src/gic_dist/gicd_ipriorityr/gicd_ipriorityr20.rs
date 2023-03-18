#[doc = "Register `GICD_IPRIORITYR20` reader"]
pub struct R(crate::R<GICD_IPRIORITYR20_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_IPRIORITYR20_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_IPRIORITYR20_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_IPRIORITYR20_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_IPRIORITYR20` writer"]
pub struct W(crate::W<GICD_IPRIORITYR20_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_IPRIORITYR20_SPEC>;
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
impl From<crate::W<GICD_IPRIORITYR20_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_IPRIORITYR20_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT80` reader - Interrupt 80"]
pub type INT80_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT80` writer - Interrupt 80"]
pub type INT80_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR20_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT81` reader - Interrupt 81"]
pub type INT81_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT81` writer - Interrupt 81"]
pub type INT81_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR20_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT82` reader - Interrupt 82"]
pub type INT82_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT82` writer - Interrupt 82"]
pub type INT82_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR20_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT83` reader - Interrupt 83"]
pub type INT83_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT83` writer - Interrupt 83"]
pub type INT83_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR20_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 80"]
    #[inline(always)]
    pub fn int80(&self) -> INT80_R {
        INT80_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 81"]
    #[inline(always)]
    pub fn int81(&self) -> INT81_R {
        INT81_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 82"]
    #[inline(always)]
    pub fn int82(&self) -> INT82_R {
        INT82_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 83"]
    #[inline(always)]
    pub fn int83(&self) -> INT83_R {
        INT83_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 80"]
    #[inline(always)]
    #[must_use]
    pub fn int80(&mut self) -> INT80_W<0> {
        INT80_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 81"]
    #[inline(always)]
    #[must_use]
    pub fn int81(&mut self) -> INT81_W<8> {
        INT81_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 82"]
    #[inline(always)]
    #[must_use]
    pub fn int82(&mut self) -> INT82_W<16> {
        INT82_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 83"]
    #[inline(always)]
    #[must_use]
    pub fn int83(&mut self) -> INT83_W<24> {
        INT83_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority 80 - 83 (Lower is first)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr20](index.html) module"]
pub struct GICD_IPRIORITYR20_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR20_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_ipriorityr20::R](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR20_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr20::W](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR20_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR20 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR20_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
