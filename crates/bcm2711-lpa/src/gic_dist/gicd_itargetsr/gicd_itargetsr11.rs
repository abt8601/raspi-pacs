#[doc = "Register `GICD_ITARGETSR11` reader"]
pub struct R(crate::R<GICD_ITARGETSR11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ITARGETSR11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ITARGETSR11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ITARGETSR11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ITARGETSR11` writer"]
pub struct W(crate::W<GICD_ITARGETSR11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ITARGETSR11_SPEC>;
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
impl From<crate::W<GICD_ITARGETSR11_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ITARGETSR11_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT44` reader - Interrupt 44"]
pub type INT44_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT44` writer - Interrupt 44"]
pub type INT44_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR11_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT45` reader - Interrupt 45"]
pub type INT45_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT45` writer - Interrupt 45"]
pub type INT45_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR11_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT46` reader - Interrupt 46"]
pub type INT46_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT46` writer - Interrupt 46"]
pub type INT46_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR11_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT47` reader - Interrupt 47"]
pub type INT47_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT47` writer - Interrupt 47"]
pub type INT47_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR11_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 44"]
    #[inline(always)]
    pub fn int44(&self) -> INT44_R {
        INT44_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 45"]
    #[inline(always)]
    pub fn int45(&self) -> INT45_R {
        INT45_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 46"]
    #[inline(always)]
    pub fn int46(&self) -> INT46_R {
        INT46_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 47"]
    #[inline(always)]
    pub fn int47(&self) -> INT47_R {
        INT47_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 44"]
    #[inline(always)]
    #[must_use]
    pub fn int44(&mut self) -> INT44_W<0> {
        INT44_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 45"]
    #[inline(always)]
    #[must_use]
    pub fn int45(&mut self) -> INT45_W<8> {
        INT45_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 46"]
    #[inline(always)]
    #[must_use]
    pub fn int46(&mut self) -> INT46_W<16> {
        INT46_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 47"]
    #[inline(always)]
    #[must_use]
    pub fn int47(&mut self) -> INT47_W<24> {
        INT47_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Processor Target 44 - 47\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr11](index.html) module"]
pub struct GICD_ITARGETSR11_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_itargetsr11::R](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR11_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr11::W](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR11_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR11 to value 0"]
impl crate::Resettable for GICD_ITARGETSR11_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
