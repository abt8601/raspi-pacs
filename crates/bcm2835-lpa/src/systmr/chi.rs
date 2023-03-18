#[doc = "Register `CHI` reader"]
pub struct R(crate::R<CHI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Higher 32 bits for the free running counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chi](index.html) module"]
pub struct CHI_SPEC;
impl crate::RegisterSpec for CHI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chi::R](R) reader structure"]
impl crate::Readable for CHI_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CHI to value 0"]
impl crate::Resettable for CHI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
