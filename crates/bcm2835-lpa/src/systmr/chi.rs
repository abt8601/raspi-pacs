#[doc = "Register `CHI` reader"]
pub type R = crate::R<CHI_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<CHI_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Higher 32 bits for the free running counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chi::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHI_SPEC;
impl crate::RegisterSpec for CHI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chi::R`](R) reader structure"]
impl crate::Readable for CHI_SPEC {}
#[doc = "`reset()` method sets CHI to value 0"]
impl crate::Resettable for CHI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
