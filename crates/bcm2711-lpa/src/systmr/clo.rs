#[doc = "Register `CLO` reader"]
pub struct R(crate::R<CLO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Lower 32 bits for the free running counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clo](index.html) module"]
pub struct CLO_SPEC;
impl crate::RegisterSpec for CLO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clo::R](R) reader structure"]
impl crate::Readable for CLO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CLO to value 0"]
impl crate::Resettable for CLO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
