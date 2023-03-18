#[doc = "Register `GICC_RPR` reader"]
pub struct R(crate::R<GICC_RPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICC_RPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICC_RPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICC_RPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PRIORITY` reader - Current running priority"]
pub type PRIORITY_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Current running priority"]
    #[inline(always)]
    pub fn priority(&self) -> PRIORITY_R {
        PRIORITY_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Running Priority\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicc_rpr](index.html) module"]
pub struct GICC_RPR_SPEC;
impl crate::RegisterSpec for GICC_RPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicc_rpr::R](R) reader structure"]
impl crate::Readable for GICC_RPR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GICC_RPR to value 0"]
impl crate::Resettable for GICC_RPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
