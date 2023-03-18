#[doc = "Register `TUNE_STEPS_STD` reader"]
pub struct R(crate::R<TUNE_STEPS_STD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TUNE_STEPS_STD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TUNE_STEPS_STD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TUNE_STEPS_STD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TUNE_STEPS_STD` writer"]
pub struct W(crate::W<TUNE_STEPS_STD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TUNE_STEPS_STD_SPEC>;
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
impl From<crate::W<TUNE_STEPS_STD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TUNE_STEPS_STD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STEPS` reader - "]
pub type STEPS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STEPS` writer - "]
pub type STEPS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TUNE_STEPS_STD_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn steps(&self) -> STEPS_R {
        STEPS_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn steps(&mut self) -> STEPS_W<0> {
        STEPS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sample clock delay step count for SDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tune_steps_std](index.html) module"]
pub struct TUNE_STEPS_STD_SPEC;
impl crate::RegisterSpec for TUNE_STEPS_STD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tune_steps_std::R](R) reader structure"]
impl crate::Readable for TUNE_STEPS_STD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tune_steps_std::W](W) writer structure"]
impl crate::Writable for TUNE_STEPS_STD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TUNE_STEPS_STD to value 0"]
impl crate::Resettable for TUNE_STEPS_STD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
