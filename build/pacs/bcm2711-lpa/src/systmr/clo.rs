#[doc = "Register `CLO` reader"]
pub type R = crate::R<CLO_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<CLO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Lower 32 bits for the free running counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clo::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLO_SPEC;
impl crate::RegisterSpec for CLO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clo::R`](R) reader structure"]
impl crate::Readable for CLO_SPEC {}
#[doc = "`reset()` method sets CLO to value 0"]
impl crate::Resettable for CLO_SPEC {
    const RESET_VALUE: u32 = 0;
}
