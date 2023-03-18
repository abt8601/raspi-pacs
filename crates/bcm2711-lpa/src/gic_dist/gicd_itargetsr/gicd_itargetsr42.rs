#[doc = "Register `GICD_ITARGETSR42` reader"]
pub struct R(crate::R<GICD_ITARGETSR42_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ITARGETSR42_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ITARGETSR42_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ITARGETSR42_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ITARGETSR42` writer"]
pub struct W(crate::W<GICD_ITARGETSR42_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ITARGETSR42_SPEC>;
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
impl From<crate::W<GICD_ITARGETSR42_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ITARGETSR42_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT168` reader - Interrupt 168"]
pub type INT168_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT168` writer - Interrupt 168"]
pub type INT168_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR42_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT169` reader - Interrupt 169"]
pub type INT169_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT169` writer - Interrupt 169"]
pub type INT169_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR42_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT170` reader - Interrupt 170"]
pub type INT170_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT170` writer - Interrupt 170"]
pub type INT170_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR42_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT171` reader - Interrupt 171"]
pub type INT171_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT171` writer - Interrupt 171"]
pub type INT171_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR42_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 168"]
    #[inline(always)]
    pub fn int168(&self) -> INT168_R {
        INT168_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 169"]
    #[inline(always)]
    pub fn int169(&self) -> INT169_R {
        INT169_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 170"]
    #[inline(always)]
    pub fn int170(&self) -> INT170_R {
        INT170_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 171"]
    #[inline(always)]
    pub fn int171(&self) -> INT171_R {
        INT171_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 168"]
    #[inline(always)]
    #[must_use]
    pub fn int168(&mut self) -> INT168_W<0> {
        INT168_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 169"]
    #[inline(always)]
    #[must_use]
    pub fn int169(&mut self) -> INT169_W<8> {
        INT169_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 170"]
    #[inline(always)]
    #[must_use]
    pub fn int170(&mut self) -> INT170_W<16> {
        INT170_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 171"]
    #[inline(always)]
    #[must_use]
    pub fn int171(&mut self) -> INT171_W<24> {
        INT171_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Processor Target 168 - 171\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr42](index.html) module"]
pub struct GICD_ITARGETSR42_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR42_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_itargetsr42::R](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR42_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr42::W](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR42_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR42 to value 0"]
impl crate::Resettable for GICD_ITARGETSR42_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
