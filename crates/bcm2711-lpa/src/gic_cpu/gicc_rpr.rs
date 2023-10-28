#[doc = "Register `GICC_RPR` reader"]
pub type R = crate::R<GICC_RPR_SPEC>;
#[doc = "Field `PRIORITY` reader - Current running priority"]
pub type PRIORITY_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Current running priority"]
    #[inline(always)]
    pub fn priority(&self) -> PRIORITY_R {
        PRIORITY_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICC_RPR")
            .field("priority", &format_args!("{}", self.priority().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICC_RPR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Running Priority\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicc_rpr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICC_RPR_SPEC;
impl crate::RegisterSpec for GICC_RPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicc_rpr::R`](R) reader structure"]
impl crate::Readable for GICC_RPR_SPEC {}
#[doc = "`reset()` method sets GICC_RPR to value 0"]
impl crate::Resettable for GICC_RPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
