#[doc = "Register `GICD_ITARGETSR4` reader"]
pub struct R(crate::R<GICD_ITARGETSR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ITARGETSR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ITARGETSR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ITARGETSR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ITARGETSR4` writer"]
pub struct W(crate::W<GICD_ITARGETSR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ITARGETSR4_SPEC>;
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
impl From<crate::W<GICD_ITARGETSR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ITARGETSR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT16` reader - Interrupt 16"]
pub type INT16_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT16` writer - Interrupt 16"]
pub type INT16_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GICD_ITARGETSR4_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT17` reader - Interrupt 17"]
pub type INT17_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT17` writer - Interrupt 17"]
pub type INT17_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GICD_ITARGETSR4_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT18` reader - Interrupt 18"]
pub type INT18_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT18` writer - Interrupt 18"]
pub type INT18_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GICD_ITARGETSR4_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT19` reader - Interrupt 19"]
pub type INT19_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT19` writer - Interrupt 19"]
pub type INT19_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GICD_ITARGETSR4_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 16"]
    #[inline(always)]
    pub fn int16(&self) -> INT16_R {
        INT16_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 17"]
    #[inline(always)]
    pub fn int17(&self) -> INT17_R {
        INT17_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 18"]
    #[inline(always)]
    pub fn int18(&self) -> INT18_R {
        INT18_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 19"]
    #[inline(always)]
    pub fn int19(&self) -> INT19_R {
        INT19_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 16"]
    #[inline(always)]
    #[must_use]
    pub fn int16(&mut self) -> INT16_W<0> {
        INT16_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 17"]
    #[inline(always)]
    #[must_use]
    pub fn int17(&mut self) -> INT17_W<8> {
        INT17_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 18"]
    #[inline(always)]
    #[must_use]
    pub fn int18(&mut self) -> INT18_W<16> {
        INT18_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 19"]
    #[inline(always)]
    #[must_use]
    pub fn int19(&mut self) -> INT19_W<24> {
        INT19_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Processor Target 16 - 19\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr4](index.html) module"]
pub struct GICD_ITARGETSR4_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_itargetsr4::R](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr4::W](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR4 to value 0"]
impl crate::Resettable for GICD_ITARGETSR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
