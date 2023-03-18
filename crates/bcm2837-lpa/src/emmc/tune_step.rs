#[doc = "Register `TUNE_STEP` reader"]
pub struct R(crate::R<TUNE_STEP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TUNE_STEP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TUNE_STEP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TUNE_STEP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TUNE_STEP` writer"]
pub struct W(crate::W<TUNE_STEP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TUNE_STEP_SPEC>;
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
impl From<crate::W<TUNE_STEP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TUNE_STEP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DELAY` reader - "]
pub type DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DELAY` writer - "]
pub type DELAY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TUNE_STEP_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn delay(&mut self) -> DELAY_W<0> {
        DELAY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sample clock delay step duration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tune_step](index.html) module"]
pub struct TUNE_STEP_SPEC;
impl crate::RegisterSpec for TUNE_STEP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tune_step::R](R) reader structure"]
impl crate::Readable for TUNE_STEP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tune_step::W](W) writer structure"]
impl crate::Writable for TUNE_STEP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TUNE_STEP to value 0"]
impl crate::Resettable for TUNE_STEP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
