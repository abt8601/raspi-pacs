#[doc = "Register `GICD_ITARGETSR45` reader"]
pub struct R(crate::R<GICD_ITARGETSR45_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ITARGETSR45_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ITARGETSR45_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ITARGETSR45_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ITARGETSR45` writer"]
pub struct W(crate::W<GICD_ITARGETSR45_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ITARGETSR45_SPEC>;
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
impl From<crate::W<GICD_ITARGETSR45_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ITARGETSR45_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT180` reader - Interrupt 180"]
pub type INT180_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT180` writer - Interrupt 180"]
pub type INT180_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR45_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT181` reader - Interrupt 181"]
pub type INT181_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT181` writer - Interrupt 181"]
pub type INT181_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR45_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT182` reader - Interrupt 182"]
pub type INT182_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT182` writer - Interrupt 182"]
pub type INT182_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR45_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT183` reader - Interrupt 183"]
pub type INT183_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT183` writer - Interrupt 183"]
pub type INT183_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR45_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 180"]
    #[inline(always)]
    pub fn int180(&self) -> INT180_R {
        INT180_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 181"]
    #[inline(always)]
    pub fn int181(&self) -> INT181_R {
        INT181_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 182"]
    #[inline(always)]
    pub fn int182(&self) -> INT182_R {
        INT182_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 183"]
    #[inline(always)]
    pub fn int183(&self) -> INT183_R {
        INT183_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 180"]
    #[inline(always)]
    #[must_use]
    pub fn int180(&mut self) -> INT180_W<0> {
        INT180_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 181"]
    #[inline(always)]
    #[must_use]
    pub fn int181(&mut self) -> INT181_W<8> {
        INT181_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 182"]
    #[inline(always)]
    #[must_use]
    pub fn int182(&mut self) -> INT182_W<16> {
        INT182_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 183"]
    #[inline(always)]
    #[must_use]
    pub fn int183(&mut self) -> INT183_W<24> {
        INT183_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Processor Target 180 - 183\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr45](index.html) module"]
pub struct GICD_ITARGETSR45_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR45_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_itargetsr45::R](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR45_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr45::W](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR45_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR45 to value 0"]
impl crate::Resettable for GICD_ITARGETSR45_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
