#[doc = "Register `GICD_ITARGETSR52` reader"]
pub struct R(crate::R<GICD_ITARGETSR52_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ITARGETSR52_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ITARGETSR52_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ITARGETSR52_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ITARGETSR52` writer"]
pub struct W(crate::W<GICD_ITARGETSR52_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ITARGETSR52_SPEC>;
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
impl From<crate::W<GICD_ITARGETSR52_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ITARGETSR52_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT208` reader - Interrupt 208"]
pub type INT208_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT208` writer - Interrupt 208"]
pub type INT208_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR52_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT209` reader - Interrupt 209"]
pub type INT209_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT209` writer - Interrupt 209"]
pub type INT209_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR52_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT210` reader - Interrupt 210"]
pub type INT210_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT210` writer - Interrupt 210"]
pub type INT210_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR52_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT211` reader - Interrupt 211"]
pub type INT211_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT211` writer - Interrupt 211"]
pub type INT211_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR52_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 208"]
    #[inline(always)]
    pub fn int208(&self) -> INT208_R {
        INT208_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 209"]
    #[inline(always)]
    pub fn int209(&self) -> INT209_R {
        INT209_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 210"]
    #[inline(always)]
    pub fn int210(&self) -> INT210_R {
        INT210_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 211"]
    #[inline(always)]
    pub fn int211(&self) -> INT211_R {
        INT211_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 208"]
    #[inline(always)]
    #[must_use]
    pub fn int208(&mut self) -> INT208_W<0> {
        INT208_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 209"]
    #[inline(always)]
    #[must_use]
    pub fn int209(&mut self) -> INT209_W<8> {
        INT209_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 210"]
    #[inline(always)]
    #[must_use]
    pub fn int210(&mut self) -> INT210_W<16> {
        INT210_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 211"]
    #[inline(always)]
    #[must_use]
    pub fn int211(&mut self) -> INT211_W<24> {
        INT211_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Processor Target 208 - 211\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr52](index.html) module"]
pub struct GICD_ITARGETSR52_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR52_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_itargetsr52::R](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR52_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr52::W](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR52_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR52 to value 0"]
impl crate::Resettable for GICD_ITARGETSR52_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
