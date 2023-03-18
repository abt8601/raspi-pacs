#[doc = "Register `PEEK0` reader"]
pub struct R(crate::R<PEEK0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PEEK0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PEEK0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PEEK0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PEEK0` writer"]
pub struct W(crate::W<PEEK0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PEEK0_SPEC>;
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
impl From<crate::W<PEEK0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PEEK0_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peek0](index.html) module"]
pub struct PEEK0_SPEC;
impl crate::RegisterSpec for PEEK0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [peek0::R](R) reader structure"]
impl crate::Readable for PEEK0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [peek0::W](W) writer structure"]
impl crate::Writable for PEEK0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
