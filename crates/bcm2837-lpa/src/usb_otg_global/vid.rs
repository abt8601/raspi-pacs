#[doc = "Register `VID` reader"]
pub type R = crate::R<VID_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<VID_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "OTG_HS vendor ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vid::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VID_SPEC;
impl crate::RegisterSpec for VID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vid::R`](R) reader structure"]
impl crate::Readable for VID_SPEC {}
