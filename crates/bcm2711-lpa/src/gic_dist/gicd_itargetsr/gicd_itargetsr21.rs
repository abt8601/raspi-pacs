#[doc = "Register `GICD_ITARGETSR21` reader"]
pub struct R(crate::R<GICD_ITARGETSR21_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ITARGETSR21_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ITARGETSR21_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ITARGETSR21_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ITARGETSR21` writer"]
pub struct W(crate::W<GICD_ITARGETSR21_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ITARGETSR21_SPEC>;
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
impl From<crate::W<GICD_ITARGETSR21_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ITARGETSR21_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT84` reader - Interrupt 84"]
pub type INT84_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT84` writer - Interrupt 84"]
pub type INT84_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR21_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT85` reader - Interrupt 85"]
pub type INT85_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT85` writer - Interrupt 85"]
pub type INT85_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR21_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT86` reader - Interrupt 86"]
pub type INT86_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT86` writer - Interrupt 86"]
pub type INT86_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR21_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT87` reader - Interrupt 87"]
pub type INT87_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT87` writer - Interrupt 87"]
pub type INT87_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR21_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 84"]
    #[inline(always)]
    pub fn int84(&self) -> INT84_R {
        INT84_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 85"]
    #[inline(always)]
    pub fn int85(&self) -> INT85_R {
        INT85_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 86"]
    #[inline(always)]
    pub fn int86(&self) -> INT86_R {
        INT86_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 87"]
    #[inline(always)]
    pub fn int87(&self) -> INT87_R {
        INT87_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 84"]
    #[inline(always)]
    #[must_use]
    pub fn int84(&mut self) -> INT84_W<0> {
        INT84_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 85"]
    #[inline(always)]
    #[must_use]
    pub fn int85(&mut self) -> INT85_W<8> {
        INT85_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 86"]
    #[inline(always)]
    #[must_use]
    pub fn int86(&mut self) -> INT86_W<16> {
        INT86_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 87"]
    #[inline(always)]
    #[must_use]
    pub fn int87(&mut self) -> INT87_W<24> {
        INT87_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Processor Target 84 - 87\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr21](index.html) module"]
pub struct GICD_ITARGETSR21_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR21_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_itargetsr21::R](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR21_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr21::W](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR21_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR21 to value 0"]
impl crate::Resettable for GICD_ITARGETSR21_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
