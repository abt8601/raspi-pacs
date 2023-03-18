#[doc = "Register `GICD_ITARGETSR10` reader"]
pub struct R(crate::R<GICD_ITARGETSR10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ITARGETSR10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ITARGETSR10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ITARGETSR10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ITARGETSR10` writer"]
pub struct W(crate::W<GICD_ITARGETSR10_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ITARGETSR10_SPEC>;
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
impl From<crate::W<GICD_ITARGETSR10_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ITARGETSR10_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT40` reader - Interrupt 40"]
pub type INT40_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT40` writer - Interrupt 40"]
pub type INT40_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR10_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT41` reader - Interrupt 41"]
pub type INT41_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT41` writer - Interrupt 41"]
pub type INT41_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR10_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT42` reader - Interrupt 42"]
pub type INT42_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT42` writer - Interrupt 42"]
pub type INT42_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR10_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT43` reader - Interrupt 43"]
pub type INT43_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT43` writer - Interrupt 43"]
pub type INT43_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR10_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 40"]
    #[inline(always)]
    pub fn int40(&self) -> INT40_R {
        INT40_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 41"]
    #[inline(always)]
    pub fn int41(&self) -> INT41_R {
        INT41_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 42"]
    #[inline(always)]
    pub fn int42(&self) -> INT42_R {
        INT42_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 43"]
    #[inline(always)]
    pub fn int43(&self) -> INT43_R {
        INT43_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 40"]
    #[inline(always)]
    #[must_use]
    pub fn int40(&mut self) -> INT40_W<0> {
        INT40_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 41"]
    #[inline(always)]
    #[must_use]
    pub fn int41(&mut self) -> INT41_W<8> {
        INT41_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 42"]
    #[inline(always)]
    #[must_use]
    pub fn int42(&mut self) -> INT42_W<16> {
        INT42_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 43"]
    #[inline(always)]
    #[must_use]
    pub fn int43(&mut self) -> INT43_W<24> {
        INT43_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Processor Target 40 - 43\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr10](index.html) module"]
pub struct GICD_ITARGETSR10_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_itargetsr10::R](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR10_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr10::W](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR10_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR10 to value 0"]
impl crate::Resettable for GICD_ITARGETSR10_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
