#[doc = "Register `GICD_ITARGETSR49` reader"]
pub struct R(crate::R<GICD_ITARGETSR49_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ITARGETSR49_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ITARGETSR49_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ITARGETSR49_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ITARGETSR49` writer"]
pub struct W(crate::W<GICD_ITARGETSR49_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ITARGETSR49_SPEC>;
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
impl From<crate::W<GICD_ITARGETSR49_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ITARGETSR49_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT196` reader - Interrupt 196"]
pub type INT196_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT196` writer - Interrupt 196"]
pub type INT196_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR49_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT197` reader - Interrupt 197"]
pub type INT197_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT197` writer - Interrupt 197"]
pub type INT197_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR49_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT198` reader - Interrupt 198"]
pub type INT198_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT198` writer - Interrupt 198"]
pub type INT198_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR49_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT199` reader - Interrupt 199"]
pub type INT199_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT199` writer - Interrupt 199"]
pub type INT199_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR49_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 196"]
    #[inline(always)]
    pub fn int196(&self) -> INT196_R {
        INT196_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 197"]
    #[inline(always)]
    pub fn int197(&self) -> INT197_R {
        INT197_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 198"]
    #[inline(always)]
    pub fn int198(&self) -> INT198_R {
        INT198_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 199"]
    #[inline(always)]
    pub fn int199(&self) -> INT199_R {
        INT199_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 196"]
    #[inline(always)]
    #[must_use]
    pub fn int196(&mut self) -> INT196_W<0> {
        INT196_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 197"]
    #[inline(always)]
    #[must_use]
    pub fn int197(&mut self) -> INT197_W<8> {
        INT197_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 198"]
    #[inline(always)]
    #[must_use]
    pub fn int198(&mut self) -> INT198_W<16> {
        INT198_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 199"]
    #[inline(always)]
    #[must_use]
    pub fn int199(&mut self) -> INT199_W<24> {
        INT199_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Processor Target 196 - 199\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr49](index.html) module"]
pub struct GICD_ITARGETSR49_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR49_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_itargetsr49::R](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR49_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr49::W](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR49_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR49 to value 0"]
impl crate::Resettable for GICD_ITARGETSR49_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
