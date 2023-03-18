#[doc = "Register `GICD_ITARGETSR23` reader"]
pub struct R(crate::R<GICD_ITARGETSR23_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ITARGETSR23_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ITARGETSR23_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ITARGETSR23_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ITARGETSR23` writer"]
pub struct W(crate::W<GICD_ITARGETSR23_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ITARGETSR23_SPEC>;
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
impl From<crate::W<GICD_ITARGETSR23_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ITARGETSR23_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT92` reader - Interrupt 92"]
pub type INT92_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT92` writer - Interrupt 92"]
pub type INT92_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR23_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT93` reader - Interrupt 93"]
pub type INT93_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT93` writer - Interrupt 93"]
pub type INT93_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR23_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT94` reader - Interrupt 94"]
pub type INT94_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT94` writer - Interrupt 94"]
pub type INT94_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR23_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT95` reader - Interrupt 95"]
pub type INT95_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT95` writer - Interrupt 95"]
pub type INT95_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR23_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 92"]
    #[inline(always)]
    pub fn int92(&self) -> INT92_R {
        INT92_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 93"]
    #[inline(always)]
    pub fn int93(&self) -> INT93_R {
        INT93_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 94"]
    #[inline(always)]
    pub fn int94(&self) -> INT94_R {
        INT94_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 95"]
    #[inline(always)]
    pub fn int95(&self) -> INT95_R {
        INT95_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 92"]
    #[inline(always)]
    #[must_use]
    pub fn int92(&mut self) -> INT92_W<0> {
        INT92_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 93"]
    #[inline(always)]
    #[must_use]
    pub fn int93(&mut self) -> INT93_W<8> {
        INT93_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 94"]
    #[inline(always)]
    #[must_use]
    pub fn int94(&mut self) -> INT94_W<16> {
        INT94_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 95"]
    #[inline(always)]
    #[must_use]
    pub fn int95(&mut self) -> INT95_W<24> {
        INT95_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Processor Target 92 - 95\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr23](index.html) module"]
pub struct GICD_ITARGETSR23_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR23_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_itargetsr23::R](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR23_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr23::W](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR23_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR23 to value 0"]
impl crate::Resettable for GICD_ITARGETSR23_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
