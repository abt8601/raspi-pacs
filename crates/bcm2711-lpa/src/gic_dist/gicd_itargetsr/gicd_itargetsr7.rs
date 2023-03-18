#[doc = "Register `GICD_ITARGETSR7` reader"]
pub struct R(crate::R<GICD_ITARGETSR7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ITARGETSR7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ITARGETSR7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ITARGETSR7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ITARGETSR7` writer"]
pub struct W(crate::W<GICD_ITARGETSR7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ITARGETSR7_SPEC>;
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
impl From<crate::W<GICD_ITARGETSR7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ITARGETSR7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT28` reader - Interrupt 28"]
pub type INT28_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT28` writer - Interrupt 28"]
pub type INT28_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GICD_ITARGETSR7_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT29` reader - Interrupt 29"]
pub type INT29_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT29` writer - Interrupt 29"]
pub type INT29_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GICD_ITARGETSR7_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT30` reader - Interrupt 30"]
pub type INT30_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT30` writer - Interrupt 30"]
pub type INT30_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GICD_ITARGETSR7_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT31` reader - Interrupt 31"]
pub type INT31_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT31` writer - Interrupt 31"]
pub type INT31_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GICD_ITARGETSR7_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 28"]
    #[inline(always)]
    pub fn int28(&self) -> INT28_R {
        INT28_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 29"]
    #[inline(always)]
    pub fn int29(&self) -> INT29_R {
        INT29_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 30"]
    #[inline(always)]
    pub fn int30(&self) -> INT30_R {
        INT30_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 31"]
    #[inline(always)]
    pub fn int31(&self) -> INT31_R {
        INT31_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 28"]
    #[inline(always)]
    #[must_use]
    pub fn int28(&mut self) -> INT28_W<0> {
        INT28_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 29"]
    #[inline(always)]
    #[must_use]
    pub fn int29(&mut self) -> INT29_W<8> {
        INT29_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 30"]
    #[inline(always)]
    #[must_use]
    pub fn int30(&mut self) -> INT30_W<16> {
        INT30_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 31"]
    #[inline(always)]
    #[must_use]
    pub fn int31(&mut self) -> INT31_W<24> {
        INT31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Processor Target 28 - 31\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr7](index.html) module"]
pub struct GICD_ITARGETSR7_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_itargetsr7::R](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr7::W](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR7 to value 0"]
impl crate::Resettable for GICD_ITARGETSR7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
